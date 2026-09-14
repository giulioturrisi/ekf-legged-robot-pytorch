"""Batched PyTorch contact-aided error-state EKF for quadrupeds.

The filter state is stored as tensors with leading dimension ``num_envs``:

    p_WB, v_WB, R_WB, b_g, b_a

and a fixed 15-dimensional error covariance ordered as:

    dp, dv, dtheta, dbg, dba

Contact feet are maintained as world-frame anchors with shape
``(num_envs, num_legs, 3)``.  This module intentionally uses PyTorch only so
the same estimator can run in a Python ROS2 node or directly inside IsaacLab.
"""

from __future__ import annotations

from typing import Any, Dict, Mapping, Optional, Sequence, Tuple
import time

import torch

import config
from mujoco_warp_kinematics import build_mujoco_forward_kinematics_from_env


TensorLike = Any


def skew(v: TensorLike) -> torch.Tensor:
    """Return the skew matrix such that ``skew(v) @ w == v x w``."""
    v = torch.as_tensor(v)
    if v.shape[-1] != 3:
        raise ValueError(f"skew expects last dimension 3, got {tuple(v.shape)}")
    z = torch.zeros_like(v[..., 0])
    x, y, z_axis = v[..., 0], v[..., 1], v[..., 2]
    return torch.stack(
        (
            torch.stack((z, -z_axis, y), dim=-1),
            torch.stack((z_axis, z, -x), dim=-1),
            torch.stack((-y, x, z), dim=-1),
        ),
        dim=-2,
    )


def so3_exp(phi: TensorLike) -> torch.Tensor:
    """Batched SO(3) exponential map."""
    phi = torch.as_tensor(phi)
    if phi.shape[-1] != 3:
        raise ValueError(f"so3_exp expects last dimension 3, got {tuple(phi.shape)}")

    K = skew(phi)
    K2 = K @ K
    theta2 = (phi * phi).sum(dim=-1, keepdim=True)
    theta = torch.sqrt(torch.clamp(theta2, min=0.0))
    small = theta < 1e-6

    theta4 = theta2 * theta2
    A_small = 1.0 - theta2 / 6.0 + theta4 / 120.0
    B_small = 0.5 - theta2 / 24.0 + theta4 / 720.0
    A = torch.where(small, A_small, torch.sin(theta) / theta.clamp_min(1e-12))
    B = torch.where(small, B_small, (1.0 - torch.cos(theta)) / theta2.clamp_min(1e-12))

    eye = torch.eye(3, device=phi.device, dtype=phi.dtype).expand(phi.shape[:-1] + (3, 3))
    return eye + A[..., None] * K + B[..., None] * K2


def project_so3(R: TensorLike) -> torch.Tensor:
    """Project near-rotation matrices back to SO(3)."""
    R = torch.as_tensor(R)
    U, _, Vh = torch.linalg.svd(R)
    R_proj = U @ Vh
    det = torch.linalg.det(R_proj)
    if bool((det < 0.0).any()):
        U = U.clone()
        U[..., :, -1] = torch.where((det < 0.0)[..., None], -U[..., :, -1], U[..., :, -1])
        R_proj = U @ Vh
    return R_proj


def quat_wxyz_to_rot(q: TensorLike) -> torch.Tensor:
    """Convert ``(..., 4)`` quaternions in ``[w, x, y, z]`` order to rotations."""
    q = torch.as_tensor(q)
    if q.shape[-1] != 4:
        raise ValueError(f"quat_wxyz_to_rot expects last dimension 4, got {tuple(q.shape)}")

    norm = torch.linalg.norm(q, dim=-1, keepdim=True)
    q = torch.where(norm > 1e-12, q / norm.clamp_min(1e-12), torch.zeros_like(q))
    w, x, y, z = q.unbind(dim=-1)
    one = torch.ones_like(w)
    two = 2.0

    R = torch.stack(
        (
            torch.stack((one - two * (y * y + z * z), two * (x * y - z * w), two * (x * z + y * w)), dim=-1),
            torch.stack((two * (x * y + z * w), one - two * (x * x + z * z), two * (y * z - x * w)), dim=-1),
            torch.stack((two * (x * z - y * w), two * (y * z + x * w), one - two * (x * x + y * y)), dim=-1),
        ),
        dim=-2,
    )

    eye = torch.eye(3, device=q.device, dtype=q.dtype).expand(q.shape[:-1] + (3, 3))
    return torch.where((norm[..., None] > 1e-12), R, eye)


def align_quat_wxyz(q: TensorLike, reference_wxyz: TensorLike) -> torch.Tensor:
    """Flip quaternion signs to stay in the same hemisphere as ``reference_wxyz``."""
    q = torch.as_tensor(q)
    if q.shape[-1] != 4:
        raise ValueError(f"align_quat_wxyz expects q last dimension 4, got {tuple(q.shape)}")
    reference = torch.as_tensor(reference_wxyz, device=q.device, dtype=q.dtype)
    if reference.shape[-1] != 4:
        raise ValueError(
            f"align_quat_wxyz expects reference last dimension 4, got {tuple(reference.shape)}"
        )

    norm = torch.linalg.norm(reference, dim=-1, keepdim=True)
    reference = torch.where(norm > 1e-12, reference / norm.clamp_min(1e-12), reference)
    return torch.where((q * reference).sum(dim=-1, keepdim=True) < 0.0, -q, q)


def rot_to_quat_wxyz(R: TensorLike, reference_wxyz: Optional[TensorLike] = None) -> torch.Tensor:
    """Convert rotation matrices to normalized ``[w, x, y, z]`` quaternions.

    If ``reference_wxyz`` is provided, the returned quaternion sign is chosen
    to be closest to the reference. This avoids discontinuous sign flips when
    publishing a sequence of orientations from rotation matrices.
    """
    R = project_so3(torch.as_tensor(R))
    m00 = R[..., 0, 0]
    m01 = R[..., 0, 1]
    m02 = R[..., 0, 2]
    m10 = R[..., 1, 0]
    m11 = R[..., 1, 1]
    m12 = R[..., 1, 2]
    m20 = R[..., 2, 0]
    m21 = R[..., 2, 1]
    m22 = R[..., 2, 2]

    q_abs = torch.sqrt(
        torch.clamp(
            torch.stack(
                (
                    1.0 + m00 + m11 + m22,
                    1.0 + m00 - m11 - m22,
                    1.0 - m00 + m11 - m22,
                    1.0 - m00 - m11 + m22,
                ),
                dim=-1,
            ),
            min=0.0,
        )
    )

    quat_by_w = torch.stack((q_abs[..., 0] ** 2, m21 - m12, m02 - m20, m10 - m01), dim=-1)
    quat_by_x = torch.stack((m21 - m12, q_abs[..., 1] ** 2, m01 + m10, m02 + m20), dim=-1)
    quat_by_y = torch.stack((m02 - m20, m01 + m10, q_abs[..., 2] ** 2, m12 + m21), dim=-1)
    quat_by_z = torch.stack((m10 - m01, m02 + m20, m12 + m21, q_abs[..., 3] ** 2), dim=-1)

    quat_candidates = torch.stack((quat_by_w, quat_by_x, quat_by_y, quat_by_z), dim=-2)
    denom = (2.0 * q_abs).clamp_min(1e-12)[..., None]
    quat_candidates = quat_candidates / denom
    best = q_abs.argmax(dim=-1)
    q = quat_candidates.gather(
        -2,
        best[..., None, None].expand(best.shape + (1, 4)),
    ).squeeze(-2)
    q = q / torch.linalg.norm(q, dim=-1, keepdim=True).clamp_min(1e-12)
    if reference_wxyz is not None:
        q = align_quat_wxyz(q, reference_wxyz)
    return q


class EKFNoise:
    # These values are standard deviations in the measurement and process
    # models; squared values are used when building covariance matrices.
    def __init__(
        self,
        gyro: Optional[float] = None,
        accel: Optional[float] = None,
        gyro_bias: Optional[float] = None,
        accel_bias: Optional[float] = None,
        contact_position: Optional[float] = None,
        contact_velocity: Optional[float] = None,
        joint_position: Optional[float] = None,
    ) -> None:
        values = config.noise
        self.gyro = values["gyro"] if gyro is None else float(gyro)
        self.accel = values["accel"] if accel is None else float(accel)
        self.gyro_bias = values["gyro_bias"] if gyro_bias is None else float(gyro_bias)
        self.accel_bias = values["accel_bias"] if accel_bias is None else float(accel_bias)
        self.contact_position = (
            values["contact_position"] if contact_position is None else float(contact_position)
        )
        self.contact_velocity = (
            values["contact_velocity"] if contact_velocity is None else float(contact_velocity)
        )
        self.joint_position = (
            values["joint_position"] if joint_position is None else float(joint_position)
        )


class ContactMeasurement:
    def __init__(self, leg: str, position_base: TensorLike, jacobian: Optional[TensorLike] = None) -> None:
        self.leg = leg
        self.position_base = position_base
        self.jacobian = jacobian


TorchContactMeasurement = ContactMeasurement


class ContactAidedEKF:
    # The filter uses a 15-state error model: position, velocity, attitude,
    # gyro bias, and accelerometer bias.
    def __init__(
        self,
        num_envs: int = 1,
        leg_order: Optional[Sequence[str]] = None,
        gravity: Optional[TensorLike] = None,
        noise: Optional[EKFNoise] = None,
        max_dt: Optional[float] = None,
        use_contact_velocity: Optional[bool] = None,
        kinematics: Optional[Any] = None,
        device: TensorLike = "cpu",
        dtype: torch.dtype = torch.float32,
        configure_kinematics: bool = True,
    ) -> None:
        self.num_envs = int(num_envs)
        if self.num_envs <= 0:
            raise ValueError("num_envs must be positive")
        self.leg_order = tuple(config.leg_order if leg_order is None else leg_order)
        self.device = torch.device(device)
        self.dtype = dtype
        self.noise = EKFNoise() if noise is None else noise
        self.max_dt = config.max_dt if max_dt is None else float(max_dt)
        self.use_contact_velocity = (
            config.use_contact_velocity if use_contact_velocity is None else bool(use_contact_velocity)
        )
        self.compute_kinematic_jacobians = self.use_contact_velocity
        self.kinematics = kinematics

        if gravity is None:
            gravity = list(config.gravity)
        self.gravity = self._tensor(gravity).reshape(3)

        # Nominal state tensors are batched so the same implementation works
        # for one ROS2 robot or many parallel simulation environments.
        self.p = torch.zeros((self.num_envs, 3), device=self.device, dtype=self.dtype)
        self.v = torch.zeros_like(self.p)
        self.R = self._eye(3).expand(self.num_envs, 3, 3).clone()
        self.bg = torch.zeros_like(self.p)
        self.ba = torch.zeros_like(self.p)
        self.P = self._initial_covariance().expand(self.num_envs, 15, 15).clone()
        self.contact_anchors = torch.zeros(
            (self.num_envs, len(self.leg_order), 3),
            device=self.device,
            dtype=self.dtype,
        )
        self.contact_active = torch.zeros(
            (self.num_envs, len(self.leg_order)),
            device=self.device,
            dtype=torch.bool,
        )
        self._last_quat_wxyz = torch.zeros((self.num_envs, 4), device=self.device, dtype=self.dtype)
        self._last_quat_wxyz[:, 0] = 1.0
        self.initialized = torch.zeros((self.num_envs,), device=self.device, dtype=torch.bool)

        if self.kinematics is None and configure_kinematics:
            self.configure_kinematics()

    def configure_kinematics(self, prefix: str = "EKF") -> Any:
        self.kinematics = build_mujoco_forward_kinematics_from_env(
            prefix=prefix,
            leg_order=self.leg_order,
            num_envs=self.num_envs,
        )
        return self.kinematics

    def tuning_parameters(self) -> Dict[str, Any]:
        """Return the active filter settings for startup diagnostics."""
        return {
            "gyro_noise": self.noise.gyro,
            "accel_noise": self.noise.accel,
            "gyro_bias_noise": self.noise.gyro_bias,
            "accel_bias_noise": self.noise.accel_bias,
            "contact_position_noise": self.noise.contact_position,
            "contact_velocity_noise": self.noise.contact_velocity,
            "joint_position_noise": self.noise.joint_position,
            "max_dt": self.max_dt,
            "gravity": tuple(float(value) for value in self.gravity.detach().cpu()),
            "use_contact_velocity": self.use_contact_velocity,
            "compute_kinematic_jacobians": self.compute_kinematic_jacobians,
        }

    def contact_measurements(
        self,
        joint_positions: TensorLike,
        with_jacobians: Optional[bool] = None,
    ) -> Any:
        if self.kinematics is None:
            raise RuntimeError("MuJoCo Warp kinematics is not configured")
        compute_jacobians = self.compute_kinematic_jacobians if with_jacobians is None else with_jacobians
        if compute_jacobians:
            return self.kinematics.contact_measurements_with_jacobians(joint_positions)
        return self.kinematics.contact_measurements(joint_positions)

    @property
    def active_contacts(self) -> torch.Tensor:
        return self.contact_active

    def reset(
        self,
        position: TensorLike,
        orientation_wxyz: TensorLike,
        linear_velocity: Optional[TensorLike] = None,
        env_ids: Optional[TensorLike] = None,
    ) -> None:
        # Reset both the nominal state and uncertainty for the selected
        # environments. Contact anchors are rebuilt from the next measurement.
        ids = self._env_ids(env_ids)
        n = int(ids.numel())
        self.p[ids] = self._batch_tensor(position, 3, n)
        orientation = self._batch_tensor(orientation_wxyz, 4, n)
        self.v[ids] = (
            torch.zeros((n, 3), device=self.device, dtype=self.dtype)
            if linear_velocity is None
            else self._batch_tensor(linear_velocity, 3, n)
        )
        self.R[ids] = quat_wxyz_to_rot(orientation)
        self._last_quat_wxyz[ids] = rot_to_quat_wxyz(self.R[ids], reference_wxyz=orientation)
        self.bg[ids] = 0.0
        self.ba[ids] = 0.0
        self.P[ids] = self._initial_covariance().expand(n, 15, 15).clone()
        self.contact_anchors[ids] = 0.0
        self.contact_active[ids] = False
        self.initialized[ids] = True

    def propagate(self, gyro: TensorLike, accel: TensorLike, dt: TensorLike) -> None:
        # Integrate the IMU in the world frame. The accelerometer is assumed to
        # measure specific force, so gravity is added after rotating it.
        mask = self.initialized
        if not bool(mask.any()):
            return

        gyro = self._batch_tensor(gyro, 3, self.num_envs)
        accel = self._batch_tensor(accel, 3, self.num_envs)
        dt = self._dt_tensor(dt)

        omega = gyro - self.bg
        acc = accel - self.ba
        R0 = self.R
        v0 = self.v
        a_world = torch.einsum("nij,nj->ni", R0, acc) + self.gravity

        p_new = self.p + v0 * dt[:, None] + 0.5 * a_world * dt[:, None] * dt[:, None]
        v_new = self.v + a_world * dt[:, None]
        R_new = project_so3(R0 @ so3_exp(omega * dt[:, None]))

        # Linearized error dynamics used to propagate the covariance alongside
        # the nominal position, velocity, and attitude.
        F = self._eye(15).expand(self.num_envs, 15, 15).clone()
        eye3 = self._eye(3)
        F[:, 0:3, 3:6] = eye3 * dt[:, None, None]
        F[:, 3:6, 6:9] = -(R0 @ skew(acc)) * dt[:, None, None]
        F[:, 3:6, 12:15] = -R0 * dt[:, None, None]
        F[:, 6:9, 6:9] = eye3 - skew(omega) * dt[:, None, None]
        F[:, 6:9, 9:12] = -eye3 * dt[:, None, None]

        G = torch.zeros((self.num_envs, 15, 12), device=self.device, dtype=self.dtype)
        G[:, 6:9, 0:3] = -eye3
        G[:, 3:6, 3:6] = -R0
        G[:, 9:12, 6:9] = eye3
        G[:, 12:15, 9:12] = eye3

        q_diag = self._tensor(
            [self.noise.gyro**2] * 3
            + [self.noise.accel**2] * 3
            + [self.noise.gyro_bias**2] * 3
            + [self.noise.accel_bias**2] * 3
        )
        Qc = torch.diag_embed(q_diag.expand(self.num_envs, 12))
        P_new = F @ self.P @ F.transpose(-1, -2) + (G @ Qc @ G.transpose(-1, -2)) * dt[:, None, None]
        P_new = self._symmetrize(P_new)

        self.p = torch.where(mask[:, None], p_new, self.p)
        self.v = torch.where(mask[:, None], v_new, self.v)
        self.R = torch.where(mask[:, None, None], R_new, self.R)
        self.P = torch.where(mask[:, None, None], P_new, self.P)

    def set_contacts(self, contacts: TensorLike, measurements: Any) -> None:
        # A newly detected stance foot becomes a fixed world-frame anchor;
        # losing contact releases that anchor on the next update.
        desired = self._contacts_tensor(contacts)
        foot_positions, _, valid = self._measurement_tensors(measurements)
        desired = desired & self.initialized[:, None]
        valid_desired = desired & valid

        touchdown = valid_desired & ~self.contact_active
        anchors = self.p[:, None, :] + torch.einsum("nij,nlj->nli", self.R, foot_positions)
        self.contact_anchors = torch.where(touchdown[..., None], anchors, self.contact_anchors)
        self.contact_active = (self.contact_active & desired) | touchdown

    def correct_kinematics(
        self,
        measurements: Any,
        gyro: Optional[TensorLike] = None,
        joint_velocities: Optional[TensorLike] = None,
        foot_jacobians: Optional[TensorLike] = None,
    ) -> None:
        # Contact position and optional zero-foot-velocity observations are
        # applied one leg at a time because each leg has its own validity mask.
        if not bool(self.initialized.any()):
            return

        foot_positions, jacobians, valid = self._measurement_tensors(measurements, foot_jacobians)
        for leg_idx in range(len(self.leg_order)):
            self._correct_contact_position(leg_idx, foot_positions[:, leg_idx], None if jacobians is None else jacobians[:, leg_idx], valid[:, leg_idx])
            if self.use_contact_velocity:
                self._correct_contact_velocity(
                    leg_idx,
                    foot_positions[:, leg_idx],
                    None if jacobians is None else jacobians[:, leg_idx],
                    valid[:, leg_idx],
                    gyro,
                    joint_velocities,
                )

    def step(
        self,
        gyro: TensorLike,
        accel: TensorLike,
        joint_measurements: Any = None,
        contacts: Optional[TensorLike] = None,
        dt: Optional[TensorLike] = None,
        joint_velocities: Optional[TensorLike] = None,
        foot_jacobians: Optional[TensorLike] = None,
        joint_positions: Optional[TensorLike] = None,
    ) -> None:
        # Keep the update order explicit: predict first, register contacts,
        # then apply kinematic corrections using the current joint sample.
        if contacts is None or dt is None:
            raise ValueError("contacts and dt are required")
        if joint_measurements is None:
            if joint_positions is None:
                raise ValueError("joint_positions are required when joint_measurements are not provided")
            joint_measurements = self.contact_measurements(joint_positions)
        self.propagate(gyro, accel, dt)
        self.set_contacts(contacts, joint_measurements)
        self.correct_kinematics(
            joint_measurements,
            gyro=gyro,
            joint_velocities=joint_velocities,
            foot_jacobians=foot_jacobians,
        )

    def state_dict(self, quat_reference_wxyz: Optional[TensorLike] = None) -> Dict[str, torch.Tensor]:
        reference = self._last_quat_wxyz if quat_reference_wxyz is None else quat_reference_wxyz
        quat_wxyz = rot_to_quat_wxyz(self.R, reference_wxyz=reference)
        self._last_quat_wxyz = quat_wxyz.detach().clone()
        return {
            "position": self.p.clone(),
            "linear_velocity": self.v.clone(),
            "R": self.R.clone(),
            "quat_wxyz": quat_wxyz,
            "gyro_bias": self.bg.clone(),
            "accel_bias": self.ba.clone(),
            "contact_anchors": self.contact_anchors.clone(),
            "contact_active": self.contact_active.clone(),
            "covariance": self.P.clone(),
            "initialized": self.initialized.clone(),
        }

    def to(self, device: Optional[TensorLike] = None, dtype: Optional[torch.dtype] = None) -> "ContactAidedEKF":
        if device is not None:
            self.device = torch.device(device)
        if dtype is not None:
            self.dtype = dtype
        for name in ("gravity", "p", "v", "R", "bg", "ba", "P", "contact_anchors", "_last_quat_wxyz"):
            setattr(self, name, getattr(self, name).to(device=self.device, dtype=self.dtype))
        self.contact_active = self.contact_active.to(device=self.device)
        self.initialized = self.initialized.to(device=self.device)
        return self

    def _correct_contact_position(
        self,
        leg_idx: int,
        foot_base: torch.Tensor,
        jacobian: Optional[torch.Tensor],
        valid: torch.Tensor,
    ) -> None:
        mask = self.contact_active[:, leg_idx] & valid & self.initialized
        if not bool(mask.any()):
            return

        predicted = self.p + torch.einsum("nij,nj->ni", self.R, foot_base)
        residual = self.contact_anchors[:, leg_idx] - predicted

        H = torch.zeros((self.num_envs, 3, 15), device=self.device, dtype=self.dtype)
        H[:, :, 0:3] = self._eye(3)
        H[:, :, 6:9] = -(self.R @ skew(foot_base))
        N = self._contact_position_covariance(jacobian)
        self._update(residual, H, N, mask)

    def _correct_contact_velocity(
        self,
        leg_idx: int,
        foot_base: torch.Tensor,
        jacobian: Optional[torch.Tensor],
        valid: torch.Tensor,
        gyro: Optional[TensorLike],
        joint_velocities: Optional[TensorLike],
    ) -> None:
        # For a stance foot, the measured foot velocity should be zero in the
        # world frame. The Jacobian maps joint rates into base-frame foot rate.
        if gyro is None or joint_velocities is None or jacobian is None:
            return

        foot_velocity_base, valid_velocity = self._foot_velocity_base(jacobian, joint_velocities, leg_idx)
        mask = self.contact_active[:, leg_idx] & valid & valid_velocity & self.initialized
        if not bool(mask.any()):
            return

        gyro = self._batch_tensor(gyro, 3, self.num_envs)
        omega = gyro - self.bg
        local_velocity = torch.cross(omega, foot_base, dim=-1) + foot_velocity_base
        predicted = self.v + torch.einsum("nij,nj->ni", self.R, local_velocity)
        residual = -predicted

        H = torch.zeros((self.num_envs, 3, 15), device=self.device, dtype=self.dtype)
        H[:, :, 3:6] = self._eye(3)
        H[:, :, 6:9] = -(self.R @ skew(local_velocity))
        H[:, :, 9:12] = self.R @ skew(foot_base)
        N = self._eye(3).expand(self.num_envs, 3, 3).clone() * (self.noise.contact_velocity**2)
        self._update(residual, H, N, mask)

    def _contact_position_covariance(self, jacobian: Optional[torch.Tensor]) -> torch.Tensor:
        N = self._eye(3).expand(self.num_envs, 3, 3).clone() * (self.noise.contact_position**2)
        if jacobian is None:
            return N
        joint_cov = self._eye(jacobian.shape[-1]).expand(self.num_envs, jacobian.shape[-1], jacobian.shape[-1])
        joint_cov = joint_cov * (self.noise.joint_position**2)
        return N + self.R @ jacobian @ joint_cov @ jacobian.transpose(-1, -2) @ self.R.transpose(-1, -2)

    def _update(self, residual: torch.Tensor, H: torch.Tensor, N: torch.Tensor, mask: torch.Tensor) -> None:
        # Use the Joseph covariance form to preserve symmetry and positive
        # semidefiniteness after each contact measurement update.
        ids = torch.nonzero(mask, as_tuple=False).squeeze(-1)
        if ids.numel() == 0:
            return

        P = self.P[ids]
        H_i = H[ids]
        N_i = N[ids]
        residual_i = residual[ids]

        PHt = P @ H_i.transpose(-1, -2)
        S = H_i @ PHt + N_i
        try:
            K = torch.linalg.solve(S.transpose(-1, -2), PHt.transpose(-1, -2)).transpose(-1, -2)
        except RuntimeError:
            K = PHt @ torch.linalg.pinv(S)

        delta = (K @ residual_i[..., None]).squeeze(-1)
        self._apply_error(delta, ids)

        eye15 = self._eye(15).expand(ids.numel(), 15, 15)
        KH = K @ H_i
        P_new = (eye15 - KH) @ P @ (eye15 - KH).transpose(-1, -2) + K @ N_i @ K.transpose(-1, -2)
        self.P[ids] = self._symmetrize(P_new)

    def _apply_error(self, delta: torch.Tensor, env_ids: torch.Tensor) -> None:
        self.p[env_ids] = self.p[env_ids] + delta[:, 0:3]
        self.v[env_ids] = self.v[env_ids] + delta[:, 3:6]
        self.R[env_ids] = project_so3(self.R[env_ids] @ so3_exp(delta[:, 6:9]))
        self.bg[env_ids] = self.bg[env_ids] + delta[:, 9:12]
        self.ba[env_ids] = self.ba[env_ids] + delta[:, 12:15]

    def _measurement_tensors(
        self,
        measurements: Any,
        foot_jacobians: Optional[TensorLike] = None,
    ) -> Tuple[torch.Tensor, Optional[torch.Tensor], torch.Tensor]:
        # Normalize the different measurement containers accepted by the
        # analytic and MuJoCo backends into batched tensors with one leg axis.
        if isinstance(measurements, Mapping) and (
            "position_base" in measurements or "positions_base" in measurements or "foot_positions_base" in measurements
        ):
            key = "position_base"
            if key not in measurements:
                key = "positions_base" if "positions_base" in measurements else "foot_positions_base"
            positions = self._foot_positions_tensor(measurements[key])
            jacobians = self._jacobians_tensor(measurements.get("jacobian", measurements.get("jacobians", foot_jacobians)))
            valid = torch.ones((self.num_envs, len(self.leg_order)), device=self.device, dtype=torch.bool)
            return positions, jacobians, valid

        if not isinstance(measurements, Mapping):
            positions = self._foot_positions_tensor(measurements)
            jacobians = self._jacobians_tensor(foot_jacobians)
            valid = torch.ones((self.num_envs, len(self.leg_order)), device=self.device, dtype=torch.bool)
            return positions, jacobians, valid

        positions = torch.zeros((self.num_envs, len(self.leg_order), 3), device=self.device, dtype=self.dtype)
        valid = torch.zeros((self.num_envs, len(self.leg_order)), device=self.device, dtype=torch.bool)
        raw_jacobians = []
        jacobian_width = 0

        for leg_idx, leg in enumerate(self.leg_order):
            measurement = measurements.get(leg)
            if measurement is None:
                raw_jacobians.append(None)
                continue

            position = self._get_measurement_value(measurement, "position_base")
            positions[:, leg_idx] = self._batch_tensor(position, 3, self.num_envs)
            valid[:, leg_idx] = True

            jacobian = self._get_measurement_value(measurement, "jacobian", default=None)
            if jacobian is None:
                raw_jacobians.append(None)
                continue
            jacobian = self._leg_jacobian_tensor(jacobian)
            jacobian_width = max(jacobian_width, jacobian.shape[-1])
            raw_jacobians.append(jacobian)

        if foot_jacobians is not None:
            return positions, self._jacobians_tensor(foot_jacobians), valid
        if jacobian_width == 0:
            return positions, None, valid

        jacobians = torch.zeros(
            (self.num_envs, len(self.leg_order), 3, jacobian_width),
            device=self.device,
            dtype=self.dtype,
        )
        for leg_idx, jacobian in enumerate(raw_jacobians):
            if jacobian is not None:
                jacobians[:, leg_idx, :, : jacobian.shape[-1]] = jacobian
        return positions, jacobians, valid

    def _foot_positions_tensor(self, positions: TensorLike) -> torch.Tensor:
        positions = self._tensor(positions)
        n_legs = len(self.leg_order)
        if positions.ndim == 2 and positions.shape == (n_legs, 3):
            return positions.unsqueeze(0).expand(self.num_envs, n_legs, 3).clone()
        if positions.ndim == 3 and positions.shape[1:] == (n_legs, 3):
            if positions.shape[0] == 1 and self.num_envs != 1:
                return positions.expand(self.num_envs, n_legs, 3).clone()
            if positions.shape[0] == self.num_envs:
                return positions
        raise ValueError(
            "foot positions must have shape (num_legs, 3) or (num_envs, num_legs, 3); "
            f"got {tuple(positions.shape)}"
        )

    def _jacobians_tensor(self, jacobians: Optional[TensorLike]) -> Optional[torch.Tensor]:
        if jacobians is None:
            return None
        jacobians = self._tensor(jacobians)
        n_legs = len(self.leg_order)
        if jacobians.ndim == 3 and jacobians.shape[:2] == (n_legs, 3):
            return jacobians.unsqueeze(0).expand(self.num_envs, n_legs, 3, jacobians.shape[-1]).clone()
        if jacobians.ndim == 4 and jacobians.shape[1:3] == (n_legs, 3):
            if jacobians.shape[0] == 1 and self.num_envs != 1:
                return jacobians.expand(self.num_envs, n_legs, 3, jacobians.shape[-1]).clone()
            if jacobians.shape[0] == self.num_envs:
                return jacobians
        raise ValueError(
            "foot jacobians must have shape (num_legs, 3, dof) or "
            f"(num_envs, num_legs, 3, dof); got {tuple(jacobians.shape)}"
        )

    def _leg_jacobian_tensor(self, jacobian: TensorLike) -> torch.Tensor:
        jacobian = self._tensor(jacobian)
        if jacobian.ndim == 2 and jacobian.shape[0] == 3:
            return jacobian.unsqueeze(0).expand(self.num_envs, 3, jacobian.shape[-1]).clone()
        if jacobian.ndim == 3 and jacobian.shape[0] == self.num_envs and jacobian.shape[1] == 3:
            return jacobian
        if jacobian.ndim == 3 and jacobian.shape[0] == 1 and jacobian.shape[1] == 3:
            return jacobian.expand(self.num_envs, 3, jacobian.shape[-1]).clone()
        raise ValueError(f"leg jacobian must have shape (3, dof) or (num_envs, 3, dof); got {tuple(jacobian.shape)}")

    def _foot_velocity_base(
        self,
        jacobian: torch.Tensor,
        joint_velocities: TensorLike,
        leg_idx: int,
    ) -> Tuple[torch.Tensor, torch.Tensor]:
        qd = self._tensor(joint_velocities)
        if qd.ndim == 1:
            qd = qd.unsqueeze(0).expand(self.num_envs, qd.shape[0])
        if qd.ndim == 2 and qd.shape[0] == 1 and self.num_envs != 1:
            qd = qd.expand(self.num_envs, qd.shape[1])

        if qd.ndim == 2 and qd.shape[0] == self.num_envs and qd.shape[1] == jacobian.shape[-1]:
            return torch.einsum("nij,nj->ni", jacobian, qd), torch.ones(self.num_envs, device=self.device, dtype=torch.bool)
        if qd.ndim == 3 and qd.shape[0] == self.num_envs and qd.shape[1] == len(self.leg_order) and qd.shape[2] == jacobian.shape[-1]:
            return (
                torch.einsum("nij,nj->ni", jacobian, qd[:, leg_idx]),
                torch.ones(self.num_envs, device=self.device, dtype=torch.bool),
            )
        return torch.zeros((self.num_envs, 3), device=self.device, dtype=self.dtype), torch.zeros(self.num_envs, device=self.device, dtype=torch.bool)

    def _contacts_tensor(self, contacts: TensorLike) -> torch.Tensor:
        if isinstance(contacts, Mapping):
            result = torch.zeros((self.num_envs, len(self.leg_order)), device=self.device, dtype=torch.bool)
            for leg_idx, leg in enumerate(self.leg_order):
                if leg not in contacts:
                    continue
                value = torch.as_tensor(contacts[leg], device=self.device)
                if value.ndim == 0:
                    result[:, leg_idx] = bool(value.item())
                else:
                    value = value.to(device=self.device, dtype=torch.bool).reshape(-1)
                    if value.numel() == 1:
                        result[:, leg_idx] = bool(value.item())
                    elif value.numel() == self.num_envs:
                        result[:, leg_idx] = value
                    else:
                        raise ValueError(f"contact flag for {leg} must be scalar or num_envs long")
            return result

        contacts = torch.as_tensor(contacts, device=self.device)
        if contacts.ndim == 1 and contacts.shape[0] == len(self.leg_order):
            return contacts.to(dtype=torch.bool).unsqueeze(0).expand(self.num_envs, len(self.leg_order)).clone()
        if contacts.ndim == 2 and contacts.shape[1] == len(self.leg_order):
            contacts = contacts.to(dtype=torch.bool)
            if contacts.shape[0] == 1 and self.num_envs != 1:
                return contacts.expand(self.num_envs, len(self.leg_order)).clone()
            if contacts.shape[0] == self.num_envs:
                return contacts
        raise ValueError(
            "contacts must have shape (num_legs,) or (num_envs, num_legs); "
            f"got {tuple(contacts.shape)}"
        )

    def _batch_tensor(self, value: TensorLike, width: int, count: int) -> torch.Tensor:
        tensor = self._tensor(value)
        if tensor.ndim == 1 and tensor.shape[0] == width:
            return tensor.unsqueeze(0).expand(count, width).clone()
        if tensor.ndim == 2 and tensor.shape[1] == width:
            if tensor.shape[0] == 1 and count != 1:
                return tensor.expand(count, width).clone()
            if tensor.shape[0] == count:
                return tensor
        raise ValueError(f"expected shape ({width},) or ({count}, {width}), got {tuple(tensor.shape)}")

    def _dt_tensor(self, dt: TensorLike) -> torch.Tensor:
        dt = self._tensor(dt).reshape(-1)
        if dt.numel() == 1:
            dt = dt.expand(self.num_envs)
        elif dt.numel() != self.num_envs:
            raise ValueError(f"dt must be scalar or have {self.num_envs} values")
        return dt.clamp(1e-5, self.max_dt)

    def _env_ids(self, env_ids: Optional[TensorLike]) -> torch.Tensor:
        if env_ids is None:
            return torch.arange(self.num_envs, device=self.device, dtype=torch.long)
        ids = torch.as_tensor(env_ids, device=self.device, dtype=torch.long).reshape(-1)
        if bool((ids < 0).any()) or bool((ids >= self.num_envs).any()):
            raise IndexError("env_ids out of range")
        return ids

    def _tensor(self, value: TensorLike) -> torch.Tensor:
        return torch.as_tensor(value, device=self.device, dtype=self.dtype)

    def _eye(self, n: int) -> torch.Tensor:
        return torch.eye(n, device=self.device, dtype=self.dtype)

    def _initial_covariance(self) -> torch.Tensor:
        P = self._eye(15) * 0.05
        P[0:3, 0:3] = self._eye(3) * 0.05
        P[3:6, 3:6] = self._eye(3) * 0.1
        P[6:9, 6:9] = self._eye(3) * 0.02
        P[9:12, 9:12] = self._eye(3) * 0.02
        P[12:15, 12:15] = self._eye(3) * 0.1
        return P

    def _get_measurement_value(self, measurement: Any, name: str, default: Any = None) -> Any:
        if isinstance(measurement, Mapping):
            return measurement.get(name, default)
        return getattr(measurement, name, default)

    @staticmethod
    def _symmetrize(P: torch.Tensor) -> torch.Tensor:
        P = 0.5 * (P + P.transpose(-1, -2))
        diag = torch.diagonal(P, dim1=-2, dim2=-1)
        diag_clamped = diag.clamp_min(1e-12)
        return P - torch.diag_embed(diag) + torch.diag_embed(diag_clamped)




if __name__ == "__main__":
    leg_order = tuple(config.leg_order)
    device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
    foot_positions = torch.tensor(
        [
            [0.30, 0.18, -0.45],
            [0.30, -0.18, -0.45],
            [-0.30, 0.18, -0.45],
            [-0.30, -0.18, -0.45],
        ],
        device=device,
        dtype=torch.float32,
    )

    single = ContactAidedEKF(num_envs=1, leg_order=leg_order, device=device, configure_kinematics=False)
    single_position = torch.tensor([[0.0, 0.0, 0.35]], device=device, dtype=single.dtype)
    single_orientation = torch.tensor([[1.0, 0.0, 0.0, 0.0]], device=device, dtype=single.dtype)
    single_gyro = torch.zeros((1, 3), device=device, dtype=single.dtype)
    single_accel = torch.tensor([[0.0, 0.0, 9.81]], device=device, dtype=single.dtype)
    single_contacts = torch.ones((1, len(leg_order)), device=device, dtype=torch.bool)
    single_dt = torch.tensor([0.01], device=device, dtype=single.dtype)

    runs = 20
    for _ in range(3):
        single.reset(position=single_position, orientation_wxyz=single_orientation)
        single.step(
            gyro=single_gyro,
            accel=single_accel,
            joint_measurements=foot_positions,
            contacts=single_contacts,
            dt=single_dt,
        )
    if device.type == "cuda":
        torch.cuda.synchronize()
    start = time.perf_counter()
    for _ in range(runs):
        single.reset(position=single_position, orientation_wxyz=single_orientation)
        single.step(
            gyro=single_gyro,
            accel=single_accel,
            joint_measurements=foot_positions,
            contacts=single_contacts,
            dt=single_dt,
        )
    if device.type == "cuda":
        torch.cuda.synchronize()
    end = time.perf_counter()
    print(f"Single EKF step on {device} took {((end - start) * 1000.0 / runs):.2f} ms")


    batch_size = 2
    batched = ContactAidedEKF(num_envs=batch_size, leg_order=leg_order, device=device, configure_kinematics=False)

    position = torch.zeros((batch_size, 3), device=device, dtype=batched.dtype)
    position[:, 0] = torch.arange(batch_size, device=device, dtype=batched.dtype)
    position[:, 2] = 0.35
    orientation = torch.zeros((batch_size, 4), device=device, dtype=batched.dtype)
    orientation[:, 0] = 1.0
    linear_velocity = torch.zeros((batch_size, 3), device=device, dtype=batched.dtype)
    linear_velocity[:, 0] = torch.linspace(0.0, 0.1, batch_size, device=device, dtype=batched.dtype)
    gyro = torch.zeros((batch_size, 3), device=device, dtype=batched.dtype)
    gyro[:, 2] = torch.linspace(0.0, 0.1, batch_size, device=device, dtype=batched.dtype)
    accel = torch.zeros((batch_size, 3), device=device, dtype=batched.dtype)
    accel[:, 2] = 9.81
    contacts = torch.ones((batch_size, len(leg_order)), device=device, dtype=torch.bool)
    contacts[1::2, 1] = False
    contacts[1::2, 3] = False
    dt = torch.linspace(0.01, 0.02, batch_size, device=device, dtype=batched.dtype)

    for _ in range(3):
        batched.reset(position=position, orientation_wxyz=orientation, linear_velocity=linear_velocity)
        batched.step(gyro=gyro, accel=accel, joint_measurements=foot_positions, contacts=contacts, dt=dt)
    if device.type == "cuda":
        torch.cuda.synchronize()
    start = time.perf_counter()
    for _ in range(runs):
        batched.reset(position=position, orientation_wxyz=orientation, linear_velocity=linear_velocity)
        batched.step(gyro=gyro, accel=accel, joint_measurements=foot_positions, contacts=contacts, dt=dt)
    if device.type == "cuda":
        torch.cuda.synchronize()
    end = time.perf_counter()
    print(f"Batched EKF step on {device} took {((end - start) * 1000.0 / runs):.2f} ms")

    print("single position:", single.p.detach().cpu())
    print("single active contacts:", single.active_contacts.detach().cpu())
    print("batched position shape:", tuple(batched.p.shape))
    print("batched active contacts:", batched.active_contacts.detach().cpu())
