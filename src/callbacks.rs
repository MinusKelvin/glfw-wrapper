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

macro_rules! window_callback {
    (
        $name:ident
        args = $($arg_n:ident: $arg_t:ty),*;
        glfw = $glfw:ident;
        transform = $($glfw_n:ident: $glfw_t:ty),* => $($transform:expr),*;
    ) => {
        use std::panic;
        use std::process::abort;

        use ffi;
        use PROCESSING_EVENTS;
        use callbacks::WindowCallbacks;

        pub trait Callback {
            fn callback(&mut self, $($arg_n: $arg_t),*);
        }

        pub(crate) fn set(ptr: *mut ffi::GLFWwindow, cb: Box<Callback>) {
            unsafe {
                let callbacks = &mut *(ffi::glfwGetWindowUserPointer(ptr) as *mut WindowCallbacks);
                callbacks.$name = Some(cb);
                ffi::$glfw(ptr, Some(callback));
            }
        }

        pub(crate) fn unset(ptr: *mut ffi::GLFWwindow) {
            unsafe {
                let callbacks = &mut *(ffi::glfwGetWindowUserPointer(ptr) as *mut WindowCallbacks);
                callbacks.$name = None;
                ffi::$glfw(ptr, None);
            }
        }

        extern "C" fn callback(ptr: *mut ffi::GLFWwindow, $($glfw_n: $glfw_t),*) {
            if let Err(_) = panic::catch_unwind(|| unsafe {
                PROCESSING_EVENTS.with(|v| v.set(true));
                let callbacks = &mut *(ffi::glfwGetWindowUserPointer(ptr) as *mut WindowCallbacks);
                if let Some(ref mut cb) = callbacks.$name {
                    cb.callback($($transform),*);
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
        #[derive(Default)]
        pub(crate) struct WindowCallbacks {
            $($name: Option<Box<$tname>>),*
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
        transform = x: c_int, y: c_int => x as i32, y as i32;
    }

    window_size: WindowSizeCallback {
        use libc::c_int;
        args = width: i32, height: i32;
        glfw = glfwSetWindowSizeCallback;
        transform = x: c_int, y: c_int => x as i32, y as i32;
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
        transform = width: c_int, height: c_int => width as i32, height as i32;
    }

    window_content_scale: WindowContentScaleCallback {
        use libc::c_float;
        args = xscale: f32, yscale: f32;
        glfw = glfwSetWindowContentScaleCallback;
        transform = xscale: c_float, yscale: c_float => xscale, yscale;
    }
}