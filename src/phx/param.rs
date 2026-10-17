use std::ffi::{c_char, c_void};

use crate::{cpp, phx::Hash40};

#[repr(C)]
pub struct ParameterReceiverBase {
    vtable: &'static ParameterReceiverBaseVtable,
}

#[repr(C)]
struct ParameterReceiverBaseVtable {
    /// Releases the resources that may have been acquired.
    pub destroy: extern "C" fn(*mut ParameterReceiverBase),

    /// Deallocates the memory reserved for `this`.
    pub delete: extern "C" fn(*mut ParameterReceiverBase),

    /// Returns the value of the given key wrapped in a [`ParameterMember`].
    pub get_param_member: extern "C" fn(*const ParameterReceiverBase, Hash40) -> ParameterMember,

    /// Allocates memory on the heap and then places a duplicate of `this` into it.
    pub clone: extern "C" fn(*const ParameterReceiverBase) -> cpp::UniquePtr<ParameterReceiverBase>,

    /// Deserializes this object from the given parameter file accessor.
    pub read: extern "C" fn(*mut ParameterReceiverBase, &ParameterResource),

    // TBD
    pub _28: extern "C" fn(*const ParameterReceiverBase) -> u32,
}

#[repr(C)]
pub struct ParameterMember {
    vtable: &'static ParameterMemberVtable,
    pub value: *const c_void,
}

#[repr(C)]
struct ParameterMemberVtable {
    /// Returns `true` if the type of this parameter represents a Boolean or an 8/16/32-bit integer.
    pub is_integer: extern "C" fn(*const ParameterMember) -> bool,

    /// Returns `true` if the type of this parameter represents a 32-bit floating-point number.
    pub is_float: extern "C" fn(*const ParameterMember) -> bool,

    /// Returns `true` if the type of this parameter represents a nul-terminated string.
    pub is_string: extern "C" fn(*const ParameterMember) -> bool,

    /// Returns `true` if the type of this parameter represents a [`Hash40`].
    pub is_hash: extern "C" fn(*const ParameterMember) -> bool,

    /// Returns `true` if the type of this parameter represents a struct.
    pub is_struct: extern "C" fn(*const ParameterMember) -> bool,

    /// Returns `true` if the type of this parameter represents an array.
    pub is_array: extern "C" fn(*const ParameterMember) -> bool,

    /// Returns the underlying value as a 32-bit integer.
    pub get_integer: extern "C" fn(*const ParameterMember) -> i32,

    /// Returns the underlying value as a 32-bit floating-point number.
    pub get_float: extern "C" fn(*const ParameterMember) -> f32,

    /// Returns the underlying value as a raw pointer to a nul-terminated string.
    pub get_string: extern "C" fn(*const ParameterMember) -> *const c_char,

    /// Returns the underlying value as a [`Hash40`].
    pub get_hash: extern "C" fn(*const ParameterMember) -> Hash40,

    /// Returns the value of the given key wrapped in a [`ParameterMember`].
    pub get_struct_member: extern "C" fn(*const ParameterMember, Hash40) -> ParameterMember,

    /// Returns the number of elements in the array.
    pub get_array_size: extern "C" fn(*const ParameterMember) -> usize,

    /// Returns the element at the given index wrapped in a [`ParameterMember`].
    pub get_array_member: extern "C" fn(*const ParameterMember, i32) -> ParameterMember,

    /// Updates the underlying value with the given 32-bit integer.
    pub set_integer: extern "C" fn(*mut ParameterMember, i32),

    /// Updates the underlying value with the given 32-bit floating-point number.
    pub set_float: extern "C" fn(*mut ParameterMember, f32),
}

#[repr(C)]
pub struct ParameterResource {
    pub this: *const Self,
    pub root: *const u8,
    pub resource_handle: u32,
    pub resource_buf: *const u8,
    pub hash_table: *const Hash40,
    pub reference_table: *const ParameterReference,
}

#[repr(C)]
pub struct ParameterReference {
    pub hash_index: u32,
    pub value_offset: u32,
}
