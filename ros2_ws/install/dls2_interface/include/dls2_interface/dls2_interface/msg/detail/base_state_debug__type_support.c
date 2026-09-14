// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from dls2_interface:msg/BaseStateDebug.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "dls2_interface/msg/detail/base_state_debug__rosidl_typesupport_introspection_c.h"
#include "dls2_interface/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "dls2_interface/msg/detail/base_state_debug__functions.h"
#include "dls2_interface/msg/detail/base_state_debug__struct.h"


// Include directives for member types
// Member `frame_id`
// Member `robot_name`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  dls2_interface__msg__BaseStateDebug__init(message_memory);
}

void dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_fini_function(void * message_memory)
{
  dls2_interface__msg__BaseStateDebug__fini(message_memory);
}

size_t dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__size_function__BaseStateDebug__linear_velocity_base(
  const void * untyped_member)
{
  (void)untyped_member;
  return 3;
}

const void * dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__get_const_function__BaseStateDebug__linear_velocity_base(
  const void * untyped_member, size_t index)
{
  const float * member =
    (const float *)(untyped_member);
  return &member[index];
}

void * dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__get_function__BaseStateDebug__linear_velocity_base(
  void * untyped_member, size_t index)
{
  float * member =
    (float *)(untyped_member);
  return &member[index];
}

void dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__fetch_function__BaseStateDebug__linear_velocity_base(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const float * item =
    ((const float *)
    dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__get_const_function__BaseStateDebug__linear_velocity_base(untyped_member, index));
  float * value =
    (float *)(untyped_value);
  *value = *item;
}

void dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__assign_function__BaseStateDebug__linear_velocity_base(
  void * untyped_member, size_t index, const void * untyped_value)
{
  float * item =
    ((float *)
    dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__get_function__BaseStateDebug__linear_velocity_base(untyped_member, index));
  const float * value =
    (const float *)(untyped_value);
  *item = *value;
}

static rosidl_typesupport_introspection_c__MessageMember dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_message_member_array[5] = {
  {
    "frame_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(dls2_interface__msg__BaseStateDebug, frame_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "sequence_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT32,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(dls2_interface__msg__BaseStateDebug, sequence_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "timestamp",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_DOUBLE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(dls2_interface__msg__BaseStateDebug, timestamp),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "robot_name",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(dls2_interface__msg__BaseStateDebug, robot_name),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "linear_velocity_base",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    true,  // is array
    3,  // array size
    false,  // is upper bound
    offsetof(dls2_interface__msg__BaseStateDebug, linear_velocity_base),  // bytes offset in struct
    NULL,  // default value
    dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__size_function__BaseStateDebug__linear_velocity_base,  // size() function pointer
    dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__get_const_function__BaseStateDebug__linear_velocity_base,  // get_const(index) function pointer
    dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__get_function__BaseStateDebug__linear_velocity_base,  // get(index) function pointer
    dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__fetch_function__BaseStateDebug__linear_velocity_base,  // fetch(index, &value) function pointer
    dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__assign_function__BaseStateDebug__linear_velocity_base,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_message_members = {
  "dls2_interface__msg",  // message namespace
  "BaseStateDebug",  // message name
  5,  // number of fields
  sizeof(dls2_interface__msg__BaseStateDebug),
  dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_message_member_array,  // message members
  dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_init_function,  // function to initialize message memory (memory has to be allocated)
  dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_message_type_support_handle = {
  0,
  &dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_dls2_interface
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, dls2_interface, msg, BaseStateDebug)() {
  if (!dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_message_type_support_handle.typesupport_identifier) {
    dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &dls2_interface__msg__BaseStateDebug__rosidl_typesupport_introspection_c__BaseStateDebug_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
