// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from dls2_interface:msg/BaseStateDebug.idl
// generated code does not contain a copyright notice

#ifndef DLS2_INTERFACE__MSG__DETAIL__BASE_STATE_DEBUG__STRUCT_HPP_
#define DLS2_INTERFACE__MSG__DETAIL__BASE_STATE_DEBUG__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__dls2_interface__msg__BaseStateDebug __attribute__((deprecated))
#else
# define DEPRECATED__dls2_interface__msg__BaseStateDebug __declspec(deprecated)
#endif

namespace dls2_interface
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct BaseStateDebug_
{
  using Type = BaseStateDebug_<ContainerAllocator>;

  explicit BaseStateDebug_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->frame_id = "";
      this->sequence_id = 0ul;
      this->timestamp = 0.0;
      this->robot_name = "";
      std::fill<typename std::array<float, 3>::iterator, float>(this->linear_velocity_base.begin(), this->linear_velocity_base.end(), 0.0f);
    }
  }

  explicit BaseStateDebug_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : frame_id(_alloc),
    robot_name(_alloc),
    linear_velocity_base(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->frame_id = "";
      this->sequence_id = 0ul;
      this->timestamp = 0.0;
      this->robot_name = "";
      std::fill<typename std::array<float, 3>::iterator, float>(this->linear_velocity_base.begin(), this->linear_velocity_base.end(), 0.0f);
    }
  }

  // field types and members
  using _frame_id_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _frame_id_type frame_id;
  using _sequence_id_type =
    uint32_t;
  _sequence_id_type sequence_id;
  using _timestamp_type =
    double;
  _timestamp_type timestamp;
  using _robot_name_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _robot_name_type robot_name;
  using _linear_velocity_base_type =
    std::array<float, 3>;
  _linear_velocity_base_type linear_velocity_base;

  // setters for named parameter idiom
  Type & set__frame_id(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->frame_id = _arg;
    return *this;
  }
  Type & set__sequence_id(
    const uint32_t & _arg)
  {
    this->sequence_id = _arg;
    return *this;
  }
  Type & set__timestamp(
    const double & _arg)
  {
    this->timestamp = _arg;
    return *this;
  }
  Type & set__robot_name(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->robot_name = _arg;
    return *this;
  }
  Type & set__linear_velocity_base(
    const std::array<float, 3> & _arg)
  {
    this->linear_velocity_base = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    dls2_interface::msg::BaseStateDebug_<ContainerAllocator> *;
  using ConstRawPtr =
    const dls2_interface::msg::BaseStateDebug_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dls2_interface::msg::BaseStateDebug_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dls2_interface::msg::BaseStateDebug_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dls2_interface::msg::BaseStateDebug_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dls2_interface::msg::BaseStateDebug_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dls2_interface::msg::BaseStateDebug_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dls2_interface::msg::BaseStateDebug_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dls2_interface::msg::BaseStateDebug_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dls2_interface::msg::BaseStateDebug_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dls2_interface__msg__BaseStateDebug
    std::shared_ptr<dls2_interface::msg::BaseStateDebug_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dls2_interface__msg__BaseStateDebug
    std::shared_ptr<dls2_interface::msg::BaseStateDebug_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const BaseStateDebug_ & other) const
  {
    if (this->frame_id != other.frame_id) {
      return false;
    }
    if (this->sequence_id != other.sequence_id) {
      return false;
    }
    if (this->timestamp != other.timestamp) {
      return false;
    }
    if (this->robot_name != other.robot_name) {
      return false;
    }
    if (this->linear_velocity_base != other.linear_velocity_base) {
      return false;
    }
    return true;
  }
  bool operator!=(const BaseStateDebug_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct BaseStateDebug_

// alias to use template instance with default allocator
using BaseStateDebug =
  dls2_interface::msg::BaseStateDebug_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace dls2_interface

#endif  // DLS2_INTERFACE__MSG__DETAIL__BASE_STATE_DEBUG__STRUCT_HPP_
