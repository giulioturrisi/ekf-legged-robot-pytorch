# ekf-legged-robot-pytorch


### Conda installation

1. install [miniforge/conda](https://github.com/conda-forge/miniforge/releases) (x86_64 or arm64 depending on your platform)

2. create a ROS 2 environment using a file in the folder [installation](./installation). For NVIDIA GPU support:


```bash
conda env create -f installation/mamba_environment_ros2_nvidia.yaml
conda activate ekf_legged_robot_nvidia_env

```

For CPU-only installation (no GPU or CUDA toolkit required):

```bash
conda env create -f installation/mamba_environment_ros2_cpu.yaml
conda activate ekf_legged_robot_cpu_env
```

Keep `torch_device = "cpu"` and `warp_device = "cpu"` in `config.py`
(the current defaults). The CPU environment explicitly selects the CPU build of
PyTorch and retains Warp and MuJoCo Warp for the kinematics backend.

### Run

In config.py you can change noise and stuff. Then:

```bash
python3 run_state_estimator_ros2.py

```

P.S. Even when you launch plotjuggler, remember to source  **./ros2_localhost_connect.sh**

```bash
source ros2_ws/install/setup.bash
ros2 run plotjuggler plotjuggler

```

### More Info

All the state information are taken from the message LowState. The node publish BaseState with linear velocity in world, e BaseStateDebug with linear velocity in base frame for easier debug.


To run a simulator/controller with the ground truth, use:


https://github.com/iit-DLSLab/Quadruped-PyMPC/tree/feat/cleaning_helpers_main

and follow its istruction.
