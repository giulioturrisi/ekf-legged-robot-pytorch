#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to unitree_go__msg__BmsState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BmsState::default())
  }
}

impl rosidl_runtime_rs::Message for BmsState {
  type RmwMsg = super::msg::rmw::BmsState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        version_high: msg.version_high,
        version_low: msg.version_low,
        status: msg.status,
        soc: msg.soc,
        current: msg.current,
        cycle: msg.cycle,
        bq_ntc: msg.bq_ntc,
        mcu_ntc: msg.mcu_ntc,
        cell_vol: msg.cell_vol,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      version_high: msg.version_high,
      version_low: msg.version_low,
      status: msg.status,
      soc: msg.soc,
      current: msg.current,
      cycle: msg.cycle,
        bq_ntc: msg.bq_ntc,
        mcu_ntc: msg.mcu_ntc,
        cell_vol: msg.cell_vol,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      version_high: msg.version_high,
      version_low: msg.version_low,
      status: msg.status,
      soc: msg.soc,
      current: msg.current,
      cycle: msg.cycle,
      bq_ntc: msg.bq_ntc,
      mcu_ntc: msg.mcu_ntc,
      cell_vol: msg.cell_vol,
    }
  }
}


// Corresponds to unitree_go__msg__IMUState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::IMUState::default())
  }
}

impl rosidl_runtime_rs::Message for IMUState {
  type RmwMsg = super::msg::rmw::IMUState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        quaternion: msg.quaternion,
        gyroscope: msg.gyroscope,
        accelerometer: msg.accelerometer,
        rpy: msg.rpy,
        temperature: msg.temperature,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        quaternion: msg.quaternion,
        gyroscope: msg.gyroscope,
        accelerometer: msg.accelerometer,
        rpy: msg.rpy,
      temperature: msg.temperature,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      quaternion: msg.quaternion,
      gyroscope: msg.gyroscope,
      accelerometer: msg.accelerometer,
      rpy: msg.rpy,
      temperature: msg.temperature,
    }
  }
}


// Corresponds to unitree_go__msg__LowState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub imu_state: super::msg::IMUState,


    // This member is not documented.
    #[allow(missing_docs)]
    pub motor_state: [super::msg::MotorState; 20],


    // This member is not documented.
    #[allow(missing_docs)]
    pub bms_state: super::msg::BmsState,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LowState::default())
  }
}

impl rosidl_runtime_rs::Message for LowState {
  type RmwMsg = super::msg::rmw::LowState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        head: msg.head,
        level_flag: msg.level_flag,
        frame_reserve: msg.frame_reserve,
        sn: msg.sn,
        version: msg.version,
        bandwidth: msg.bandwidth,
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Owned(msg.imu_state)).into_owned(),
        motor_state: msg.motor_state
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        bms_state: super::msg::BmsState::into_rmw_message(std::borrow::Cow::Owned(msg.bms_state)).into_owned(),
        foot_force: msg.foot_force,
        foot_force_est: msg.foot_force_est,
        tick: msg.tick,
        wireless_remote: msg.wireless_remote,
        bit_flag: msg.bit_flag,
        adc_reel: msg.adc_reel,
        temperature_ntc1: msg.temperature_ntc1,
        temperature_ntc2: msg.temperature_ntc2,
        power_v: msg.power_v,
        power_a: msg.power_a,
        fan_frequency: msg.fan_frequency,
        reserve: msg.reserve,
        crc: msg.crc,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        head: msg.head,
      level_flag: msg.level_flag,
      frame_reserve: msg.frame_reserve,
        sn: msg.sn,
        version: msg.version,
      bandwidth: msg.bandwidth,
        imu_state: super::msg::IMUState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.imu_state)).into_owned(),
        motor_state: msg.motor_state
          .iter()
          .map(|elem| super::msg::MotorState::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
        bms_state: super::msg::BmsState::into_rmw_message(std::borrow::Cow::Borrowed(&msg.bms_state)).into_owned(),
        foot_force: msg.foot_force,
        foot_force_est: msg.foot_force_est,
      tick: msg.tick,
        wireless_remote: msg.wireless_remote,
      bit_flag: msg.bit_flag,
      adc_reel: msg.adc_reel,
      temperature_ntc1: msg.temperature_ntc1,
      temperature_ntc2: msg.temperature_ntc2,
      power_v: msg.power_v,
      power_a: msg.power_a,
        fan_frequency: msg.fan_frequency,
      reserve: msg.reserve,
      crc: msg.crc,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      head: msg.head,
      level_flag: msg.level_flag,
      frame_reserve: msg.frame_reserve,
      sn: msg.sn,
      version: msg.version,
      bandwidth: msg.bandwidth,
      imu_state: super::msg::IMUState::from_rmw_message(msg.imu_state),
      motor_state: msg.motor_state
        .map(super::msg::MotorState::from_rmw_message),
      bms_state: super::msg::BmsState::from_rmw_message(msg.bms_state),
      foot_force: msg.foot_force,
      foot_force_est: msg.foot_force_est,
      tick: msg.tick,
      wireless_remote: msg.wireless_remote,
      bit_flag: msg.bit_flag,
      adc_reel: msg.adc_reel,
      temperature_ntc1: msg.temperature_ntc1,
      temperature_ntc2: msg.temperature_ntc2,
      power_v: msg.power_v,
      power_a: msg.power_a,
      fan_frequency: msg.fan_frequency,
      reserve: msg.reserve,
      crc: msg.crc,
    }
  }
}


// Corresponds to unitree_go__msg__MotorState

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorState::default())
  }
}

impl rosidl_runtime_rs::Message for MotorState {
  type RmwMsg = super::msg::rmw::MotorState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        q: msg.q,
        dq: msg.dq,
        ddq: msg.ddq,
        tau_est: msg.tau_est,
        q_raw: msg.q_raw,
        dq_raw: msg.dq_raw,
        ddq_raw: msg.ddq_raw,
        temperature: msg.temperature,
        lost: msg.lost,
        reserve: msg.reserve,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      ddq: msg.ddq,
      tau_est: msg.tau_est,
      q_raw: msg.q_raw,
      dq_raw: msg.dq_raw,
      ddq_raw: msg.ddq_raw,
      temperature: msg.temperature,
      lost: msg.lost,
        reserve: msg.reserve,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      q: msg.q,
      dq: msg.dq,
      ddq: msg.ddq,
      tau_est: msg.tau_est,
      q_raw: msg.q_raw,
      dq_raw: msg.dq_raw,
      ddq_raw: msg.ddq_raw,
      temperature: msg.temperature,
      lost: msg.lost,
      reserve: msg.reserve,
    }
  }
}


