// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dls2_interface:msg/BaseStateDebug.idl
// generated code does not contain a copyright notice

#ifndef DLS2_INTERFACE__MSG__DETAIL__BASE_STATE_DEBUG__BUILDER_HPP_
#define DLS2_INTERFACE__MSG__DETAIL__BASE_STATE_DEBUG__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dls2_interface/msg/detail/base_state_debug__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dls2_interface
{

namespace msg
{

namespace builder
{

class Init_BaseStateDebug_linear_velocity_base
{
public:
  explicit Init_BaseStateDebug_linear_velocity_base(::dls2_interface::msg::BaseStateDebug & msg)
  : msg_(msg)
  {}
  ::dls2_interface::msg::BaseStateDebug linear_velocity_base(::dls2_interface::msg::BaseStateDebug::_linear_velocity_base_type arg)
  {
    msg_.linear_velocity_base = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dls2_interface::msg::BaseStateDebug msg_;
};

class Init_BaseStateDebug_robot_name
{
public:
  explicit Init_BaseStateDebug_robot_name(::dls2_interface::msg::BaseStateDebug & msg)
  : msg_(msg)
  {}
  Init_BaseStateDebug_linear_velocity_base robot_name(::dls2_interface::msg::BaseStateDebug::_robot_name_type arg)
  {
    msg_.robot_name = std::move(arg);
    return Init_BaseStateDebug_linear_velocity_base(msg_);
  }

private:
  ::dls2_interface::msg::BaseStateDebug msg_;
};

class Init_BaseStateDebug_timestamp
{
public:
  explicit Init_BaseStateDebug_timestamp(::dls2_interface::msg::BaseStateDebug & msg)
  : msg_(msg)
  {}
  Init_BaseStateDebug_robot_name timestamp(::dls2_interface::msg::BaseStateDebug::_timestamp_type arg)
  {
    msg_.timestamp = std::move(arg);
    return Init_BaseStateDebug_robot_name(msg_);
  }

private:
  ::dls2_interface::msg::BaseStateDebug msg_;
};

class Init_BaseStateDebug_sequence_id
{
public:
  explicit Init_BaseStateDebug_sequence_id(::dls2_interface::msg::BaseStateDebug & msg)
  : msg_(msg)
  {}
  Init_BaseStateDebug_timestamp sequence_id(::dls2_interface::msg::BaseStateDebug::_sequence_id_type arg)
  {
    msg_.sequence_id = std::move(arg);
    return Init_BaseStateDebug_timestamp(msg_);
  }

private:
  ::dls2_interface::msg::BaseStateDebug msg_;
};

class Init_BaseStateDebug_frame_id
{
public:
  Init_BaseStateDebug_frame_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_BaseStateDebug_sequence_id frame_id(::dls2_interface::msg::BaseStateDebug::_frame_id_type arg)
  {
    msg_.frame_id = std::move(arg);
    return Init_BaseStateDebug_sequence_id(msg_);
  }

private:
  ::dls2_interface::msg::BaseStateDebug msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::dls2_interface::msg::BaseStateDebug>()
{
  return dls2_interface::msg::builder::Init_BaseStateDebug_frame_id();
}

}  // namespace dls2_interface

#endif  // DLS2_INTERFACE__MSG__DETAIL__BASE_STATE_DEBUG__BUILDER_HPP_
