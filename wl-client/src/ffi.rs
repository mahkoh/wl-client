//! libwayland FFI interface.
//!
//! This module contains the type definitions necessary to interact with libwayland.

#![expect(non_camel_case_types)]

use {
    isnt::std_1::primitive::IsntConstPtrExt,
    std::{
        ffi::{CStr, c_char, c_int, c_void},
        ptr,
    },
};

#[cfg(test)]
mod tests;

/// The libwayland `wl_message` type.
#[repr(C)]
pub struct wl_message {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub types: *const *const wl_interface,
}

// SAFETY: pointers not implementing Sync is only a lint
unsafe impl Sync for wl_message {}
// SAFETY: pointers not implementing Send is only a lint
unsafe impl Send for wl_message {}

/// The libwayland `wl_interface` type.
#[repr(C)]
pub struct wl_interface {
    pub name: *const c_char,
    pub version: c_int,
    pub method_count: c_int,
    pub methods: *const wl_message,
    pub event_count: c_int,
    pub events: *const wl_message,
}

// SAFETY: pointers not implementing Send is only a lint
unsafe impl Send for wl_interface {}
// SAFETY: pointers not implementing Sync is only a lint
unsafe impl Sync for wl_interface {}

/// The libwayland `wl_array` type.
#[repr(C)]
pub struct wl_array {
    pub size: usize,
    pub alloc: usize,
    pub data: *mut c_void,
}

// SAFETY: pointers not implementing Send is only a lint
unsafe impl Send for wl_array {}
// SAFETY: pointers not implementing Sync is only a lint
unsafe impl Sync for wl_array {}

/// The libwayland `wl_fixed_t` type.
pub type wl_fixed_t = i32;

pub(crate) type wl_dispatcher_func_t = unsafe extern "C" fn(
    user_data: *const c_void,
    target: *mut c_void,
    opcode: u32,
    msg: *const wl_message,
    args: *mut wl_argument,
) -> c_int;

/// The libwayland `wl_object` type.
///
/// This is always a `wl_proxy` in the context of this crate.
#[repr(C)]
pub struct wl_object(());

/// The libwayland `wl_argument` type.
#[derive(Copy, Clone)]
#[repr(C)]
pub union wl_argument {
    pub i: i32,
    pub u: u32,
    pub f: wl_fixed_t,
    pub s: *const c_char,
    pub o: *mut wl_object,
    pub n: u32,
    pub a: *mut wl_array,
    pub h: i32,
}

// SAFETY: pointers not implementing Send is only a lint
unsafe impl Send for wl_argument {}
// SAFETY: pointers not implementing Sync is only a lint
unsafe impl Sync for wl_argument {}

/// The libwayland `wl_display` type.
///
/// This type morally has the following fields:
///
/// - `mutex: a mutex`
#[repr(C)]
pub struct wl_display(());

/// The libwayland `wl_proxy` type.
///
/// This type morally has the following fields:
///
/// - `interface: *const wl_interface`
/// - `display: *mut wl_display`,
/// - `queue: *mut wl_event_queue`,
/// - `user_data: *mut c_void`,
/// - `dispatcher: *mut c_void`,
/// - `dispatcher_data: *mut c_void`,
///
/// The `interface` and `display` fields are immutable.
///
/// The `queue` field is mutable and access is protected by the `wl_display.mutex`.
///
/// The `user_data`, `dispatcher`, and `dispatcher_data` fields are mutable and access is
/// not synchronized.
#[repr(C)]
pub struct wl_proxy(());

/// The libwayland `wl_event_display` type.
///
/// This type morally has the following fields:
///
/// - `events: Vec<..>`
///
/// where each event is an event that has been read from the socket but which has not yet
/// been dispatched.
///
/// Access to `events` is protected by the `wl_display.mutex`.
///
/// The mutex is dropped before an event is dispatched.
#[repr(C)]
pub struct wl_event_queue(());

/// Atomically destroys a proxy while sending a request.
///
/// If this is passed into wl_proxy_marshal_array_flags, the message is sent and the proxy
/// is destroyed without dropping the display mutex.
///
/// This is required when sending a destructor message for a server-created object. Since
/// otherwise the server might send a new object with the same ID before we have a chance
/// to call wl_proxy_destroy. This would cause a fatal error in libwayland.
pub(crate) const WL_MARSHAL_FLAG_DESTROY: u32 = 1 << 0;

/// # Safety
///
/// `l` and `r` must be valid `wl_interface` definitions.
pub(crate) unsafe fn interface_compatible(l: &wl_interface, r: &wl_interface) -> bool {
    if ptr::eq(l, r) {
        return true;
    }
    macro_rules! cmp_sig {
        ($count:ident, $messages:ident) => {
            if l.$count != r.$count {
                return false;
            }
            for i in 0..l.$count as usize {
                // SAFETY: This function requires that l and r are valid interface definitions.
                let l_msg = unsafe { &*l.$messages.add(i) };
                // SAFETY: Dito
                let r_msg = unsafe { &*r.$messages.add(i) };
                // SAFETY: Dito
                let l_sig = unsafe { CStr::from_ptr(l_msg.signature) };
                // SAFETY: Dito
                let r_sig = unsafe { CStr::from_ptr(r_msg.signature) };
                if l_sig != r_sig {
                    return false;
                }
            }
        };
    }
    cmp_sig!(method_count, methods);
    cmp_sig!(event_count, events);
    for i in 0..l.event_count as usize {
        // SAFETY: This function requires that l and r are valid interface definitions.
        let l_msg = unsafe { &*l.events.add(i) };
        // SAFETY: Dito
        let r_msg = unsafe { &*r.events.add(i) };
        // SAFETY: Dito
        let sig = unsafe { CStr::from_ptr(l_msg.signature) };
        let mut idx = 0;
        for &b in sig.to_bytes() {
            if b == b'?' {
                continue;
            }
            if b == b'o' || b == b'n' {
                // SAFETY: Dito
                let l_if = unsafe { *l_msg.types.add(idx as usize) };
                // SAFETY: Dito
                let r_if = unsafe { *r_msg.types.add(idx as usize) };
                if l_if.is_null() != r_if.is_null() {
                    return false;
                }
                if l_if.is_not_null() {
                    // SAFETY: Dito
                    let l_if = unsafe { &*l_if };
                    // SAFETY: Dito
                    let r_if = unsafe { &*r_if };
                    // SAFETY: Dito
                    if unsafe { !interface_compatible(l_if, r_if) } {
                        return false;
                    }
                }
            }
            idx += 1;
        }
    }
    true
}
