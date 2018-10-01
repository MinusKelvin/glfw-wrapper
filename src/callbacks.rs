pub use self::monitor::Callback as MonitorCallback;
pub(crate) mod monitor {
    use std::panic;
    use std::process::abort;
    use libc::c_int;

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
    use libc::c_int;
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

macro_rules! window_callback {
    (
        $name:ident
        args = $($arg_n:ident: $arg_t:ty),*;
        glfw = $glfw:ident;
        transform = $($glfw_n:ident: $glfw_t:ty),* => $($transform:expr),*;
    ) => {
        use std::panic;
        use std::process::abort;
        use libc::c_void;

        use ffi;
        use PROCESSING_EVENTS;
        use callbacks::WindowCallbacks;

        pub struct Owner<'a, U>(pub Option<Box<Callback<U> + 'a>>);

        impl<'a, U> Owner<'a, U> {
            pub fn set<F: Callback<U> + 'a>(&mut self, callback: F) {
                self.0 = Some(Box::new(callback));
            }

            pub fn unset(&mut self) {
                self.0 = None;
            }
        }

        pub trait Callback<U> {
            fn callback(&mut self, userdata: &mut U, $($arg_n: $arg_t),*);
        }

        impl<F, U> Callback<U> for F where F: FnMut(&mut U, $($arg_t),*) {
            fn callback(&mut self, userdata: &mut U, $($arg_n: $arg_t),*) {
                self(userdata, $($arg_n),*)
            }
        }

        pub(crate) fn set(ptr: *mut ffi::GLFWwindow) {
            unsafe {
                ffi::$glfw(ptr, Some(callback));
            }
        }

        extern "C" fn callback(ptr: *mut ffi::GLFWwindow, $($glfw_n: $glfw_t),*) {
            if let Err(_) = panic::catch_unwind(|| unsafe {
                PROCESSING_EVENTS.with(|v| v.set(true));
                let callbacks = ffi::glfwGetWindowUserPointer(ptr) as
                        *mut (&mut WindowCallbacks<c_void>, &mut c_void);
                if !callbacks.is_null() {
                    let callbacks = &mut *callbacks;
                    if let Some(ref mut cb) = callbacks.0 .$name.0 {
                        cb.callback(callbacks.1, $($transform),*);
                    }
                }
                PROCESSING_EVENTS.with(|v| v.set(false));
            }) {
                eprintln!("Panic in GLFW {} callback", stringify!($name));
                abort();
            }
        }
    };
}

macro_rules! window_callbacks {
    ($($name:ident: $tname:ident {$(use $s:path;)* args $($v:tt)*})*) => {
        pub struct WindowCallbacks<'a, U> {
            $(pub $name: $name::Owner<'a, U>),*
        }

        impl<'a, U> WindowCallbacks<'a, U> {
            pub fn new() -> Self {
                WindowCallbacks {
                    $($name: $name::Owner(None)),*
                }
            }
        }

        pub(crate) fn init_callbacks(ptr: *mut ::ffi::GLFWwindow) {
            $($name::set(ptr);)*
        }

        $(
            pub use self::$name::Callback as $tname;
            pub(crate) mod $name {
                $(use $s;)*
                window_callback!($name args $($v)*);
            }
        )*
    };
}

window_callbacks! {
    window_pos: WindowPosCallback {
        use libc::c_int;
        args = x: i32, y: i32;
        glfw = glfwSetWindowPosCallback;
        transform = x: c_int, y: c_int => x, y;
    }

    window_size: WindowSizeCallback {
        use libc::c_int;
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
        use libc::c_int;
        use cint_to_bool;
        args = focused: bool;
        glfw = glfwSetWindowFocusCallback;
        transform = focused: c_int => cint_to_bool(focused);
    }

    window_iconify: WindowIconifyCallback {
        use libc::c_int;
        use cint_to_bool;
        args = iconified: bool;
        glfw = glfwSetWindowIconifyCallback;
        transform = iconified: c_int => cint_to_bool(iconified);
    }

    window_maximize: WindowMaximizeCallback {
        use libc::c_int;
        use cint_to_bool;
        args = maximized: bool;
        glfw = glfwSetWindowMaximizeCallback;
        transform = maximized: c_int => cint_to_bool(maximized);
    }

    framebuffer_size: FramebufferSizeCallback {
        use libc::c_int;
        args = width: i32, height: i32;
        glfw = glfwSetFramebufferSizeCallback;
        transform = width: c_int, height: c_int => width, height;
    }

    window_content_scale: WindowContentScaleCallback {
        use libc::c_float;
        args = xscale: f32, yscale: f32;
        glfw = glfwSetWindowContentScaleCallback;
        transform = xscale: c_float, yscale: c_float => xscale, yscale;
    }

    mouse_button: MouseButtonCallback {
        use libc::c_int;
        use enum_primitive::FromPrimitive;
        use cint_to_bool;
        use MouseButton;
        use Modifiers;
        args = button: MouseButton, pressed: bool, mods: Modifiers;
        glfw = glfwSetMouseButtonCallback;
        transform = button: c_int, action: c_int, mods: c_int =>
                MouseButton::from_i32(button).unwrap(),
                cint_to_bool(action),
                Modifiers::from_bits(mods).unwrap();
    }

    cursor_pos: CursorPosCallback {
        use libc::c_double;
        args = x: f64, y: f64;
        glfw = glfwSetCursorPosCallback;
        transform = x: c_double, y: c_double => x, y;
    }

    cursor_enter: CursorEnterCallback {
        use libc::c_int;
        use cint_to_bool;
        args = entered: bool;
        glfw = glfwSetCursorEnterCallback;
        transform = entered: c_int => cint_to_bool(entered);
    }

    scroll: ScrollCallback {
        use libc::c_double;
        args = xoffset: f64, yoffset: f64;
        glfw = glfwSetScrollCallback;
        transform = xoffset: c_double, yoffset: c_double => xoffset, yoffset;
    }

    key: KeyCallback {
        use libc::c_int;
        use enum_primitive::FromPrimitive;
        use Action;
        use Key;
        use KeyCode;
        use Modifiers;
        args = key: Key, action: Action, mods: Modifiers;
        glfw = glfwSetKeyCallback;
        transform = key: c_int, scancode: c_int, action: c_int, mods: c_int =>
                KeyCode::from_i32(key).map_or(Key::Unknown(scancode), |c| Key::Known(c)),
                Action::from_i32(action).unwrap(),
                Modifiers::from_bits(mods).unwrap();
    }

    char: CharCallback {
        use libc::c_uint;
        use std::char::from_u32;
        args = char: char;
        glfw = glfwSetCharCallback;
        transform = char: c_uint => from_u32(char).unwrap();
    }

    char_mods: CharModsCallback {
        use libc::c_uint;
        use libc::c_int;
        use std::char::from_u32;
        use Modifiers;
        args = char: char, mods: Modifiers;
        glfw = glfwSetCharModsCallback;
        transform = char: c_uint, mods: c_int =>
                from_u32(char).unwrap(),
                Modifiers::from_bits(mods).unwrap();
    }

    file_drop: FileDropCallback {
        use libc::c_int;
        use libc::c_char;
        use std::slice;
        use std::path::PathBuf;
        use std::ffi::CStr;
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
