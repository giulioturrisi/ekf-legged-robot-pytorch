#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to dls2_interface__msg__Pose

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pose {

    // This member is not documented.
    #[allow(missing_docs)]
    pub position: [f64; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation: [f64; 4],

}



impl Default for Pose {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Pose::default())
  }
}

impl rosidl_runtime_rs::Message for Pose {
  type RmwMsg = super::msg::rmw::Pose;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: msg.position,
        orientation: msg.orientation,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        position: msg.position,
        orientation: msg.orientation,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      position: msg.position,
      orientation: msg.orientation,
    }
  }
}


// Corresponds to dls2_interface__msg__Screw

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Screw {

    // This member is not documented.
    #[allow(missing_docs)]
    pub linear: [f64; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub angular: [f64; 3],

}



impl Default for Screw {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Screw::default())
  }
}

impl rosidl_runtime_rs::Message for Screw {
  type RmwMsg = super::msg::rmw::Screw;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        linear: msg.linear,
        angular: msg.angular,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        linear: msg.linear,
        angular: msg.angular,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      linear: msg.linear,
      angular: msg.angular,
    }
  }
}


// Corresponds to dls2_interface__msg__BlindState
/// Header

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BlindState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_name: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_position: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_velocity: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_acceleration: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_effort: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_temperature: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feet_contact: Vec<bool>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_feet_positions: Vec<f64>,

}



impl Default for BlindState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BlindState::default())
  }
}

impl rosidl_runtime_rs::Message for BlindState {
  type RmwMsg = super::msg::rmw::BlindState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        sequence_id: msg.sequence_id,
        timestamp: msg.timestamp,
        robot_name: msg.robot_name.as_str().into(),
        joints_name: msg.joints_name
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        joints_position: msg.joints_position.into(),
        joints_velocity: msg.joints_velocity.into(),
        joints_acceleration: msg.joints_acceleration.into(),
        joints_effort: msg.joints_effort.into(),
        joints_temperature: msg.joints_temperature.into(),
        feet_contact: msg.feet_contact.into(),
        current_feet_positions: msg.current_feet_positions.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
        robot_name: msg.robot_name.as_str().into(),
        joints_name: msg.joints_name
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        joints_position: msg.joints_position.as_slice().into(),
        joints_velocity: msg.joints_velocity.as_slice().into(),
        joints_acceleration: msg.joints_acceleration.as_slice().into(),
        joints_effort: msg.joints_effort.as_slice().into(),
        joints_temperature: msg.joints_temperature.as_slice().into(),
        feet_contact: msg.feet_contact.as_slice().into(),
        current_feet_positions: msg.current_feet_positions.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame_id: msg.frame_id.to_string(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
      robot_name: msg.robot_name.to_string(),
      joints_name: msg.joints_name
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      joints_position: msg.joints_position
          .into_iter()
          .collect(),
      joints_velocity: msg.joints_velocity
          .into_iter()
          .collect(),
      joints_acceleration: msg.joints_acceleration
          .into_iter()
          .collect(),
      joints_effort: msg.joints_effort
          .into_iter()
          .collect(),
      joints_temperature: msg.joints_temperature
          .into_iter()
          .collect(),
      feet_contact: msg.feet_contact
          .into_iter()
          .collect(),
      current_feet_positions: msg.current_feet_positions
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to dls2_interface__msg__BaseState
/// Header

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BaseState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_name: std::string::String,

    /// Base pose
    pub pose: super::msg::Pose,

    /// Base velocity
    pub velocity: super::msg::Screw,

    /// Base acceleration
    pub acceleration: super::msg::Screw,

    /// Stance status
    pub stance_status: Vec<bool>,

}



impl Default for BaseState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BaseState::default())
  }
}

impl rosidl_runtime_rs::Message for BaseState {
  type RmwMsg = super::msg::rmw::BaseState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        sequence_id: msg.sequence_id,
        timestamp: msg.timestamp,
        robot_name: msg.robot_name.as_str().into(),
        pose: super::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        velocity: super::msg::Screw::into_rmw_message(std::borrow::Cow::Owned(msg.velocity)).into_owned(),
        acceleration: super::msg::Screw::into_rmw_message(std::borrow::Cow::Owned(msg.acceleration)).into_owned(),
        stance_status: msg.stance_status.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
        robot_name: msg.robot_name.as_str().into(),
        pose: super::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
        velocity: super::msg::Screw::into_rmw_message(std::borrow::Cow::Borrowed(&msg.velocity)).into_owned(),
        acceleration: super::msg::Screw::into_rmw_message(std::borrow::Cow::Borrowed(&msg.acceleration)).into_owned(),
        stance_status: msg.stance_status.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame_id: msg.frame_id.to_string(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
      robot_name: msg.robot_name.to_string(),
      pose: super::msg::Pose::from_rmw_message(msg.pose),
      velocity: super::msg::Screw::from_rmw_message(msg.velocity),
      acceleration: super::msg::Screw::from_rmw_message(msg.acceleration),
      stance_status: msg.stance_status
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to dls2_interface__msg__BaseStateDebug
/// Header

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BaseStateDebug {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub linear_velocity_base: [f32; 3],

}



impl Default for BaseStateDebug {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BaseStateDebug::default())
  }
}

impl rosidl_runtime_rs::Message for BaseStateDebug {
  type RmwMsg = super::msg::rmw::BaseStateDebug;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        sequence_id: msg.sequence_id,
        timestamp: msg.timestamp,
        robot_name: msg.robot_name.as_str().into(),
        linear_velocity_base: msg.linear_velocity_base,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
        robot_name: msg.robot_name.as_str().into(),
        linear_velocity_base: msg.linear_velocity_base,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame_id: msg.frame_id.to_string(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
      robot_name: msg.robot_name.to_string(),
      linear_velocity_base: msg.linear_velocity_base,
    }
  }
}


// Corresponds to dls2_interface__msg__Imu
/// Header

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Imu {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation: [f64; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation_rpy: [f64; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation_covariance: [f64; 9],


    // This member is not documented.
    #[allow(missing_docs)]
    pub angular_velocity: [f64; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub angular_velocity_covariance: [f64; 9],


    // This member is not documented.
    #[allow(missing_docs)]
    pub linear_acceleration: [f64; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub linear_acceleration_covariance: [f64; 9],

}



impl Default for Imu {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Imu::default())
  }
}

impl rosidl_runtime_rs::Message for Imu {
  type RmwMsg = super::msg::rmw::Imu;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        sequence_id: msg.sequence_id,
        timestamp: msg.timestamp,
        orientation: msg.orientation,
        orientation_rpy: msg.orientation_rpy,
        orientation_covariance: msg.orientation_covariance,
        angular_velocity: msg.angular_velocity,
        angular_velocity_covariance: msg.angular_velocity_covariance,
        linear_acceleration: msg.linear_acceleration,
        linear_acceleration_covariance: msg.linear_acceleration_covariance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
        orientation: msg.orientation,
        orientation_rpy: msg.orientation_rpy,
        orientation_covariance: msg.orientation_covariance,
        angular_velocity: msg.angular_velocity,
        angular_velocity_covariance: msg.angular_velocity_covariance,
        linear_acceleration: msg.linear_acceleration,
        linear_acceleration_covariance: msg.linear_acceleration_covariance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame_id: msg.frame_id.to_string(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
      orientation: msg.orientation,
      orientation_rpy: msg.orientation_rpy,
      orientation_covariance: msg.orientation_covariance,
      angular_velocity: msg.angular_velocity,
      angular_velocity_covariance: msg.angular_velocity_covariance,
      linear_acceleration: msg.linear_acceleration,
      linear_acceleration_covariance: msg.linear_acceleration_covariance,
    }
  }
}


// Corresponds to dls2_interface__msg__TrajectoryGenerator
/// Header

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryGenerator {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub com_pose: super::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub com_vel: super::msg::Screw,


    // This member is not documented.
    #[allow(missing_docs)]
    pub com_acc: super::msg::Screw,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_position: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_velocity: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_acceleration: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_effort: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kp: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kd: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wrench: [f64; 6],


    // This member is not documented.
    #[allow(missing_docs)]
    pub stance_legs: Vec<bool>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub nominal_touch_down: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub touch_down: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swing_period: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub normal_force_max: Vec<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub normal_force_min: Vec<f64>,

}



impl Default for TrajectoryGenerator {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajectoryGenerator::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryGenerator {
  type RmwMsg = super::msg::rmw::TrajectoryGenerator;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
        sequence_id: msg.sequence_id,
        timestamp: msg.timestamp,
        com_pose: super::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.com_pose)).into_owned(),
        com_vel: super::msg::Screw::into_rmw_message(std::borrow::Cow::Owned(msg.com_vel)).into_owned(),
        com_acc: super::msg::Screw::into_rmw_message(std::borrow::Cow::Owned(msg.com_acc)).into_owned(),
        joints_position: msg.joints_position.into(),
        joints_velocity: msg.joints_velocity.into(),
        joints_acceleration: msg.joints_acceleration.into(),
        joints_effort: msg.joints_effort.into(),
        kp: msg.kp.into(),
        kd: msg.kd.into(),
        wrench: msg.wrench,
        stance_legs: msg.stance_legs.into(),
        nominal_touch_down: msg.nominal_touch_down.into(),
        touch_down: msg.touch_down.into(),
        swing_period: msg.swing_period.into(),
        normal_force_max: msg.normal_force_max.into(),
        normal_force_min: msg.normal_force_min.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame_id: msg.frame_id.as_str().into(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
        com_pose: super::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.com_pose)).into_owned(),
        com_vel: super::msg::Screw::into_rmw_message(std::borrow::Cow::Borrowed(&msg.com_vel)).into_owned(),
        com_acc: super::msg::Screw::into_rmw_message(std::borrow::Cow::Borrowed(&msg.com_acc)).into_owned(),
        joints_position: msg.joints_position.as_slice().into(),
        joints_velocity: msg.joints_velocity.as_slice().into(),
        joints_acceleration: msg.joints_acceleration.as_slice().into(),
        joints_effort: msg.joints_effort.as_slice().into(),
        kp: msg.kp.as_slice().into(),
        kd: msg.kd.as_slice().into(),
        wrench: msg.wrench,
        stance_legs: msg.stance_legs.as_slice().into(),
        nominal_touch_down: msg.nominal_touch_down.as_slice().into(),
        touch_down: msg.touch_down.as_slice().into(),
        swing_period: msg.swing_period.as_slice().into(),
        normal_force_max: msg.normal_force_max.as_slice().into(),
        normal_force_min: msg.normal_force_min.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame_id: msg.frame_id.to_string(),
      sequence_id: msg.sequence_id,
      timestamp: msg.timestamp,
      com_pose: super::msg::Pose::from_rmw_message(msg.com_pose),
      com_vel: super::msg::Screw::from_rmw_message(msg.com_vel),
      com_acc: super::msg::Screw::from_rmw_message(msg.com_acc),
      joints_position: msg.joints_position
          .into_iter()
          .collect(),
      joints_velocity: msg.joints_velocity
          .into_iter()
          .collect(),
      joints_acceleration: msg.joints_acceleration
          .into_iter()
          .collect(),
      joints_effort: msg.joints_effort
          .into_iter()
          .collect(),
      kp: msg.kp
          .into_iter()
          .collect(),
      kd: msg.kd
          .into_iter()
          .collect(),
      wrench: msg.wrench,
      stance_legs: msg.stance_legs
          .into_iter()
          .collect(),
      nominal_touch_down: msg.nominal_touch_down
          .into_iter()
          .collect(),
      touch_down: msg.touch_down
          .into_iter()
          .collect(),
      swing_period: msg.swing_period
          .into_iter()
          .collect(),
      normal_force_max: msg.normal_force_max
          .into_iter()
          .collect(),
      normal_force_min: msg.normal_force_min
          .into_iter()
          .collect(),
    }
  }
}


