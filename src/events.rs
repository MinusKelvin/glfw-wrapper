use std::os::raw::{ c_char, c_int, c_uint, c_float, c_double };
use std::path::PathBuf;
use std::char::from_u32;
use std::panic;
use std::process::abort;
use std::slice;
use std::ffi::CStr;
use enum_primitive::FromPrimitive;

use Key;
use KeyCode;
use MouseButton;
use Modifiers;
use Window;
use ffi;
use cint_to_bool;
use Monitor;
use DisconnectedMonitor;
use Joystick;
use invalidate_monitor;

pub(crate) static mut EVENT_PROCESSOR: Option<*mut FnMut(Event) -> ()> = None;

#[derive(Debug)]
pub enum Event<'a> {
    MonitorConnected(Monitor),
    MonitorDisconnected(&'a DisconnectedMonitor),
    JoystickConnected(Joystick),
    JoystickDisconnected(Joystick),
    WindowPosition {
        win: &'a Window<'a>,
        x: i32,
        y: i32
    },
    WindowResize {
        win: &'a Window<'a>,
        width: i32,
        height: i32
    },
    WindowCloseRequested(&'a Window<'a>),
    WindowRefresh(&'a Window<'a>),
    WindowFocused(&'a Window<'a>),
    WindowDefocused(&'a Window<'a>),
    WindowIconified(&'a Window<'a>),
    WindowDeiconified(&'a Window<'a>),
    WindowMaximized(&'a Window<'a>),
    WindowDemaximized(&'a Window<'a>),
    FramebufferSizeChanged {
        win: &'a Window<'a>,
        width: i32,
        height: i32
    },
    ContentScaleChanged {
        win: &'a Window<'a>,
        x_scale: f32,
        y_scale: f32
    },
    MouseButtonDown {
        win: &'a Window<'a>,
        button: MouseButton,
        modifiers: Modifiers
    },
    MouseButtonUp {
        win: &'a Window<'a>,
        button: MouseButton,
        modifiers: Modifiers
    },
    CursorMoved {
        win: &'a Window<'a>,
        x: f64,
        y: f64
    },
    CursorEntered(&'a Window<'a>),
    CursorExited(&'a Window<'a>),
    Scroll {
        win: &'a Window<'a>,
        x_offset: f64,
        y_offset: f64
    },
    KeyDown {
        win: &'a Window<'a>,
        key: Key,
        modifiers: Modifiers
    },
    KeyUp {
        win: &'a Window<'a>,
        key: Key,
        modifiers: Modifiers
    },
    KeyRepeat {
        win: &'a Window<'a>,
        key: Key,
        modifiers: Modifiers
    },
    CharTyped(&'a Window<'a>, char),
    FileDrop(&'a Window<'a>, Vec<PathBuf>)
}

macro_rules! callback {
    ($name:ident, $cb:ident; $($p_n:ident: $p_t:ty),* => $code:block) => {
        extern "C" fn $name($($p_n: $p_t),*) {
            if let Err(_) = panic::catch_unwind(|| unsafe {
                if let Some(cbptr) = EVENT_PROCESSOR {
                    let $cb = &mut *cbptr;
                    $code
                }
            }) {
                eprintln!(concat!("Panic in GLFW ", stringify!($name), " callback"));
                abort();
            }
        }
    };
}

callback! { monitor, cb;
    monitor: *mut ffi::GLFWmonitor, event: c_int => {
        match event {
            ffi::GLFW_CONNECTED => cb(Event::MonitorConnected(Monitor::create_from(monitor))),
            ffi::GLFW_DISCONNECTED => {
                invalidate_monitor(monitor);
                cb(Event::MonitorDisconnected(&DisconnectedMonitor(monitor)));
            }
            _ => unreachable!()
        }
    }
}

callback! { joystick, cb;
    joystick: c_int, event: c_int => {
        match event {
            ffi::GLFW_CONNECTED =>
                    cb(Event::JoystickConnected(Joystick::from_i32(joystick).unwrap())),
            ffi::GLFW_DISCONNECTED =>
                    cb(Event::JoystickDisconnected(Joystick::from_i32(joystick).unwrap())),
            _ => unreachable!()
        }
    }
}

pub(crate) fn initialize_callbacks() {
    unsafe {
        ffi::glfwSetMonitorCallback(Some(monitor));
        ffi::glfwSetJoystickCallback(Some(joystick));
    }
}

macro_rules! window_callbacks {
    ($($name:ident {
        glfw = $glfw:ident;
        transform $w:ident = $($glfw_n:ident: $glfw_t:ty),* => $transform:expr;
    })*) => {
        pub(crate) fn init_window_callbacks(ptr: *mut ::ffi::GLFWwindow) {
            unsafe { $(
                ffi::$glfw(ptr, Some($name));
            )* }
        }

        $(
            callback! { $name, cb;
                ptr: *mut ffi::GLFWwindow $(, $glfw_n: $glfw_t)* => {
                    let $w = Window::init(None, ptr);
                    cb($transform);
                }
            }
        )*
    };
}

window_callbacks! {
    window_pos {
        glfw = glfwSetWindowPosCallback;
        transform win = x: c_int, y: c_int => Event::WindowPosition { win: &win, x, y };
    }

    window_size {
        glfw = glfwSetWindowSizeCallback;
        transform win = width: c_int, height: c_int =>
                Event::WindowResize { win: &win, width, height };
    }

    window_close {
        glfw = glfwSetWindowCloseCallback;
        transform win = => Event::WindowCloseRequested(&win);
    }

    window_refresh {
        glfw = glfwSetWindowRefreshCallback;
        transform win = => Event::WindowRefresh(&win);
    }

    window_focus {
        glfw = glfwSetWindowFocusCallback;
        transform win = focused: c_int => if cint_to_bool(focused) {
            Event::WindowFocused(&win)
        } else {
            Event::WindowDefocused(&win)
        };
    }

    window_iconify {
        glfw = glfwSetWindowIconifyCallback;
        transform win = iconified: c_int => if cint_to_bool(iconified) {
            Event::WindowIconified(&win)
        } else {
            Event::WindowDeiconified(&win)
        };
    }

    window_maximize {
        glfw = glfwSetWindowMaximizeCallback;
        transform win = maximized: c_int => if cint_to_bool(maximized) {
            Event::WindowMaximized(&win)
        } else {
            Event::WindowDemaximized(&win)
        };
    }

    framebuffer_size {
        glfw = glfwSetFramebufferSizeCallback;
        transform win = width: c_int, height: c_int =>
                Event::FramebufferSizeChanged { win: &win, width, height };
    }

    content_scale {
        glfw = glfwSetWindowContentScaleCallback;
        transform win = x_scale: c_float, y_scale: c_float =>
                Event::ContentScaleChanged { win: &win, x_scale, y_scale };
    }

    mouse_button {
        glfw = glfwSetMouseButtonCallback;
        transform win = button: c_int, action: c_int, mods: c_int => if cint_to_bool(action) {
            Event::MouseButtonDown {
                win: &win,
                button: MouseButton::from_i32(button).unwrap(),
                modifiers: Modifiers::from_bits(mods).unwrap()
            }
        } else {
            Event::MouseButtonUp {
                win: &win,
                button: MouseButton::from_i32(button).unwrap(),
                modifiers: Modifiers::from_bits(mods).unwrap()
            }
        };
    }

    cursor_pos {
        glfw = glfwSetCursorPosCallback;
        transform win = x: c_double, y: c_double => Event::CursorMoved { win: &win, x, y };
    }

    cursor_enter {
        glfw = glfwSetCursorEnterCallback;
        transform win = entered: c_int => if cint_to_bool(entered) {
            Event::CursorEntered(&win)
        } else {
            Event::CursorExited(&win)
        };
    }

    scroll {
        glfw = glfwSetScrollCallback;
        transform win = x_offset: c_double, y_offset: c_double =>
                Event::Scroll { win: &win, x_offset, y_offset };
    }

    key {
        glfw = glfwSetKeyCallback;
        transform win = key: c_int, scancode: c_int, action: c_int, mods: c_int => match action {
            ffi::GLFW_PRESS => Event::KeyDown {
                win: &win,
                key: KeyCode::from_i32(key).map_or(Key::Unnamed(scancode), |c| Key::Named(c)),
                modifiers: Modifiers::from_bits(mods).unwrap()
            },
            ffi::GLFW_RELEASE => Event::KeyUp {
                win: &win,
                key: KeyCode::from_i32(key).map_or(Key::Unnamed(scancode), |c| Key::Named(c)),
                modifiers: Modifiers::from_bits(mods).unwrap()
            },
            ffi::GLFW_REPEAT => Event::KeyRepeat {
                win: &win,
                key: KeyCode::from_i32(key).map_or(Key::Unnamed(scancode), |c| Key::Named(c)),
                modifiers: Modifiers::from_bits(mods).unwrap()
            },
            _ => unreachable!()
        };
    }

    char {
        glfw = glfwSetCharCallback;
        transform win = char: c_uint => Event::CharTyped(&win, from_u32(char).unwrap());
    }

    file_drop {
        glfw = glfwSetDropCallback;
        transform win = count: c_int, paths: *const *const c_char => {
            let s = slice::from_raw_parts(paths, count as usize);
            Event::FileDrop(&win, s.iter().map(|p|
                PathBuf::from(CStr::from_ptr(*p).to_string_lossy().into_owned())
            ).collect())
        };
    }
}