use crate::{ErrorHandler, Window};
use glfw_rs_sys as glfw_sys;
use std::{marker::PhantomData, sync::OnceLock};

static CONTEXT: OnceLock<Context> = OnceLock::new();

/// The GLFW context, can only be constructed once.
pub struct Context {
    hidden: PhantomData<()>,
}

impl Context {
    /// Initialize the context, or just get it if its already initialized
    pub fn init() -> &'static Context {
        CONTEXT.get_or_init(|| {
            unsafe { glfw_sys::glfwInit() };
            Self {
                hidden: PhantomData,
            }
        })
    }

    /// Get the context, returns [`None`] if it hasn't been initialized.
    pub fn get() -> Option<&'static Context> {
        CONTEXT.get()
    }
}

impl Context {
    /// Poll the events and run handleres registered in the context.
    pub fn poll_events(&self) {
        unsafe { glfw_sys::glfwPollEvents() }
    }

    /// Wait for events to happen
    pub fn wait_events(&self) {
        unsafe { glfw_sys::glfwWaitEvents() }
    }

    /// Set the amount of frames to wait to rerender
    pub fn set_swap_interval(&self, interval: i32) {
        unsafe { glfw_sys::glfwSwapInterval(interval) }
    }

    /// Set the error handler for the current context. You can use [`parse_params`]
    /// to parse the arguments from [`ErrorHandler`]
    ///
    /// [parse_params]: crate::parse_params
    pub fn set_handler(&self, handler: ErrorHandler) {
        unsafe { glfw_sys::glfwSetErrorCallback(Some(handler)) };
    }

    /// Create a window inside the current context
    pub fn window<S: AsRef<str>>(&self, name: S, size: (i32, i32)) -> Option<Window> {
        Window::new(name, size)
    }

    /// Create a fullscreen window inside the current context
    pub fn window_fullscreen<S: AsRef<str>>(&self, name: S) -> Option<Window> {
        Window::new_fullscreen(name)
    }

    /// Set the version of the OpenGL context. Should be called before creating a window.
    pub fn gl_version(&self, major: i32, minor: i32) {
        unsafe {
            glfw_sys::glfwWindowHint(glfw_sys::GLFW_CONTEXT_VERSION_MAJOR as i32, major);
            glfw_sys::glfwWindowHint(glfw_sys::GLFW_CONTEXT_VERSION_MINOR as i32, minor);
            glfw_sys::glfwWindowHint(
                glfw_sys::GLFW_OPENGL_PROFILE as i32,
                glfw_sys::GLFW_OPENGL_CORE_PROFILE as i32,
            );
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { glfw_sys::glfwTerminate() }
    }
}

impl Context {
    pub fn get_time(&self) -> f64 {
        unsafe { glfw_sys::glfwGetTime() }
    }

    pub fn set_time(&self, time: f64) {
        unsafe { glfw_sys::glfwSetTime(time) }
    }
}
