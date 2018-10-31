extern crate glfw_wrapper;
extern crate gl;

use glfw_wrapper::*;

fn main() {
    let glfw = glfw_wrapper::init(InitHints::default()).unwrap();

    let window = glfw.create_window(&WindowHints {
        opengl_profile: OpenGlProfile::Core,
        opengl_forward_compatible: true,
        context_version: (3, 2),
        ..WindowHints::default()
    }, 800, 600, "Basic Example", None, None).unwrap();

    unsafe {
        window.make_context_current().unwrap();
        gl::load_with(|s| glfw.get_proc_address(s).unwrap() as *const _);
    }

    let mut col = 0;

    while !window.should_close() {
        unsafe {
            gl::ClearColor(
                (col / 1 % 4) as f32 / 3.0,
                (col / 4 % 4) as f32 / 3.0,
                (col / 16 % 4) as f32 / 3.0,
                1.0
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers().unwrap();
        glfw.poll_events(&mut |e| match e {
            Event::MouseButtonDown {..} => col += 1,
            _ => {}
        }).unwrap();
    }
}