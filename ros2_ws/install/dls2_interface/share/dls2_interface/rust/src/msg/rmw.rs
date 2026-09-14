#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "dls2_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__Pose() -> *const std::ffi::c_void;
}

#[link(name = "dls2_interface__rosidl_generator_c")]
extern "C" {
    fn dls2_interface__msg__Pose__init(msg: *mut Pose) -> bool;
    fn dls2_interface__msg__Pose__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pose>, size: usize) -> bool;
    fn dls2_interface__msg__Pose__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pose>);
    fn dls2_interface__msg__Pose__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pose>, out_seq: *mut rosidl_runtime_rs::Sequence<Pose>) -> bool;
}

// Corresponds to dls2_interface__msg__Pose
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dls2_interface__msg__Pose__init(&mut msg as *mut _) {
        panic!("Call to dls2_interface__msg__Pose__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pose {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Pose__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Pose__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Pose__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pose {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pose where Self: Sized {
  const TYPE_NAME: &'static str = "dls2_interface/msg/Pose";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__Pose() }
  }
}


#[link(name = "dls2_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__Screw() -> *const std::ffi::c_void;
}

#[link(name = "dls2_interface__rosidl_generator_c")]
extern "C" {
    fn dls2_interface__msg__Screw__init(msg: *mut Screw) -> bool;
    fn dls2_interface__msg__Screw__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Screw>, size: usize) -> bool;
    fn dls2_interface__msg__Screw__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Screw>);
    fn dls2_interface__msg__Screw__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Screw>, out_seq: *mut rosidl_runtime_rs::Sequence<Screw>) -> bool;
}

// Corresponds to dls2_interface__msg__Screw
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dls2_interface__msg__Screw__init(&mut msg as *mut _) {
        panic!("Call to dls2_interface__msg__Screw__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Screw {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Screw__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Screw__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Screw__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Screw {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Screw where Self: Sized {
  const TYPE_NAME: &'static str = "dls2_interface/msg/Screw";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__Screw() }
  }
}


#[link(name = "dls2_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__BlindState() -> *const std::ffi::c_void;
}

#[link(name = "dls2_interface__rosidl_generator_c")]
extern "C" {
    fn dls2_interface__msg__BlindState__init(msg: *mut BlindState) -> bool;
    fn dls2_interface__msg__BlindState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BlindState>, size: usize) -> bool;
    fn dls2_interface__msg__BlindState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BlindState>);
    fn dls2_interface__msg__BlindState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BlindState>, out_seq: *mut rosidl_runtime_rs::Sequence<BlindState>) -> bool;
}

// Corresponds to dls2_interface__msg__BlindState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Header

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BlindState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_name: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_position: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_velocity: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_acceleration: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_effort: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_temperature: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feet_contact: rosidl_runtime_rs::Sequence<bool>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_feet_positions: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for BlindState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dls2_interface__msg__BlindState__init(&mut msg as *mut _) {
        panic!("Call to dls2_interface__msg__BlindState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BlindState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BlindState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BlindState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BlindState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BlindState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BlindState where Self: Sized {
  const TYPE_NAME: &'static str = "dls2_interface/msg/BlindState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__BlindState() }
  }
}


#[link(name = "dls2_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__BaseState() -> *const std::ffi::c_void;
}

#[link(name = "dls2_interface__rosidl_generator_c")]
extern "C" {
    fn dls2_interface__msg__BaseState__init(msg: *mut BaseState) -> bool;
    fn dls2_interface__msg__BaseState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BaseState>, size: usize) -> bool;
    fn dls2_interface__msg__BaseState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BaseState>);
    fn dls2_interface__msg__BaseState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BaseState>, out_seq: *mut rosidl_runtime_rs::Sequence<BaseState>) -> bool;
}

// Corresponds to dls2_interface__msg__BaseState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Header

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BaseState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_name: rosidl_runtime_rs::String,

    /// Base pose
    pub pose: super::super::msg::rmw::Pose,

    /// Base velocity
    pub velocity: super::super::msg::rmw::Screw,

    /// Base acceleration
    pub acceleration: super::super::msg::rmw::Screw,

    /// Stance status
    pub stance_status: rosidl_runtime_rs::Sequence<bool>,

}



impl Default for BaseState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dls2_interface__msg__BaseState__init(&mut msg as *mut _) {
        panic!("Call to dls2_interface__msg__BaseState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BaseState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BaseState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BaseState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BaseState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BaseState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BaseState where Self: Sized {
  const TYPE_NAME: &'static str = "dls2_interface/msg/BaseState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__BaseState() }
  }
}


#[link(name = "dls2_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__BaseStateDebug() -> *const std::ffi::c_void;
}

#[link(name = "dls2_interface__rosidl_generator_c")]
extern "C" {
    fn dls2_interface__msg__BaseStateDebug__init(msg: *mut BaseStateDebug) -> bool;
    fn dls2_interface__msg__BaseStateDebug__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BaseStateDebug>, size: usize) -> bool;
    fn dls2_interface__msg__BaseStateDebug__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BaseStateDebug>);
    fn dls2_interface__msg__BaseStateDebug__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BaseStateDebug>, out_seq: *mut rosidl_runtime_rs::Sequence<BaseStateDebug>) -> bool;
}

// Corresponds to dls2_interface__msg__BaseStateDebug
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Header

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BaseStateDebug {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub linear_velocity_base: [f32; 3],

}



impl Default for BaseStateDebug {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dls2_interface__msg__BaseStateDebug__init(&mut msg as *mut _) {
        panic!("Call to dls2_interface__msg__BaseStateDebug__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BaseStateDebug {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BaseStateDebug__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BaseStateDebug__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__BaseStateDebug__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BaseStateDebug {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BaseStateDebug where Self: Sized {
  const TYPE_NAME: &'static str = "dls2_interface/msg/BaseStateDebug";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__BaseStateDebug() }
  }
}


#[link(name = "dls2_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__Imu() -> *const std::ffi::c_void;
}

#[link(name = "dls2_interface__rosidl_generator_c")]
extern "C" {
    fn dls2_interface__msg__Imu__init(msg: *mut Imu) -> bool;
    fn dls2_interface__msg__Imu__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Imu>, size: usize) -> bool;
    fn dls2_interface__msg__Imu__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Imu>);
    fn dls2_interface__msg__Imu__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Imu>, out_seq: *mut rosidl_runtime_rs::Sequence<Imu>) -> bool;
}

// Corresponds to dls2_interface__msg__Imu
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Header

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Imu {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: rosidl_runtime_rs::String,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dls2_interface__msg__Imu__init(&mut msg as *mut _) {
        panic!("Call to dls2_interface__msg__Imu__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Imu {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Imu__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Imu__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__Imu__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Imu {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Imu where Self: Sized {
  const TYPE_NAME: &'static str = "dls2_interface/msg/Imu";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__Imu() }
  }
}


#[link(name = "dls2_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__TrajectoryGenerator() -> *const std::ffi::c_void;
}

#[link(name = "dls2_interface__rosidl_generator_c")]
extern "C" {
    fn dls2_interface__msg__TrajectoryGenerator__init(msg: *mut TrajectoryGenerator) -> bool;
    fn dls2_interface__msg__TrajectoryGenerator__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryGenerator>, size: usize) -> bool;
    fn dls2_interface__msg__TrajectoryGenerator__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryGenerator>);
    fn dls2_interface__msg__TrajectoryGenerator__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryGenerator>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryGenerator>) -> bool;
}

// Corresponds to dls2_interface__msg__TrajectoryGenerator
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Header

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryGenerator {

    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sequence_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub com_pose: super::super::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub com_vel: super::super::msg::rmw::Screw,


    // This member is not documented.
    #[allow(missing_docs)]
    pub com_acc: super::super::msg::rmw::Screw,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_position: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_velocity: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_acceleration: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joints_effort: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kp: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub kd: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wrench: [f64; 6],


    // This member is not documented.
    #[allow(missing_docs)]
    pub stance_legs: rosidl_runtime_rs::Sequence<bool>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub nominal_touch_down: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub touch_down: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub swing_period: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub normal_force_max: rosidl_runtime_rs::Sequence<f64>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub normal_force_min: rosidl_runtime_rs::Sequence<f64>,

}



impl Default for TrajectoryGenerator {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dls2_interface__msg__TrajectoryGenerator__init(&mut msg as *mut _) {
        panic!("Call to dls2_interface__msg__TrajectoryGenerator__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryGenerator {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__TrajectoryGenerator__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__TrajectoryGenerator__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dls2_interface__msg__TrajectoryGenerator__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryGenerator {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryGenerator where Self: Sized {
  const TYPE_NAME: &'static str = "dls2_interface/msg/TrajectoryGenerator";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dls2_interface__msg__TrajectoryGenerator() }
  }
}


