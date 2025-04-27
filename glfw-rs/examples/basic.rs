use glfw_rs::Context;

fn main() {
    let ctx = Context::init();
    ctx.gl_version(3, 3);
    let window = ctx
        .window("Hello world", (640, 480))
        .expect("Cannot create the window!");
    window.make_global();

    gl::load_with(window.get_safe_load_proc());
    unsafe { gl::ClearColor(0.1, 0.5, 1.0, 1.0) };

    while !window.should_close() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        window.update();
        ctx.poll_events();
    }
}
