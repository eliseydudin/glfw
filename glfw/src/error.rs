use core::mem;
use std::ffi::CStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Error {
    NotInitialized = glfw_sys::GLFW_NOT_INITIALIZED as i32,
    NoCurrentContext = glfw_sys::GLFW_NO_CURRENT_CONTEXT as i32,
    InvalidEnum = glfw_sys::GLFW_INVALID_ENUM as i32,
    InvalidValue = glfw_sys::GLFW_INVALID_VALUE as i32,
    OutOfMemory = glfw_sys::GLFW_OUT_OF_MEMORY as i32,
    ApiUnavailable = glfw_sys::GLFW_API_UNAVAILABLE as i32,
    VersionUnavailable = glfw_sys::GLFW_VERSION_UNAVAILABLE as i32,
    PlatformError = glfw_sys::GLFW_PLATFORM_ERROR as i32,
    FormatUnavailable = glfw_sys::GLFW_FORMAT_UNAVAILABLE as i32,
}

pub type ErrorHandler = unsafe extern "C" fn(i32, *const i8);

/// Parse function parameteres received by [`ErrorHandler`]
/// # SAFETY
/// The caller must ensure that `code` could be mapped to [`Error`] and description is
/// a pointer to a valid C string
pub unsafe fn parse_params(code: i32, description: *const i8) -> (Error, String) {
    let error = unsafe { mem::transmute(code) };
    let description = unsafe { CStr::from_ptr(description) };
    (error, description.to_string_lossy().to_string())
}
