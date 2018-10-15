use std::os::raw::{ c_void, c_char, c_int, c_uint, c_float, c_double };
use std::path::PathBuf;
use std::char::from_u32;
use std::process::abort;
use std::panic;
use std::slice;
use std::ffi::CStr;
use enum_primitive::FromPrimitive;

use PROCESSING_EVENTS;
use Action;
use Key;
use KeyCode;
use MouseButton;
use Modifiers;
use Window;
use ffi;
use cint_to_bool;

pub use self::monitor::Callback as MonitorCallback;
pub(crate) mod monitor {
    use std::panic;
    use std::process::abort;
    use std::os::raw::c_int;

    use PROCESSING_EVENTS;
    use Monitor;
    use DisconnectedMonitor;
    use invalidate_monitor;
    use ffi;

    pub trait Callback {
        fn connected(&mut self, monitor: Monitor);
        fn disconnected(&mut self, monitor: &DisconnectedMonitor);
    }

    static mut CALLBACK: Option<Box<Callback>> = None;

    pub(crate) fn set(cb: Box<Callback>) {
        unsafe {
            CALLBACK = Some(cb);
        }
    }

    pub(crate) fn unset() {
        unsafe {
            CALLBACK = None;
        }
    }

    extern "C" fn callback(monitor: *mut ffi::GLFWmonitor, event: c_int) {
        if let Err(_) = panic::catch_unwind(|| unsafe {
            PROCESSING_EVENTS.with(|v| v.set(true));
            match event {
                ffi::GLFW_CONNECTED => if let Some(ref mut cb) = CALLBACK {
                    cb.connected(Monitor::create_from(monitor));
                }
                ffi::GLFW_DISCONNECTED => {
                    invalidate_monitor(monitor);
                    if let Some(ref mut cb) = CALLBACK {
                        cb.disconnected(&DisconnectedMonitor(monitor));
                    }
                }
                _ => unreachable!()
            }
            PROCESSING_EVENTS.with(|v| v.set(false));
        }) {
            eprintln!("Panic in GLFW monitor callback");
            abort();
        }
    }

    pub(crate) fn initialize() {
        unsafe { ffi::glfwSetMonitorCallback(Some(callback)) };
    }
}

pub use self::joystick::Callback as JoystickCallback;
pub(crate) mod joystick {
    use std::panic;
    use std::process::abort;
    use std::os::raw::c_int;
    use enum_primitive::FromPrimitive;

    use PROCESSING_EVENTS;
    use ffi;
    use Joystick;

    pub trait Callback {
        fn connected(&mut self, joystick: Joystick);
        fn disconnected(&mut self, joystick: Joystick);
    }

    static mut CALLBACK: Option<Box<Callback>> = None;

    pub(crate) fn set(cb: Box<Callback>) {
        unsafe {
            ffi::glfwSetJoystickCallback(Some(callback));
            CALLBACK = Some(cb);
        }
    }

    pub(crate) fn unset() {
        unsafe {
            CALLBACK = None;
            ffi::glfwSetJoystickCallback(None);
        }
    }

    extern "C" fn callback(joystick: c_int, event: c_int) {
        if let Err(_) = panic::catch_unwind(|| unsafe {
            PROCESSING_EVENTS.with(|v| v.set(true));
            match event {
                ffi::GLFW_CONNECTED => if let Some(ref mut cb) = CALLBACK {
                    cb.connected(Joystick::from_i32(joystick).unwrap());
                }
                ffi::GLFW_DISCONNECTED => if let Some(ref mut cb) = CALLBACK {
                    cb.disconnected(Joystick::from_i32(joystick).unwrap());
                }
                _ => unreachable!()
            }
            PROCESSING_EVENTS.with(|v| v.set(false));
        }) {
            eprintln!("Panic in GLFW monitor callback");
            abort();
        }
    }
}

macro_rules! window_callbacks {
    ($($name:ident: $tname:ident {
        args = $($arg_n:ident: $arg_t:ty),*;
        glfw = $glfw:ident;
        transform = $($glfw_n:ident: $glfw_t:ty),* => $($transform:expr),*;
    })*) => {
        pub trait WindowCallbacks {
            $(
                #[allow(unused_variables)]
                fn $name(&mut self, window: &Window $(, $arg_n: $arg_t)*) {}
            )*
        }

        pub(crate) fn init_callbacks(ptr: *mut ::ffi::GLFWwindow) {
            unsafe { $(
                ffi::$glfw(ptr, Some($name));
            )* }
        }

        $(
            extern "C" fn $name(ptr: *mut ffi::GLFWwindow, $($glfw_n: $glfw_t),*) {
                if let Err(_) = panic::catch_unwind(|| unsafe {
                    PROCESSING_EVENTS.with(|v| v.set(true));
                    let callbacks = ffi::glfwGetWindowUserPointer(ptr)
                            as *mut &mut WindowCallbacks;
                    if !callbacks.is_null() {
                        let callbacks = &mut *callbacks;
                        let window_ref = Window::init(None, ptr);
                        callbacks.$name(&window_ref, $($transform),*);
                    }
                    PROCESSING_EVENTS.with(|v| v.set(false));
                }) {
                    eprintln!("Panic in GLFW {} callback", stringify!($name));
                    abort();
                }
            }
        )*
    };
}

window_callbacks! {
    window_pos: WindowPosCallback {
        args = x: i32, y: i32;
        glfw = glfwSetWindowPosCallback;
        transform = x: c_int, y: c_int => x, y;
    }

    window_size: WindowSizeCallback {
        args = width: i32, height: i32;
        glfw = glfwSetWindowSizeCallback;
        transform = x: c_int, y: c_int => x, y;
    }

    window_close: WindowCloseCallback {
        args = ;
        glfw = glfwSetWindowCloseCallback;
        transform = => ;
    }

    window_refresh: WindowRefreshCallback {
        args = ;
        glfw = glfwSetWindowRefreshCallback;
        transform = => ;
    }

    window_focus: WindowFocusCallback {
        args = focused: bool;
        glfw = glfwSetWindowFocusCallback;
        transform = focused: c_int => cint_to_bool(focused);
    }

    window_iconify: WindowIconifyCallback {
        args = iconified: bool;
        glfw = glfwSetWindowIconifyCallback;
        transform = iconified: c_int => cint_to_bool(iconified);
    }

    window_maximize: WindowMaximizeCallback {
        args = maximized: bool;
        glfw = glfwSetWindowMaximizeCallback;
        transform = maximized: c_int => cint_to_bool(maximized);
    }

    framebuffer_size: FramebufferSizeCallback {
        args = width: i32, height: i32;
        glfw = glfwSetFramebufferSizeCallback;
        transform = width: c_int, height: c_int => width, height;
    }

    window_content_scale: WindowContentScaleCallback {
        args = xscale: f32, yscale: f32;
        glfw = glfwSetWindowContentScaleCallback;
        transform = xscale: c_float, yscale: c_float => xscale, yscale;
    }

    mouse_button: MouseButtonCallback {
        args = button: MouseButton, pressed: bool, mods: Modifiers;
        glfw = glfwSetMouseButtonCallback;
        transform = button: c_int, action: c_int, mods: c_int =>
                MouseButton::from_i32(button).unwrap(),
                cint_to_bool(action),
                Modifiers::from_bits(mods).unwrap();
    }

    cursor_pos: CursorPosCallback {
        args = x: f64, y: f64;
        glfw = glfwSetCursorPosCallback;
        transform = x: c_double, y: c_double => x, y;
    }

    cursor_enter: CursorEnterCallback {
        args = entered: bool;
        glfw = glfwSetCursorEnterCallback;
        transform = entered: c_int => cint_to_bool(entered);
    }

    scroll: ScrollCallback {
        args = xoffset: f64, yoffset: f64;
        glfw = glfwSetScrollCallback;
        transform = xoffset: c_double, yoffset: c_double => xoffset, yoffset;
    }

    key: KeyCallback {
        args = key: Key, action: Action, mods: Modifiers;
        glfw = glfwSetKeyCallback;
        transform = key: c_int, scancode: c_int, action: c_int, mods: c_int =>
                KeyCode::from_i32(key).map_or(Key::Unnamed(scancode), |c| Key::Named(c)),
                Action::from_i32(action).unwrap(),
                Modifiers::from_bits(mods).unwrap();
    }

    char: CharCallback {
        args = char: char;
        glfw = glfwSetCharCallback;
        transform = char: c_uint => from_u32(char).unwrap();
    }

    char_mods: CharModsCallback {
        args = char: char, mods: Modifiers;
        glfw = glfwSetCharModsCallback;
        transform = char: c_uint, mods: c_int =>
                from_u32(char).unwrap(),
                Modifiers::from_bits(mods).unwrap();
    }

    file_drop: FileDropCallback {
        args = paths: Vec<PathBuf>;
        glfw = glfwSetDropCallback;
        transform = count: c_int, paths: *const *const c_char => {
            let s = slice::from_raw_parts(paths, count as usize);
            s.iter().map(|p|
                    PathBuf::from(CStr::from_ptr(*p).to_string_lossy().into_owned())
            ).collect()
        };
    }
}

/// This function stands in for all of the `glfwSet*Callback` functions.
pub fn with_callbacks<'b, F, T>(windows: &[&Window], mut callbacks: &mut WindowCallbacks, f: F) -> T
where F: FnOnce() -> T {
    let mut prev = Vec::with_capacity(windows.len());
    let ptr = &mut callbacks as *mut &mut WindowCallbacks;
    for w in windows.iter() {
        prev.push(unsafe { ffi::glfwGetWindowUserPointer(w.ptr) });
        unsafe { ffi::glfwSetWindowUserPointer(w.ptr, ptr as *mut c_void) };
    }
    // Defer to be safe on unwind
    defer! {
        for (w, &p) in windows.iter().zip(prev.iter()) {
            unsafe { ffi::glfwSetWindowUserPointer(w.ptr, p) }
        }
    }
    f()
}