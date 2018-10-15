extern crate glfw_wrapper;
extern crate gl;

use glfw_wrapper::*;

fn main() {
    let glfw = glfw_wrapper::init(InitHints::default()).unwrap();

    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfile::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(2));

    let window = glfw.create_window(800, 600, "Basic Example", None, None).unwrap();

    unsafe {
        window.make_context_current().unwrap();
        gl::load_with(|s| glfw.get_proc_address(s).unwrap() as *const _);
    }

    let mut col = ColorChanger(0);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(
                (col.0 / 1 % 4) as f32 / 3.0,
                (col.0 / 4 % 4) as f32 / 3.0,
                (col.0 / 16 % 4) as f32 / 3.0,
                1.0
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers().unwrap();
        with_callbacks(&[&window], &mut col, || glfw.poll_events().unwrap());
    }
}

struct ColorChanger(i32);

impl WindowCallbacks for ColorChanger {
    fn mouse_button(&mut self, _: &Window, _: MouseButton, action: bool, _: Modifiers) {
        if action {
            self.0 += 1;
        }
    }
}