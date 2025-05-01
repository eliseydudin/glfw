use glfw_rs_sys as glfw_sys;
use glfw_sys::{GLFWmonitor, GLFWwindow};
use std::{
    any,
    ffi::{CString, c_void},
    ptr::{self, NonNull},
};

/// A GLFW window
pub struct Window {
    raw: NonNull<GLFWwindow>,
    should_drop: bool,
}

#[doc(hidden)]
#[derive(Clone)]
struct UserData {
    dtype: any::TypeId,
    ptr: *const c_void,
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
        Some(Self {
            raw,
            should_drop: true,
        })
    }

    /// Create a new window with the given name and size. Returns [`None`] if
    /// the inner GLFW call fails
    pub(crate) fn new<S: AsRef<str>>(name: S, size: (i32, i32)) -> Option<Self> {
        Self::new_ex(name, size, ptr::null_mut(), ptr::null_mut())
    }

    /// Create a new fullscreen window with the given name. Returns [`None`] if
    /// the inner GLFW call fails
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

    /// Create a window from a raw pointer. Note that the window created this way will
    /// not be dropped. Use this function in callbacks
    pub unsafe fn from_raw(ptr: *mut GLFWwindow) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(ptr) },
            should_drop: false,
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.should_drop {
            if !self.get_user_data_ptr().is_null() {
                self._deallocate_user_data();
            }

            unsafe { glfw_sys::glfwDestroyWindow(self.raw.as_ptr()) }
        }
    }
}

pub type LoadProc = fn(name: *const i8) -> *const c_void;
pub type SafeLoadProc = fn(name: &str) -> *const c_void;

impl Window {
    /// Make this window and its OpenGL context global. Should be called before you
    /// try to initialize you initialize your OpenGL crate.
    pub fn make_global(&self) {
        unsafe { glfw_sys::glfwMakeContextCurrent(self.raw.as_ptr()) }
    }

    /// Get the loader function for OpenGL
    pub fn get_load_proc(&self) -> LoadProc {
        |name| unsafe {
            match glfw_sys::glfwGetProcAddress(name) {
                Some(ptr) => ptr as *const c_void,
                None => ptr::null_mut(),
            }
        }
    }

    /// Get the loader function for OpenGL. This one is recommended when your
    /// OpenGL loader requires a `fn(&str) -> *const c_void`
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

    /// Swap the buffers of the window
    pub fn update(&self) {
        unsafe { glfw_sys::glfwSwapBuffers(self.raw.as_ptr()) }
    }

    /// Check if the window should close. Use this in a loop to check when the app
    /// should close
    pub fn should_close(&self) -> bool {
        let res = unsafe { glfw_sys::glfwWindowShouldClose(self.raw.as_ptr()) };
        res == 1
    }

    /// Set the function which should be called when a key is pressed
    pub fn set_key_callback(&self, callback: KeyCallback) {
        unsafe { glfw_sys::glfwSetKeyCallback(self.raw.as_ptr(), Some(callback)) };
    }

    /// Set the function which should be called when the mouse moves
    pub fn set_mouse_callback(&self, callback: MouseCallback) {
        unsafe { glfw_sys::glfwSetCursorPosCallback(self.raw.as_ptr(), Some(callback)) };
    }

    /// Set the user data which can be accessed from callbacks.
    pub fn set_user_data<T: any::Any>(&self, data: &T) {
        let ptr = self.get_user_data_ptr();

        if !ptr.is_null() {
            self._deallocate_user_data();
        }
        self._set_user_data(data);
    }

    fn _set_user_data<T: any::Any>(&self, data: &T) {
        let dtype = any::TypeId::of::<T>();
        let ptr = data as *const _ as *const c_void;
        let data = Box::new(UserData { dtype, ptr });

        unsafe {
            glfw_sys::glfwSetWindowUserPointer(
                self.raw.as_ptr(),
                Box::into_raw(data) as *mut c_void,
            )
        }
    }

    fn _deallocate_user_data(&self) {
        let ptr = self.get_user_data_ptr();
        drop(unsafe { Box::from_raw(ptr) })
    }

    pub fn get_user_data_ptr(&self) -> *mut c_void {
        unsafe { glfw_sys::glfwGetWindowUserPointer(self.raw.as_ptr()) }
    }

    pub fn get_user_data<T: any::Any>(&self) -> Option<&T> {
        let ptr = self.get_user_data_ptr() as *mut UserData;
        if ptr.is_null() {
            return None;
        }

        let UserData { dtype, ptr } = unsafe { (*ptr).clone() };
        if any::TypeId::of::<T>() == dtype {
            unsafe { (ptr as *const T).as_ref() }
        } else {
            None
        }
    }
}

/// Window, key, scancode, action, mods
pub type KeyCallback = unsafe extern "C" fn(*mut GLFWwindow, i32, i32, i32, i32);
/// Window, x, y
pub type MouseCallback = unsafe extern "C" fn(*mut GLFWwindow, f64, f64);
