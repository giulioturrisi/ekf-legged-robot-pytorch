"""Central configuration for the contact-aided state estimator.

Edit this file to switch robot, kinematics backend, device, or filter noise.
Environment variables remain available for runtime overrides where the existing
ROS2 launch workflow already uses them.
"""

from __future__ import annotations


# Robot model and signal ordering must agree with the ROS2 LowState message.
robot = "go2"
leg_order = ("FR", "FL", "RR", "RL")
joint_names = tuple(
    f"{leg}_{joint}_joint"
    for leg in leg_order
    for joint in ("hip", "thigh", "calf")
)
foot_names = {leg: f"{leg}_foot" for leg in leg_order}

warp_device = "cpu"
# Set to "cuda" to run the PyTorch EKF on the GPU. MuJoCo Warp remains on
# the device selected by warp_device.
torch_device = "cpu"

# Standard deviations used by EKFNoise. Values are SI units where applicable.
noise = {
    "gyro": 0.015,
    "accel": 0.05,
    "gyro_bias": 0.0005,
    "accel_bias": 0.005,
    "contact_position": 0.02,
    "contact_velocity": 0.08,
    "joint_position": 0.003,
}

max_dt = 0.05
gravity = (0.0, 0.0, -9.81)
# Stance feet provide a zero-velocity observation that limits IMU drift.
use_contact_velocity = True
