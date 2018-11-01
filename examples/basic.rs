extern crate glfw_wrapper;
extern crate gl;

use glfw_wrapper::{ InitHints, WindowHints };

fn main() {
    let glfw = glfw_wrapper::init(InitHints::default()).unwrap();

    let window = glfw.create_window(
        &WindowHints::default(),
        800, 600,
        "Basic Example",
        None, None
    ).unwrap();

    unsafe {
        window.make_context_current().unwrap();
        gl::load_with(|s| glfw.get_proc_address(s).unwrap() as *const _);
        gl::ClearColor(0.0, 0.25, 0.5, 1.0);
    }

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers().unwrap();
        glfw.poll_events(&mut |_| {}).unwrap();
    }
}