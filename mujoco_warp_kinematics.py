"""MuJoCo Warp kinematics utilities shared by the EKF and IEKF filters."""

from __future__ import annotations

import time
from pathlib import Path
from typing import Any, Dict, List, Mapping, Optional, Sequence, Tuple

import torch

try:
    import warp as wp
except ImportError:  # Warp is only required by the MuJoCo backend.
    wp = None

import config

import sys
import os 
dir_path = os.path.dirname(os.path.realpath(__file__))


TensorLike = Any


class _SubscriptableWarpArrayFactory:
    def __init__(self, factory: Any) -> None:
        self._factory = factory

    def __call__(self, *args: Any, **kwargs: Any) -> Any:
        return self._factory(*args, **kwargs)

    def __getitem__(self, dtype: Any) -> Any:
        return self._factory(dtype=dtype)

    def __or__(self, other: Any) -> Any:
        return self

    def __ror__(self, other: Any) -> Any:
        return self

    def __getattr__(self, name: str) -> Any:
        return getattr(self._factory, name)


class _TextureFilterMode:
    LINEAR = 0


def _ensure_warp_texture_types() -> None:
    # Keep imports working across Warp releases with different texture APIs.
    if wp is None:
        return
    try:
        from warp._src import texture as warp_texture
    except ImportError:
        warp_texture = None

    for name in ("Texture1D", "Texture2D", "Texture3D"):
        if hasattr(wp, name):
            continue
        texture_type = getattr(warp_texture, name, None) if warp_texture is not None else None
        if texture_type is None and name != "Texture1D":
            texture_type = getattr(wp, "Texture1D", None)
        if texture_type is None:
            texture_type = getattr(wp, "uint64", None)
        if texture_type is None:
            texture_type = type(name, (), {})
        setattr(wp, name, texture_type)

    if not hasattr(wp, "TextureFilterMode"):
        wp.TextureFilterMode = _TextureFilterMode


def _ensure_subscriptable_warp_array_types() -> None:
    # Older Warp versions do not expose all array types as generic annotations.
    if wp is None:
        return
    for name in ("array", "array1d", "array2d", "array3d", "array4d"):
        factory = getattr(wp, name, None)
        if factory is None:
            continue
        try:
            annotation = factory[float]
        except TypeError:
            setattr(wp, name, _SubscriptableWarpArrayFactory(factory))
            continue
        if isinstance(factory, type) and not isinstance(annotation, factory):
            factory.__class_getitem__ = classmethod(lambda cls, dtype: cls(dtype=dtype))


def _ensure_warp_runtime_helpers() -> None:
    # MuJoCo Warp may lack this helper even though the rest of the API exists.
    if wp is None:
        return
    if hasattr(wp, "is_conditional_graph_supported"):
        return

    def is_conditional_graph_supported() -> bool:
        assert_supported = getattr(wp, "assert_conditional_graph_support", None)
        if assert_supported is None:
            try:
                from warp import context as wp_context
            except ImportError:
                return False
            assert_supported = getattr(wp_context, "assert_conditional_graph_support", None)
        if assert_supported is None:
            return False
        try:
            assert_supported()
        except Exception:
            return False
        return True

    wp.is_conditional_graph_supported = is_conditional_graph_supported


_ensure_warp_texture_types()
_ensure_subscriptable_warp_array_types()
_ensure_warp_runtime_helpers()


def _patch_mujoco_warp_io() -> None:
    import mujoco_warp._src.io as mjw_io

    if getattr(mjw_io._create_array, "_iekf_shape_compat", False):
        return

    original_create_array = mjw_io._create_array

    def create_array_shape_compat(
        data: Any,
        spec: Any,
        sizes: Dict[str, int],
        batch_size: Optional[int] = None,
    ) -> Any:
        spec_shape = getattr(spec, "shape", (0,))
        if (
            data is not None
            and isinstance(spec_shape, tuple)
            and len(spec_shape) > 1
            and all(dim == 0 for dim in spec_shape)
        ):
            import numpy as np

            return mjw_io.wp.array(np.array(data), dtype=spec.dtype)
        if batch_size is None:
            return original_create_array(data, spec, sizes)
        return original_create_array(data, spec, sizes, batch_size)

    create_array_shape_compat._iekf_shape_compat = True
    mjw_io._create_array = create_array_shape_compat


def _warp_enum_compat_type(enum_type: Any) -> type:
    return type(
        enum_type.__name__,
        (),
        {name: int(value) for name, value in enum_type.__members__.items()},
    )


def _patch_mujoco_warp_enums() -> None:
    import enum
    import sys
    import mujoco_warp._src.types as mjw_types

    enum_proxies = {
        name: _warp_enum_compat_type(value)
        for name, value in vars(mjw_types).items()
        if isinstance(value, type) and issubclass(value, (enum.IntEnum, enum.IntFlag))
    }

    for module_name, module in tuple(sys.modules.items()):
        if module_name == "mujoco_warp._src.types" or not module_name.startswith("mujoco_warp._src.") or module is None:
            continue
        for name, proxy in enum_proxies.items():
            if getattr(module, name, None) is getattr(mjw_types, name, None):
                setattr(module, name, proxy)


if wp is not None:

    @wp.kernel
    def _gather_body_xpos(
        xpos: wp.array2d(dtype=wp.vec3),
        body_id: int,
        points: wp.array(dtype=wp.vec3),
    ) -> None:
        world_id = wp.tid()
        points[world_id] = xpos[world_id, body_id]
else:
    _gather_body_xpos = None


class ContactMeasurement:
    """One leg's foot pose and optional position Jacobian in the base frame."""

    def __init__(self, leg: str, position_base: torch.Tensor, jacobian: Optional[torch.Tensor] = None) -> None:
        self.leg = leg
        self.position_base = position_base
        self.jacobian = jacobian


def as_torch(value: TensorLike, *, device: torch.device, dtype: torch.dtype) -> torch.Tensor:
    """Convert Python/torch-like input to a tensor on the kinematics device."""
    if isinstance(value, torch.Tensor):
        return value.to(device=device, dtype=dtype)
    return torch.as_tensor(value, device=device, dtype=dtype)


def build_mujoco_forward_kinematics_from_env(
    prefix: str,
    leg_order: Sequence[str],
    num_envs: int = 1,
) -> "MujocoForwardKinematics":
    # The estimator uses the configured MuJoCo XML model as its only FK source.
    robot = config.robot
    model_path = dir_path + "/robot_model/" + robot + "/" + robot + ".xml"
    joint_names = config.joint_names
    foot_names = config.foot_names
    warp_device = config.warp_device

    return MujocoForwardKinematics(
        model_path=model_path,
        leg_order=leg_order,
        foot_names=foot_names,
        joint_names=joint_names,
        num_envs=num_envs,
        warp_device=warp_device,
    )


class MujocoForwardKinematics:
    """Forward kinematics adapter backed by MuJoCo Warp."""

    def __init__(
        self,
        model_path: str | Path,
        leg_order: Sequence[str] = ("FL", "FR", "RL", "RR"),
        foot_names: Mapping[str, str] | Sequence[str] = ("FL_foot", "FR_foot", "RL_foot", "RR_foot"),
        joint_names: Sequence[str] = (
            "FL_hip_joint",
            "FL_thigh_joint",
            "FL_calf_joint",
            "FR_hip_joint",
            "FR_thigh_joint",
            "FR_calf_joint",
            "RL_hip_joint",
            "RL_thigh_joint",
            "RL_calf_joint",
            "RR_hip_joint",
            "RR_thigh_joint",
            "RR_calf_joint",
        ),
        num_envs: int = 1,
        warp_device: Optional[str] = None,
        dtype: torch.dtype = torch.float32,
    ) -> None:
        # MuJoCo resolves the XML names and addresses; Warp then evaluates the
        # same model in batches for fast positions and Jacobians.
        import mujoco

        self.mujoco = mujoco
        self.model_path = str(model_path)
        self.model = mujoco.MjModel.from_xml_path(self.model_path)
        self._clear_warp_unsupported_collision_margins()
        self.data = mujoco.MjData(self.model)

        self.leg_order = tuple(leg_order)
        self.num_envs = int(num_envs)

        self.warp_device = warp_device
        self.device = torch.device(warp_device if warp_device is not None else "cpu")
        self.dtype = dtype

        self.mjw = None
        self.wp = None
        self.warp_model = None
        self.warp_data = None
        self.warp_nworld = 0
        self.free_qpos_adr = self._find_free_joint_qpos_adr()
        self.base_body_id = self._name_to_id(self.mujoco.mjtObj.mjOBJ_BODY, "base")
        if self.base_body_id < 0:
            raise ValueError(f"Base body 'base' not found in {self.model_path}")

        self.joint_names = tuple(joint_names)

        self.qpos_indices = self._resolve_joint_qpos_indices(self.joint_names)
        self.qvel_indices = self._resolve_joint_qvel_indices(self.joint_names)
        self.foot_locators = self._resolve_foot_locators(foot_names or {})

        self._reset_free_base()
        self._enable_warp()

    def _clear_warp_unsupported_collision_margins(self) -> None:
        # This class uses MuJoCo Warp only for FK/Jacobians. Collision margins
        # are irrelevant here, and MuJoCo Warp rejects nonzero box/mesh margins
        # with MULTICCD enabled.
        self.model.geom_margin[:] = 0.0
        if self.model.npair:
            self.model.pair_margin[:] = 0.0

    def _enable_warp(self) -> None:
        import contextlib

        # Fail at backend selection time instead of failing during module import.
        if wp is None:
            raise RuntimeError(
                "The MuJoCo kinematics backend requires the 'warp-lang' package; "
                "install it in the active conda environment."
            )
        import mujoco_warp as mjw

        _patch_mujoco_warp_io()
        _patch_mujoco_warp_enums()

        self.mjw = mjw
        self.wp = wp
        with self._warp_scope(contextlib):
            self.warp_model = mjw.put_model(self.model)
            self._ensure_warp_data(self.num_envs)
        _patch_mujoco_warp_enums()

    def _warp_scope(self, contextlib_module: Any):
        if self.wp is None or self.warp_device is None:
            return contextlib_module.nullcontext()
        return self.wp.ScopedDevice(self.warp_device)

    def _ensure_warp_data(self, nworld: int) -> None:
        if self.mjw is None or self.wp is None:
            raise RuntimeError("MuJoCo Warp is not available")
        if self.warp_data is not None and self.warp_nworld == nworld:
            return
        self.warp_data = self.mjw.make_data(self.model, nworld=nworld)
        self.warp_nworld = nworld

    def _reset_free_base(self) -> None:
        self.data.qpos[:] = 0.0
        if self.free_qpos_adr is not None:
            self.data.qpos[self.free_qpos_adr : self.free_qpos_adr + 3] = 0.0
            self.data.qpos[self.free_qpos_adr + 3 : self.free_qpos_adr + 7] = [1.0, 0.0, 0.0, 0.0]
        self.data.qvel[:] = 0.0

    def _name_to_id(self, obj_type: int, name: str) -> int:
        return self.mujoco.mj_name2id(self.model, obj_type, name)

    def _find_free_joint_qpos_adr(self) -> Optional[int]:
        for joint_id in range(self.model.njnt):
            if int(self.model.jnt_type[joint_id]) == int(self.mujoco.mjtJoint.mjJNT_FREE):
                return int(self.model.jnt_qposadr[joint_id])
        return None

    def _resolve_joint_qpos_indices(self, joint_names: Sequence[str]) -> Tuple[int, ...]:
        # Joint order in the ROS message is mapped to MuJoCo's qpos layout.
        indices: List[int] = []
        for name in joint_names:
            jid = self._name_to_id(self.mujoco.mjtObj.mjOBJ_JOINT, name)
            if jid < 0:
                raise ValueError(f"Joint '{name}' not found in {self.model_path}")
            indices.append(int(self.model.jnt_qposadr[jid]))
        return tuple(indices)

    def _resolve_joint_qvel_indices(self, joint_names: Sequence[str]) -> Tuple[int, ...]:
        indices: List[int] = []
        for name in joint_names:
            jid = self._name_to_id(self.mujoco.mjtObj.mjOBJ_JOINT, name)
            if jid < 0:
                raise ValueError(f"Joint '{name}' not found in {self.model_path}")
            indices.append(int(self.model.jnt_dofadr[jid]))
        return tuple(indices)

    def _resolve_foot_locators(self, foot_names: Mapping[str, str] | Sequence[str]) -> Dict[str, int]:
        # Store body ids so Warp can gather each foot position without repeated
        # name lookups during every estimator update.
        locators: Dict[str, int] = {}
        for leg_idx, leg in enumerate(self.leg_order):
            if isinstance(foot_names, Mapping):
                name = foot_names.get(leg, f"{leg}_foot")
            elif leg_idx < len(foot_names):
                name = foot_names[leg_idx]
            else:
                name = f"{leg}_foot"

            body_id = self._name_to_id(self.mujoco.mjtObj.mjOBJ_BODY, name)
            if body_id < 0:
                raise ValueError(f"Foot body '{name}' for leg '{leg}' not found in {self.model_path}")
            locators[leg] = int(body_id)
        return locators

    def _base_qpos(self, nworld: int) -> torch.Tensor:
        # Keep the floating base fixed at the origin; only measured leg joints
        # are supplied by the estimator.
        qpos = torch.zeros((nworld, self.model.nq), device=self.device, dtype=torch.float32)
        if self.free_qpos_adr is not None:
            qpos[:, self.free_qpos_adr : self.free_qpos_adr + 3] = 0.0
            qpos[:, self.free_qpos_adr + 3 : self.free_qpos_adr + 7] = qpos.new_tensor(
                [1.0, 0.0, 0.0, 0.0]
            )
        return qpos.contiguous()

    def _joint_position_matrix(self, joint_positions: TensorLike) -> torch.Tensor:
        q = as_torch(joint_positions, device=self.device, dtype=torch.float32)
        joint_count = len(self.qpos_indices)
        expected_shape = (self.num_envs, joint_count)
        if tuple(q.shape) != expected_shape:
            raise ValueError(
                f"joint positions must have shape {expected_shape}; got {tuple(q.shape)}"
            )
        return q.contiguous()

    def _copy_qpos_to_warp(self, qpos: torch.Tensor) -> None:
        # This is the Torch-to-Warp boundary for the batched MuJoCo state.
        qpos = qpos.contiguous()
        self.wp.copy(self.warp_data.qpos, self.wp.from_torch(qpos, dtype=self.wp.float32))

    def _warp_array_to_torch(self, value: Any) -> torch.Tensor:
        return self.wp.to_torch(value).to(device=self.device, dtype=self.dtype)

    def _gather_body_position(self, body_id: int, nworld: int) -> torch.Tensor:
        points_warp = self.wp.empty((nworld,), dtype=self.wp.vec3)
        self.wp.launch(
            _gather_body_xpos,
            dim=nworld,
            inputs=[self.warp_data.xpos, body_id],
            outputs=[points_warp],
        )
        return self._warp_array_to_torch(points_warp)

    def _base_position(self, nworld: int) -> torch.Tensor:
        """Return the base origin so world-frame foot positions become base-frame."""
        return self._gather_body_position(self.base_body_id, nworld)

    def _forward_positions_warp(self, q: torch.Tensor) -> torch.Tensor:
        import contextlib

        # Run MuJoCo's forward kinematics and gather only the configured foot bodies.
        nworld = q.shape[0]
        self._ensure_warp_data(nworld)
        qpos = self._base_qpos(nworld)
        qpos[:, self.qpos_indices] = q.to(dtype=torch.float32)

        with self._warp_scope(contextlib):
            self._copy_qpos_to_warp(qpos)
            self.mjw.kinematics(self.warp_model, self.warp_data)
            positions = [
                self._gather_body_position(self.foot_locators[leg], nworld)
                for leg in self.leg_order
            ]

        return torch.stack(positions, dim=1) - self._base_position(nworld)[:, None, :]

    def _forward_positions_and_jacobians_warp(self, q: torch.Tensor) -> Tuple[torch.Tensor, torch.Tensor]:
        import contextlib

        # MuJoCo's spatial Jacobian includes the full model; select only the
        # measured joint columns before returning it to the EKF.
        nworld = q.shape[0]
        self._ensure_warp_data(nworld)

        qpos = self._base_qpos(nworld)
        qpos[:, self.qpos_indices] = q.to(dtype=torch.float32)

        positions = []
        jacobians = []
        with self._warp_scope(contextlib):
            self._copy_qpos_to_warp(qpos)
            self.mjw.kinematics(self.warp_model, self.warp_data)
            self.mjw.com_pos(self.warp_model, self.warp_data)
            base_position = self._base_position(nworld)

            for leg in self.leg_order:
                body_id = self.foot_locators[leg]
                points_warp = self.wp.empty((nworld,), dtype=self.wp.vec3)
                bodies_warp = self.wp.full((nworld,), body_id, dtype=self.wp.int32)
                jacp = self.wp.zeros((nworld, 3, self.model.nv), dtype=self.wp.float32)
                self.wp.launch(
                    _gather_body_xpos,
                    dim=nworld,
                    inputs=[self.warp_data.xpos, body_id],
                    outputs=[points_warp],
                )
                self.mjw.jac(self.warp_model, self.warp_data, jacp, None, points_warp, bodies_warp)
                positions.append(self._warp_array_to_torch(points_warp) - base_position)
                jacobians.append(self._warp_array_to_torch(jacp)[:, :, self.qvel_indices])

        return torch.stack(positions, dim=1), torch.stack(jacobians, dim=1)

    def foot_positions_base(self, joint_positions: TensorLike) -> torch.Tensor:
        q = self._joint_position_matrix(joint_positions)
        return self._forward_positions_warp(q)

    def foot_jacobians_base(self, joint_positions: TensorLike) -> torch.Tensor:
        q = self._joint_position_matrix(joint_positions)
        _, jacobians = self._forward_positions_and_jacobians_warp(q)
        return jacobians

    def foot_jacobian_base(self, leg: str, joint_positions: TensorLike) -> torch.Tensor:
        jacobians = self.foot_jacobians_base(joint_positions)
        leg_idx = self.leg_order.index(leg)
        return jacobians[:, leg_idx].clone()

    def contact_measurements(self, joint_positions: TensorLike) -> Dict[str, ContactMeasurement]:
        positions = self.foot_positions_base(joint_positions)
        measurements: Dict[str, ContactMeasurement] = {}
        for leg_idx, leg in enumerate(self.leg_order):
            measurements[leg] = ContactMeasurement(
                leg=leg,
                position_base=positions[:, leg_idx].clone(),
                jacobian=None,
            )
        return measurements

    def contact_measurements_with_jacobians(
        self, joint_positions: TensorLike
    ) -> Dict[str, ContactMeasurement]:
        q = self._joint_position_matrix(joint_positions)
        positions, jacobians = self._forward_positions_and_jacobians_warp(q)
        measurements: Dict[str, ContactMeasurement] = {}
        for leg_idx, leg in enumerate(self.leg_order):
            measurements[leg] = ContactMeasurement(
                leg=leg,
                position_base=positions[:, leg_idx].clone(),
                jacobian=jacobians[:, leg_idx].clone(),
            )
        return measurements


def _set_viewer_pose(fk: MujocoForwardKinematics, q: TensorLike) -> torch.Tensor:
    q = torch.as_tensor(q, dtype=torch.float64).reshape(-1)
    fk.data.qpos[:] = fk.model.qpos0
    if fk.free_qpos_adr is not None:
        fk.data.qpos[fk.free_qpos_adr : fk.free_qpos_adr + 7] = fk.model.qpos0[
            fk.free_qpos_adr : fk.free_qpos_adr + 7
        ]
    fk.data.qpos[list(fk.qpos_indices)] = q.detach().cpu().tolist()
    fk.data.qvel[:] = 0.0
    fk.mujoco.mj_forward(fk.model, fk.data)
    if fk.free_qpos_adr is None:
        return torch.zeros(3, dtype=torch.float32)
    return torch.as_tensor(
        fk.data.qpos[fk.free_qpos_adr : fk.free_qpos_adr + 3],
        dtype=torch.float32,
    )


def _update_foot_markers(
    viewer: Any,
    mujoco: Any,
    foot_positions_world: TensorLike,
    leg_order: Sequence[str],
) -> None:
    colors = {
        "FL": [0.1, 0.7, 1.0, 1.0],
        "FR": [1.0, 0.3, 0.2, 1.0],
        "RL": [0.2, 1.0, 0.35, 1.0],
        "RR": [1.0, 0.85, 0.15, 1.0],
    }
    positions = torch.as_tensor(foot_positions_world).detach().cpu().tolist()
    scene = viewer.user_scn
    scene.ngeom = 0
    mat = torch.eye(3, dtype=torch.float64).reshape(-1).tolist()
    size = [0.035, 0.0, 0.0]
    for leg, position in zip(leg_order, positions):
        if scene.ngeom >= scene.maxgeom:
            break
        geom = scene.geoms[scene.ngeom]
        mujoco.mjv_initGeom(
            geom,
            mujoco.mjtGeom.mjGEOM_SPHERE,
            size,
            position,
            mat,
            colors.get(leg, [1.0, 1.0, 1.0, 1.0]),
        )
        geom.label = leg
        scene.ngeom += 1


if __name__ == "__main__":
    robot = config.robot
    model_path = model_path = dir_path + "/robot_model/" + robot + "/" + robot + ".xml"
    leg_order = config.leg_order
    joint_names = config.joint_names
    foot_names = config.foot_names

    num_envs = 1
    warp_device = config.warp_device

    jacobians = True
    markers = True

    fk = MujocoForwardKinematics(
        model_path=model_path,
        leg_order=leg_order,
        foot_names=foot_names,
        joint_names=joint_names,
        num_envs=num_envs,
        warp_device=warp_device,
    )

    q = torch.tensor([0.0, 0.8, -1.6] * 4, device=fk.device, dtype=fk.dtype)
    q_for_kinematics = q.unsqueeze(0).expand(num_envs, q.numel()).clone()
    if jacobians:
        measurements = fk.contact_measurements_with_jacobians(q_for_kinematics)
    else:
        measurements = fk.contact_measurements(q_for_kinematics)

    start = time.time()
    q_second = torch.tensor([0.4, 0.4, -0.8] * 4, device=fk.device, dtype=fk.dtype)
    q_for_kinematics_second = q_second.unsqueeze(0).expand(num_envs, q_second.numel()).clone()
    if jacobians:
        measurements_second = fk.contact_measurements_with_jacobians(q_for_kinematics_second)
    else:        
        measurements_second = fk.contact_measurements(q_for_kinematics_second)
    end = time.time()
    print("ms measurement:", (end - start) * 1000.0)

    for leg in leg_order:
        position = measurements_second[leg].position_base[0]
        print(f"{leg}: {position.detach().cpu().tolist()}")

    import mujoco.viewer

    positions_base = fk.foot_positions_base(q_for_kinematics_second)[0]

    print("Close the MuJoCo viewer window to exit.")
    with mujoco.viewer.launch_passive(fk.model, fk.data) as viewer:
        while viewer.is_running():
            with viewer.lock():
                base_position = _set_viewer_pose(fk, q_second)
                if markers:
                    _update_foot_markers(
                        viewer,
                        fk.mujoco,
                        positions_base + base_position.to(device=positions_base.device, dtype=positions_base.dtype),
                        leg_order,
                    )
            viewer.sync()
            time.sleep(0.02)
