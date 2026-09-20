# Description: ROS2 node for the PyTorch contact-aided IEKF state estimator.

from __future__ import annotations

import os

# Fail-safe: se non è stato scelto esplicitamente altro,
# ROS 2 comunica solamente sulla macchina locale.
os.environ.setdefault("ROS_LOCALHOST_ONLY", "1")

print(
    "ROS 2 network mode:",
    "LOCALHOST" if os.environ["ROS_LOCALHOST_ONLY"] == "1" else "NETWORK",
)

import sys
import shlex
import subprocess
from pathlib import Path

dir_path = Path(__file__).resolve().parent
sys.path.append(str(dir_path / ".."))

ros_ws = dir_path / "ros2_ws"
setup_bash = ros_ws / "install" / "setup.bash"

if not setup_bash.exists():
    print("Building the msgs first...")
    subprocess.run(["colcon", "build"], cwd=ros_ws, check=True)

if os.environ.get("BASIC_LOCOMOTION_ROS2_SOURCED") != "1":
    print("Sourcing ROS2 workspace and restarting script...")
    cmd = (
        f"source {shlex.quote(str(setup_bash))} && "
        "export BASIC_LOCOMOTION_ROS2_SOURCED=1 && "
        f"exec {shlex.quote(sys.executable)} "
        + " ".join(shlex.quote(arg) for arg in [str(Path(__file__).resolve()), *sys.argv[1:]])
    )
    os.execv("/bin/bash", ["bash", "-c", cmd])
import sys
import time
from pathlib import Path
from typing import Dict, Iterable, List, Sequence

PROJECT_ROOT = Path(__file__).resolve().parent
if str(PROJECT_ROOT) not in sys.path:
    sys.path.insert(0, str(PROJECT_ROOT))

import numpy as np
import torch

import rclpy
from rclpy.node import Node
from dls2_interface.msg import BaseState, BaseStateDebug

try:
    from unitree_go.msg import LowState
except ImportError:  # pragma: no cover
    LowState = None

import config
from contact_aided_ekf import ContactAidedEKF


def normalize_quat_order(order: str) -> str:
    value = str(order).strip().lower()
    if value not in {"wxyz", "xyzw"}:
        raise ValueError(f"Unsupported quaternion order '{order}'. Use 'wxyz' or 'xyzw'.")
    return value


def quat_to_wxyz(quat: Sequence[float], order: str) -> np.ndarray:
    q = np.asarray(quat, dtype=float).reshape(4)
    if order == "wxyz":
        return q
    return q[[3, 0, 1, 2]]


def quat_from_wxyz(quat: Sequence[float], order: str) -> np.ndarray:
    q = np.asarray(quat, dtype=float).reshape(4)
    if order == "wxyz":
        return q
    return q[[1, 2, 3, 0]]


def tensor(value, *, dtype=torch.float32, device="cpu", shape=None) -> torch.Tensor:
    out = torch.as_tensor(value, dtype=dtype, device=device)
    return out.reshape(shape) if shape is not None else out


def numpy_from_torch(value) -> np.ndarray:
    if isinstance(value, torch.Tensor):
        return value.detach().cpu().numpy()
    return np.asarray(value, dtype=float)


def leg_joint_names(leg: str) -> List[str]:
    return [f"{leg}_hip_joint", f"{leg}_thigh_joint", f"{leg}_calf_joint"]


def motor_index_by_joint(motor_leg_order: Sequence[str]) -> Dict[str, int]:
    mapping: Dict[str, int] = {}
    index = 0
    for leg in motor_leg_order:
        for joint_name in leg_joint_names(leg):
            mapping[joint_name] = index
            index += 1
    return mapping


class StateEstimatorROS2(Node):
    def __init__(self) -> None:
        super().__init__("StateEstimatorROS2")
        if LowState is None:
            raise RuntimeError("unitree_go.msg.LowState is required to read /lowstate")

        self.leg_order = tuple(config.leg_order)
        self.motor_leg_order = ("FR", "FL", "RR", "RL")
        self.foot_force_leg_order = ("FR", "FL", "RR", "RL")
        self.input_quat_order = "xyzw"
        self.output_quat_order = "xyzw"
        self.contact_force_threshold = config.contact_force_threshold
        self.assume_all_stance = False
        self.low_state_topic = "/lowstate"
        self.output_topic = "/ekf_base_state"

        self.motor_indices = self._motor_indices_for_config_joints()
        self.filter = ContactAidedEKF(num_envs=1, leg_order=self.leg_order, device=config.torch_device)

        self.subscription_low_state = self.create_subscription(LowState, self.low_state_topic, self.low_state_callback, 1)
        self.publisher_estimated_base_state = self.create_publisher(BaseState, self.output_topic, 1)
        self.publisher_estimated_state_debug = self.create_publisher(
            BaseStateDebug, "/ekf_base_state_debug", 1
        )

        self.initialized = False
        self.last_update_time: float | None = None
        self.publish_count = 0

        self.joint_positions = torch.zeros(
            (1, len(config.joint_names)), dtype=self.filter.dtype, device=self.filter.device
        )
        self.joint_velocities = torch.zeros_like(self.joint_positions)
        self.contacts = torch.zeros((len(self.leg_order),), dtype=torch.bool)
        self.imu_accel = torch.zeros(3, dtype=self.filter.dtype)
        self.imu_gyro = torch.zeros(3, dtype=self.filter.dtype)
        self.imu_quat_wxyz = torch.tensor([1.0, 0.0, 0.0, 0.0], dtype=self.filter.dtype)

        self.get_logger().info(
            f"Subscribed to {self.low_state_topic}; publishing {self.output_topic}; "
            f"motor_leg_order={self.motor_leg_order}; foot_force_leg_order={self.foot_force_leg_order}"
        )
        self.get_logger().info(
            f"IEKF tuning: {self.filter.tuning_parameters()}; "
            f"orientation_source=imu; contact_threshold={self.contact_force_threshold:.3f} N"
        )

    def _motor_indices_for_config_joints(self) -> List[int]:
        mapping = motor_index_by_joint(self.motor_leg_order)
        missing = [joint for joint in config.joint_names if joint not in mapping]
        if missing:
            raise ValueError("No LowState motor_state mapping for joints: " + ", ".join(missing))
        return [mapping[joint] for joint in config.joint_names]

    def low_state_callback(self, msg) -> None:
        self._read_low_state(msg)
        now = time.perf_counter()
        if not self.initialized:
            self._initialize_filter_from_current_sample()
            self.last_update_time = now
            self.initialized = True
            self.publish_estimated_state()
            return
        dt = now - self.last_update_time
        self.last_update_time = now
        measurements = self.filter.contact_measurements(self.joint_positions)
        self.filter.step(
            gyro=self.imu_gyro,
            accel=self.imu_accel,
            contacts=self.contacts,
            dt=dt,
            joint_measurements=measurements,
            joint_positions=self.joint_positions,
            joint_velocities=self.joint_velocities,
        )
        self.publish_estimated_state()

    def _read_low_state(self, msg) -> None:
        motor_states = list(msg.motor_state)
        self.imu_accel = tensor(msg.imu_state.accelerometer, dtype=self.filter.dtype, device=self.filter.device)
        self.imu_gyro = tensor(msg.imu_state.gyroscope, dtype=self.filter.dtype, device=self.filter.device)
        self.imu_quat_wxyz = tensor(quat_to_wxyz(msg.imu_state.quaternion, self.input_quat_order), dtype=self.filter.dtype, device=self.filter.device)
        quat_norm = self.imu_quat_wxyz.norm()
        if float(quat_norm) > 1e-9:
            self.imu_quat_wxyz = self.imu_quat_wxyz / quat_norm
        self.joint_positions = tensor([motor_states[index].q for index in self.motor_indices], dtype=self.filter.dtype, device=self.filter.device, shape=(1, -1))
        self.joint_velocities = tensor([motor_states[index].dq for index in self.motor_indices], dtype=self.filter.dtype, device=self.filter.device, shape=(1, -1))
        self.contacts = self._contacts_from_low_state(msg)

    def _contacts_from_low_state(self, msg) -> torch.Tensor:
        if self.assume_all_stance:
            return torch.ones(len(self.leg_order), dtype=torch.bool, device=self.filter.device)
        foot_forces = np.abs(np.asarray(msg.foot_force, dtype=float))
        contacts_by_leg = {
            leg: bool(foot_forces[index] > self.contact_force_threshold)
            for index, leg in enumerate(self.foot_force_leg_order)
        }
        return torch.tensor([contacts_by_leg.get(leg, False) for leg in self.leg_order], dtype=torch.bool)

    def _initialize_filter_from_current_sample(self) -> None:
        orientation = self._initial_orientation()
        position = torch.zeros(3, dtype=self.filter.dtype, device=self.filter.device)
        velocity = torch.zeros(3, dtype=self.filter.dtype, device=self.filter.device)
        self.filter.reset(position=position, orientation_wxyz=orientation, linear_velocity=velocity)
        measurements = self.filter.contact_measurements(self.joint_positions)
        self.filter.set_contacts(self.contacts, measurements)
        self.get_logger().info(
            f"State estimator initialized from first {self.low_state_topic} message with contacts={self.contacts.tolist()}"
        )

    def _initial_orientation(self) -> torch.Tensor:
        return self.imu_quat_wxyz.clone()

    def publish_estimated_state(self) -> None:
        msg = BaseState()
        with torch.no_grad():
            state = self.filter.state_dict()
            position = numpy_from_torch(state["position"][0])
            quat_wxyz = numpy_from_torch(state["quat_wxyz"][0])
            linear_velocity = numpy_from_torch(state["linear_velocity"][0])
            linear_velocity_base = numpy_from_torch(
                self.filter.R[0].transpose(0, 1) @ state["linear_velocity"][0]
            )
            angular_velocity = numpy_from_torch(self.imu_gyro - state["gyro_bias"][0])
        if hasattr(msg, "frame_id"):
            msg.frame_id = "world"
        if hasattr(msg, "robot_name"):
            msg.robot_name = config.robot
        if hasattr(msg, "sequence_id"):
            msg.sequence_id = int(self.publish_count)
        if hasattr(msg, "timestamp"):
            msg.timestamp = float(self.get_clock().now().nanoseconds) * 1e-9
        msg.pose.position = position.tolist()
        msg.pose.orientation = quat_from_wxyz(quat_wxyz, self.output_quat_order).tolist()
        msg.velocity.linear = linear_velocity.tolist()
        msg.velocity.angular = angular_velocity.tolist()
        msg.stance_status = self.contacts.detach().cpu().to(dtype=torch.bool).tolist()
        self.publisher_estimated_base_state.publish(msg)

        debug_msg = BaseStateDebug()
        debug_msg.frame_id = "world"
        debug_msg.sequence_id = self.publish_count
        debug_msg.timestamp = float(self.get_clock().now().nanoseconds) * 1e-9
        debug_msg.robot_name = config.robot
        debug_msg.linear_velocity_base = np.asarray(
            linear_velocity_base, dtype=np.float32
        ).reshape(3).tolist()
        self.publisher_estimated_state_debug.publish(debug_msg)
        self.publish_count += 1
        if self.publish_count == 1:
            self.get_logger().info("Published first estimated base state")

def main(args: Iterable[str] | None = None) -> None:
    rclpy.init(args=args)
    node = StateEstimatorROS2()
    try:
        rclpy.spin(node)
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
