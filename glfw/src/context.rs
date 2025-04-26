use std::{marker::PhantomData, sync::OnceLock};

use crate::{ErrorHandler, Window};

static CONTEXT: OnceLock<Context> = OnceLock::new();

// The GLFW context, can only be constructed once
pub struct Context {
    hidden: PhantomData<()>,
}

impl Context {
    pub fn init() -> &'static Context {
        CONTEXT.get_or_init(|| {
            unsafe { glfw_sys::glfwInit() };
            Self {
                hidden: PhantomData,
            }
        })
    }

    pub fn get() -> Option<&'static Context> {
        CONTEXT.get()
    }
}

impl Context {
    pub fn poll_events(&self) {
        unsafe { glfw_sys::glfwPollEvents() }
    }

    pub fn set_handler(&self, handler: ErrorHandler) {
        unsafe { glfw_sys::glfwSetErrorCallback(Some(handler)) };
    }

    pub fn window<S: AsRef<str>>(&self, name: S, size: (i32, i32)) -> Option<Window> {
        Window::new(name, size)
    }

    pub fn window_fullscreen<S: AsRef<str>>(&self, name: S) -> Option<Window> {
        Window::new_fullscreen(name)
    }

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
