use std::os::raw::c_void;
use std::path::PathBuf;

use Action;
use Key;
use MouseButton;
use Modifiers;
use Window;
use ffi;

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
        use { $($s:path),* } in $($glfw_n:ident: $glfw_t:ty),* => $($transform:expr),*;
    })*) => {
        pub trait WindowCallbacks {
            $(
                #[allow(unused_variables)]
                fn $name(&mut self, window: &Window $(, $arg_n: $arg_t)*) {}
            )*
        }

        pub(crate) fn init_callbacks(ptr: *mut ::ffi::GLFWwindow) {
            $($name::set(ptr);)*
        }

        $(
            pub(crate) mod $name {
                use std::panic;
                use std::process::abort;
                use ffi;
                use PROCESSING_EVENTS;
                use WindowCallbacks;
                use Window;

                $(use $s;)*

                pub(crate) fn set(ptr: *mut ffi::GLFWwindow) {
                    unsafe {
                        ffi::$glfw(ptr, Some(callback));
                    }
                }

                extern "C" fn callback(ptr: *mut ffi::GLFWwindow, $($glfw_n: $glfw_t),*) {
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
            }
        )*
    };
}

window_callbacks! {
    window_pos: WindowPosCallback {
        args = x: i32, y: i32;
        glfw = glfwSetWindowPosCallback;
        use {
            std::os::raw::c_int
        } in x: c_int, y: c_int => x, y;
    }

    window_size: WindowSizeCallback {
        args = width: i32, height: i32;
        glfw = glfwSetWindowSizeCallback;
        use {
            std::os::raw::c_int
        } in  x: c_int, y: c_int => x, y;
    }

    window_close: WindowCloseCallback {
        args = ;
        glfw = glfwSetWindowCloseCallback;
        use {} in => ;
    }

    window_refresh: WindowRefreshCallback {
        args = ;
        glfw = glfwSetWindowRefreshCallback;
        use {} in => ;
    }

    window_focus: WindowFocusCallback {
        args = focused: bool;
        glfw = glfwSetWindowFocusCallback;
        use {
            std::os::raw::c_int, cint_to_bool
        } in focused: c_int => cint_to_bool(focused);
    }

    window_iconify: WindowIconifyCallback {
        args = iconified: bool;
        glfw = glfwSetWindowIconifyCallback;
        use {
            std::os::raw::c_int, cint_to_bool
        } in iconified: c_int => cint_to_bool(iconified);
    }

    window_maximize: WindowMaximizeCallback {
        args = maximized: bool;
        glfw = glfwSetWindowMaximizeCallback;
        use {
            std::os::raw::c_int, cint_to_bool
        } in maximized: c_int => cint_to_bool(maximized);
    }

    framebuffer_size: FramebufferSizeCallback {
        args = width: i32, height: i32;
        glfw = glfwSetFramebufferSizeCallback;
        use {
            std::os::raw::c_int
        } in width: c_int, height: c_int => width, height;
    }

    window_content_scale: WindowContentScaleCallback {
        args = xscale: f32, yscale: f32;
        glfw = glfwSetWindowContentScaleCallback;
        use {
            std::os::raw::c_float
        } in xscale: c_float, yscale: c_float => xscale, yscale;
    }

    mouse_button: MouseButtonCallback {
        args = button: MouseButton, pressed: bool, mods: Modifiers;
        glfw = glfwSetMouseButtonCallback;
        use {
            std::os::raw::c_int, MouseButton, Modifiers, cint_to_bool, enum_primitive::FromPrimitive
        } in button: c_int, action: c_int, mods: c_int =>
                MouseButton::from_i32(button).unwrap(),
                cint_to_bool(action),
                Modifiers::from_bits(mods).unwrap();
    }

    cursor_pos: CursorPosCallback {
        args = x: f64, y: f64;
        glfw = glfwSetCursorPosCallback;
        use {
            std::os::raw::c_double
        } in x: c_double, y: c_double => x, y;
    }

    cursor_enter: CursorEnterCallback {
        args = entered: bool;
        glfw = glfwSetCursorEnterCallback;
        use {
            std::os::raw::c_int, cint_to_bool
        } in entered: c_int => cint_to_bool(entered);
    }

    scroll: ScrollCallback {
        args = xoffset: f64, yoffset: f64;
        glfw = glfwSetScrollCallback;
        use {
            std::os::raw::c_double
        } in xoffset: c_double, yoffset: c_double => xoffset, yoffset;
    }

    key: KeyCallback {
        args = key: Key, action: Action, mods: Modifiers;
        glfw = glfwSetKeyCallback;
        use {
            std::os::raw::c_int, Key, KeyCode, Action, Modifiers, enum_primitive::FromPrimitive
        } in key: c_int, scancode: c_int, action: c_int, mods: c_int =>
                KeyCode::from_i32(key).map_or(Key::Unknown(scancode), |c| Key::Known(c)),
                Action::from_i32(action).unwrap(),
                Modifiers::from_bits(mods).unwrap();
    }

    char: CharCallback {
        args = char: char;
        glfw = glfwSetCharCallback;
        use {
            std::os::raw::c_uint, std::char::from_u32
        } in char: c_uint => from_u32(char).unwrap();
    }

    char_mods: CharModsCallback {
        args = char: char, mods: Modifiers;
        glfw = glfwSetCharModsCallback;
        use {
            std::os::raw::c_int, std::os::raw::c_uint, std::char::from_u32, Modifiers
        } in char: c_uint, mods: c_int =>
                from_u32(char).unwrap(),
                Modifiers::from_bits(mods).unwrap();
    }

    file_drop: FileDropCallback {
        args = paths: Vec<PathBuf>;
        glfw = glfwSetDropCallback;
        use {
            std::os::raw::c_int, std::os::raw::c_char, std::slice, std::path::PathBuf,
            std::ffi::CStr
        } in count: c_int, paths: *const *const c_char => {
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