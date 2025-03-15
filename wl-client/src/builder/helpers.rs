use {
    crate::ffi::{wl_object, wl_proxy},
    std::{
        cell::RefCell,
        ffi::{CStr, c_char},
        ptr::NonNull,
    },
};

#[cfg(test)]
mod tests;

/// Converts a C string pointer to a `&str`.
///
/// # Safety
///
/// If `s` is not null, then it must be a valid C string.
#[inline]
pub unsafe fn convert_string_arg<'a>(interface: &str, arg: &str, s: *const c_char) -> &'a str {
    if s.is_null() {
        null_string(interface, arg);
    }
    // SAFETY: The requirement is forwarded to the caller.
    let cstr = unsafe { CStr::from_ptr(s) };
    match cstr.to_str() {
        Ok(s) => s,
        Err(_) => non_utf8_string(interface, arg),
    }
}

/// Converts a C string pointer, or null, to an `Option<&str>`.
///
/// # Safety
///
/// If `s` is not null, then it must be a valid C string.
#[inline]
pub unsafe fn convert_optional_string_arg<'a>(
    interface: &str,
    arg: &str,
    s: *const c_char,
) -> Option<&'a str> {
    if s.is_null() {
        return None;
    }
    // SAFETY: The requirement is forwarded to the caller.
    unsafe { Some(convert_string_arg(interface, arg, s)) }
}

#[cold]
fn null_string(interface: &str, arg: &str) -> ! {
    unreachable!("string argument {arg} of {interface} is null but should not be");
}

#[cold]
fn non_utf8_string(interface: &str, arg: &str) -> ! {
    unreachable!("string argument {arg} of {interface} is not valid UTF-8");
}

#[cold]
pub fn invalid_opcode(interface: &str, opcode: u32) -> ! {
    unreachable!("interface {interface} has no event with opcode {opcode}");
}

#[cold]
pub fn unimplemented_event_handler(interface: &str, event: &str) -> ! {
    unreachable!("event handler {event} of interface {interface} is not implemented");
}

pub fn with_cstr_cache<T>(f: impl FnOnce(&mut Vec<u8>) -> T) -> T {
    thread_local! {
        static CACHE: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }
    CACHE.with(|cache| {
        let cache = &mut *cache.borrow_mut();
        cache.clear();
        f(cache)
    })
}

#[inline]
pub fn check_argument_proxy(arg: &str, proxy: Option<NonNull<wl_proxy>>) -> *mut wl_object {
    match proxy {
        None => {
            #[cold]
            fn destroyed(arg: &str) -> ! {
                panic!("proxy argument {arg} has already been destroyed");
            }
            destroyed(arg);
        }
        Some(p) => p.as_ptr().cast(),
    }
}
