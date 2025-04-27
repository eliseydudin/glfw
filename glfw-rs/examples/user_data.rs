use glfw_rs::{Context, Window};
use glfw_rs_sys::GLFWwindow;

unsafe extern "C" fn mouse_callback(window: *mut GLFWwindow, x: f64, y: f64) {
    let window = unsafe { Window::from_raw(window) };
    let int: Option<&i32> = window.get_user_data();
    println!("user data: {int:?}, pos: ({x}, {y})")
}

fn main() {
    let ctx = Context::init();
    ctx.gl_version(3, 3);
    let window = ctx
        .window("User data", (640, 480))
        .expect("Cannot create the window!");
    window.make_global();

    let user_data = 10.0;
    window.set_user_data(&user_data);
    window.set_mouse_callback(mouse_callback);

    let user_data: Option<&f64> = window.get_user_data();
    println!("{:?}", user_data);

    gl::load_with(window.get_safe_load_proc());
    unsafe { gl::ClearColor(0.1, 0.5, 1.0, 1.0) };

    while !window.should_close() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        window.update();
        ctx.poll_events();
    }
}
