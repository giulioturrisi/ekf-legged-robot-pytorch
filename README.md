# ekf-legged-robot-pytorch


### Conda installation

1. install [miniforge/conda](https://github.com/conda-forge/miniforge/releases) (x86_64 or arm64 depending on your platform)

2. create an environment using the file in the folder [installation](./installation) choosing between **nvidia, integrated gpu and ros2 version**:


```bash
conda env create -f mamba_environment_ros2.yml
conda activate ekf_legged_robot_env

```

### Run

In config.py you can change noise and stuff. Then:

```bash
source ./ros2_localhost_connect.sh
python3 run_state_estimator_ros2.py

```

P.S. Even when you launch plotjuggler, remember to source  **./ros2_localhost_connect.sh**

```bash
source ./ros2_localhost_connect.sh
ros2 run plotjuggler plotjuggler

```

### More Info

All the state information are taken from the message LowState. The node publish BaseState with linear velocity in world, e BaseStateDebug with linear velocity in base frame for easier debug.


To run a simulator/controller with the ground truth, use:


https://github.com/iit-DLSLab/Quadruped-PyMPC/tree/feat/cleaning_helpers_main

and follow its istruction.