use glfw_sys::{GLFWmonitor, GLFWwindow};
use std::{
    ffi::{CString, c_void},
    ptr::{self, NonNull},
};

pub struct Window {
    raw: NonNull<GLFWwindow>,
}

impl Window {
    fn new_ex<S: AsRef<str>>(
        name: S,
        size: (i32, i32),
        monitor: *mut GLFWmonitor,
        shared: *mut GLFWwindow,
    ) -> Option<Self> {
        let name = CString::new(name.as_ref()).ok()?;
        let handle =
            unsafe { glfw_sys::glfwCreateWindow(size.0, size.1, name.as_ptr(), monitor, shared) };
        let raw = NonNull::new(handle)?;
        Some(Self { raw })
    }

    pub(crate) fn new<S: AsRef<str>>(name: S, size: (i32, i32)) -> Option<Self> {
        Self::new_ex(name, size, ptr::null_mut(), ptr::null_mut())
    }

    pub(crate) fn new_fullscreen<S: AsRef<str>>(name: S) -> Option<Self> {
        let monitor = unsafe { glfw_sys::glfwGetPrimaryMonitor() };
        let vidmode = unsafe { *glfw_sys::glfwGetVideoMode(monitor) };
        Self::new_ex(
            name,
            (vidmode.width, vidmode.height),
            monitor,
            ptr::null_mut(),
        )
    }

    pub fn global() -> Option<Self> {
        Some(Self {
            raw: NonNull::new(unsafe { glfw_sys::glfwGetCurrentContext() })?,
        })
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { glfw_sys::glfwDestroyWindow(self.raw.as_ptr()) }
    }
}

pub type LoadProc = fn(name: *const i8) -> *const c_void;
pub type SafeLoadProc = fn(name: &str) -> *const c_void;

impl Window {
    pub fn make_global(&self) {
        unsafe { glfw_sys::glfwMakeContextCurrent(self.raw.as_ptr()) }
    }

    pub fn get_load_proc(&self) -> LoadProc {
        |name| unsafe {
            match glfw_sys::glfwGetProcAddress(name) {
                Some(ptr) => ptr as *const c_void,
                None => ptr::null_mut(),
            }
        }
    }

    pub fn get_safe_load_proc(&self) -> SafeLoadProc {
        |name| unsafe {
            let name = match CString::new(name) {
                Ok(name) => name,
                Err(_) => return ptr::null_mut(),
            };
            match glfw_sys::glfwGetProcAddress(name.as_ptr()) {
                Some(ptr) => ptr as *const c_void,
                None => ptr::null_mut(),
            }
        }
    }

    pub fn update(&self) {
        unsafe { glfw_sys::glfwSwapBuffers(self.raw.as_ptr()) }
    }

    pub fn should_close(&self) -> bool {
        let res = unsafe { glfw_sys::glfwWindowShouldClose(self.raw.as_ptr()) };
        res == 1
    }
}
