use glfw_rs::{Context, Window};
use glfw_rs_sys::GLFWwindow;

unsafe extern "C" fn mouse_callback(window: *mut GLFWwindow, x: f64, y: f64) {
    let window = unsafe { Window::from_raw(window) };
    println!("Mouse position: ({x}, {y})");
    println!("User data: {:?}", unsafe { window.get_data::<i32>() });
}

fn main() {
    let ctx = Context::init();
    ctx.gl_version(3, 3);
    let window = ctx
        .window("User data", (640, 480))
        .expect("Cannot create the window!");
    window.make_global();

    window.set_data(20);
    let ptr: Option<&i32> = unsafe { window.get_data() };
    println!("{ptr:?}");
    window.set_mouse_callback(mouse_callback);

    gl::load_with(window.get_safe_load_proc());
    unsafe { gl::ClearColor(0.1, 0.5, 1.0, 1.0) };

    while !window.should_close() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        window.update();
        ctx.poll_events();
    }
}
