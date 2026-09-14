#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__BmsState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__BmsState__init(msg: *mut BmsState) -> bool;
    fn unitree_go__msg__BmsState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BmsState>, size: usize) -> bool;
    fn unitree_go__msg__BmsState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BmsState>);
    fn unitree_go__msg__BmsState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BmsState>, out_seq: *mut rosidl_runtime_rs::Sequence<BmsState>) -> bool;
}

// Corresponds to unitree_go__msg__BmsState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BmsState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub version_high: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub version_low: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub soc: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub cycle: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bq_ntc: [i8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub mcu_ntc: [i8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub cell_vol: [u16; 15],

}



impl Default for BmsState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__BmsState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__BmsState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BmsState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__BmsState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BmsState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BmsState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/BmsState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__BmsState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__IMUState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__IMUState__init(msg: *mut IMUState) -> bool;
    fn unitree_go__msg__IMUState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IMUState>, size: usize) -> bool;
    fn unitree_go__msg__IMUState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IMUState>);
    fn unitree_go__msg__IMUState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IMUState>, out_seq: *mut rosidl_runtime_rs::Sequence<IMUState>) -> bool;
}

// Corresponds to unitree_go__msg__IMUState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IMUState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub quaternion: [f32; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub gyroscope: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub accelerometer: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub rpy: [f32; 3],


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: i8,

}



impl Default for IMUState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__IMUState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__IMUState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IMUState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__IMUState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__IMUState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__IMUState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IMUState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IMUState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/IMUState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__IMUState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__LowState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__LowState__init(msg: *mut LowState) -> bool;
    fn unitree_go__msg__LowState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LowState>, size: usize) -> bool;
    fn unitree_go__msg__LowState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LowState>);
    fn unitree_go__msg__LowState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LowState>, out_seq: *mut rosidl_runtime_rs::Sequence<LowState>) -> bool;
}

// Corresponds to unitree_go__msg__LowState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LowState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub head: [u8; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub level_flag: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_reserve: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sn: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub version: [u32; 2],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bandwidth: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub imu_state: super::super::msg::rmw::IMUState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state: [super::super::msg::rmw::MotorState; 20],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bms_state: super::super::msg::rmw::BmsState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_force: [i16; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub foot_force_est: [i16; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub tick: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub wireless_remote: [u8; 40],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bit_flag: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub adc_reel: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature_ntc1: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature_ntc2: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub power_v: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub power_a: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fan_frequency: [u16; 4],


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub crc: u32,

}



impl Default for LowState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__LowState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__LowState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LowState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__LowState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LowState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LowState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/LowState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__LowState() }
  }
}


#[link(name = "unitree_go__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorState() -> *const std::ffi::c_void;
}

#[link(name = "unitree_go__rosidl_generator_c")]
extern "C" {
    fn unitree_go__msg__MotorState__init(msg: *mut MotorState) -> bool;
    fn unitree_go__msg__MotorState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorState>, size: usize) -> bool;
    fn unitree_go__msg__MotorState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorState>);
    fn unitree_go__msg__MotorState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorState>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorState>) -> bool;
}

// Corresponds to unitree_go__msg__MotorState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub q: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dq: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ddq: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tau_est: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub q_raw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub dq_raw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ddq_raw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub lost: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reserve: [u32; 2],

}



impl Default for MotorState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !unitree_go__msg__MotorState__init(&mut msg as *mut _) {
        panic!("Call to unitree_go__msg__MotorState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { unitree_go__msg__MotorState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorState where Self: Sized {
  const TYPE_NAME: &'static str = "unitree_go/msg/MotorState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__unitree_go__msg__MotorState() }
  }
}


