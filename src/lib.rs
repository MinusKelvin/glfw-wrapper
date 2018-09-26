//! TODO:
//! - [x] Context reference (Functions and types related to OpenGL and OpenGL ES contexts)
//! - [x] Initialization, version and error reference (Functions and types related to initialization and error handling)
//!   - [x] Error codes (Error codes)
//! - [ ] Input reference (Functions and types related to input handling)
//!   - [ ] Gamepad axes (Gamepad axes)
//!   - [ ] Gamepad buttons (Gamepad buttons)
//!   - [ ] Joystick hat states
//!   - [ ] Joysticks (Joystick IDs)
//!   - [ ] Keyboard keys (Keyboard key IDs)
//!   - [ ] Modifier key flags (Modifier key flags)
//!   - [ ] Mouse buttons (Mouse button IDs)
//!   - [ ] Standard cursor shapes (Standard system cursor shapes)
//! - [x] Monitor reference (Functions and types related to monitors)
//! - [ ] Native access (Functions related to accessing native handles)
//! - [ ] Vulkan reference (Functions and types related to Vulkan)
//! - [x] Window reference (Functions and types related to windows)
//!   - [ ] Callbacks :(
//!   - [ ] Window Icons

#[macro_use]
extern crate enum_primitive;
extern crate libc;

use std::borrow::Cow;
use std::ffi::{ CStr, CString };
use std::sync::atomic::{ AtomicBool, ATOMIC_BOOL_INIT, Ordering };
use std::marker::PhantomData;
use std::ptr;

use libc::{ c_int, c_char };
use enum_primitive::FromPrimitive;

mod enums;
mod callbacks;
mod window;
mod monitor;
mod image;

pub use enums::*;
pub use window::*;
pub use monitor::*;
pub use image::*;
pub use ffi::{
    GLFW_VERSION_MAJOR as VERSION_MAJOR,
    GLFW_VERSION_MINOR as VERSION_MINOR,
    GLFW_VERSION_REVISION as VERSION_REVISION,
    GLFWglproc as GlProc
};
pub use callbacks::{
    // error::Callback as ErrorCallback
    monitor::Callback as MonitorCallback
};

mod ffi;
mod util;

use util::*;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub description: String
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn get_version() -> (i32, i32, i32) {
    let mut triplet = (0, 0, 0);
    unsafe { ffi::glfwGetVersion(&mut triplet.0, &mut triplet.1, &mut triplet.2) };
    triplet
}

pub fn get_version_string() -> Cow<'static, str> {
    unsafe { CStr::from_ptr(ffi::glfwGetVersionString()) }.to_string_lossy()
}

pub fn get_error() -> Result<()> {
    unsafe {
        let mut desc = std::ptr::null();
        let errorcode = ffi::glfwGetError(&mut desc);
        if let Some(error) = ErrorKind::from_i32(errorcode) {
            Err(Error {
                kind: error,
                description: CStr::from_ptr(desc).to_string_lossy().into_owned()
            })
        } else {
            assert!(errorcode == ffi::GLFW_NO_ERROR);
            Ok(())
        }
    }
}

/// Tracks the initialization state of the GLFW library.
/// * `false` represents uninitialized
/// * `true` represents initialized
/// If a [`Context`] exists, this will be `true`.
static INIT_STATE: AtomicBool = ATOMIC_BOOL_INIT;

/// Initialize the GLFW library.
/// 
/// This function must be called on the first thread of the process, at least on Mac OS. All of the
/// restrictions about what can and can't be done from the main thread should be encoded in the type
/// system.
/// 
/// # Returns
/// 
/// * `Ok(Some(_))` if no context already exists
/// * `Ok(None)` if a context already exists
/// * `Err(_)` if an error occured during initialization
pub fn init(init_hints: &[InitHint]) -> Result<Option<Glfw>> {
    extern "C" fn err_cb(code: c_int, _: *const c_char) {
        if let Some(_) = ErrorKind::from_i32(code) { return }
        eprintln!("{} error occured. This should not be possible.", match code {
            ffi::GLFW_NOT_INITIALIZED => "GLFW_NOT_INITIALIZED",
            ffi::GLFW_INVALID_ENUM => "GLFW_INVALID_ENUM",
            ffi::GLFW_NO_ERROR => "GLFW_NO_ERROR (???)",
            _ => "??? (REALLY BAD)"
        });
        std::process::abort()
    }

    if INIT_STATE.swap(true, Ordering::SeqCst) == false {
        unsafe { ffi::glfwSetErrorCallback(Some(err_cb)) };
        for hint in init_hints {
            match hint {
                InitHint::CocoaChDirResources(enable) => unsafe {
                    ffi::glfwInitHint(ffi::GLFW_COCOA_CHDIR_RESOURCES, bool_to_cint(*enable))
                }
                InitHint::CocoaMenubar(enable) => unsafe {
                    ffi::glfwInitHint(ffi::GLFW_COCOA_MENUBAR, bool_to_cint(*enable))
                }
                InitHint::JoystickHatButtons(enable) => unsafe {
                    ffi::glfwInitHint(ffi::GLFW_JOYSTICK_HAT_BUTTONS, bool_to_cint(*enable))
                }
            }
        }
        if cint_to_bool(unsafe { ffi::glfwInit() }) {
            callbacks::monitor::initialize();
            Ok(Some(Glfw(PhantomData)))
        } else {
            INIT_STATE.store(false, Ordering::SeqCst);
            Err(get_error().unwrap_err())
        }
    } else {
        Ok(None)
    }
}

/// Represents ownership of the GLFW library.
pub struct Glfw(PhantomData<*const ()>);

impl Drop for Glfw {
    fn drop(&mut self) {
        invalidate_all_monitors();
        unsafe { ffi::glfwTerminate() };
        INIT_STATE.store(false, Ordering::SeqCst);
    }
}

impl Glfw {
    /// Gets a type allowing access to the parts of GLFW accessible from any thread.
    pub fn shared(&self) -> SharedGlfw {
        SharedGlfw(PhantomData)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gaa77c4898dfb83344a6b4f76aa16b9a4a
    pub fn default_window_hints(&self) {
        unsafe { ffi::glfwDefaultWindowHints() };
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga7d9c8c62384b1e2821c4dc48952d2033
    pub fn window_hint<'b>(&self, hint: WindowHint<'b>) {
        unimplemented!("Giant Window Hint Match");
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga5c336fddf2cbb5b92f65f10fb6043344
    pub fn create_window(
        &self,
        width: i32,
        height: i32,
        title: &str,
        monitor: Option<Monitor>,
        share: Option<&Window>
    ) -> Result<Window> {
        let title = CString::new(title).unwrap();
        let ptr = unsafe { ffi::glfwCreateWindow(
            width,
            height,
            title.as_ptr(),
            monitor.map_or(ptr::null_mut(), |m| m.get_ptr()),
            share.map_or(ptr::null_mut(), |w| w.ptr)
        )};
        if ptr.is_null() {
            Err(get_error().unwrap_err())
        } else {
            Ok(Window::init(ptr))
        }
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga37bd57223967b4211d60ca1a0bf3c832
    pub fn poll_events(&self) -> Result<()> {
        unsafe { ffi::glfwPollEvents() };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga554e37d781f0a997656c26b2c56c835e
    pub fn wait_events(&self) -> Result<()> {
        unsafe { ffi::glfwWaitEvents() };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga605a178db92f1a7f1a925563ef3ea2cf
    pub fn wait_events_timeout(&self, timeout: f64) -> Result<()> {
        unsafe { ffi::glfwWaitEventsTimeout(timeout) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga3fba51c8bd36491d4712aa5bd074a537
    pub fn get_monitors(&self) -> Vec<Monitor> {
        let raw = unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetMonitors(&mut count);
            if ptr.is_null() {
                return vec![];
            }
            std::slice::from_raw_parts(ptr, count as usize)
        };
        raw.iter().map(|p| Monitor::create_from(*p)).collect()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga721867d84c6d18d6790d64d2847ca0b1
    pub fn get_primary_monitor(&self) -> Option<Monitor> {
        let ptr = unsafe { ffi::glfwGetPrimaryMonitor() };
        if ptr.is_null() {
            None
        } else {
            Some(Monitor::create_from(ptr))
        }
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#gac3fe0f647f68b731f99756cd81897378
    pub fn set_monitor_callback(&self, callback: Box<MonitorCallback>) {
        callbacks::monitor::set(callback);
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#gac3fe0f647f68b731f99756cd81897378
    pub fn unset_monitor_callback(&self) {
        callbacks::monitor::unset();
    }
}

/// Encapsulates GLFW library calls that can be called from any thread.
#[derive(Copy, Clone)]
pub struct SharedGlfw<'a>(pub(crate) PhantomData<&'a Glfw>);
unsafe impl<'a> Send for SharedGlfw<'a> {}
unsafe impl<'a> Sync for SharedGlfw<'a> {}

impl<'a> SharedGlfw<'a> {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gab5997a25187e9fd5c6f2ecbbc8dfd7e9
    pub fn post_empty_event(&self) -> Result<()> {
        unsafe { ffi::glfwPostEmptyEvent() };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__context.html#ga1c04dc242268f827290fe40aa1c91157
    pub fn clear_current_context(&self) -> Result<()> {
        unsafe { ffi::glfwMakeContextCurrent(ptr::null_mut()) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__context.html#ga6d4e0cdf151b5e579bd67f13202994ed
    pub unsafe fn swap_interval(&self, interval: i32) -> Result<()> {
        ffi::glfwSwapInterval(interval);
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__context.html#ga87425065c011cef1ebd6aac75e059dfa
    pub unsafe fn extension_supported(&self, extension: &str) -> Result<bool> {
        let cstr = CString::new(extension).unwrap();
        let supported = ffi::glfwExtensionSupported(cstr.as_ptr());
        get_error().map(|_| cint_to_bool(supported))
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__context.html#ga35f1837e6f666781842483937612f163
    pub unsafe fn get_proc_address(&self, proc_name: &str) -> Result<GlProc> {
        let cstr = CString::new(proc_name).unwrap();
        let proc = ffi::glfwGetProcAddress(cstr.as_ptr());
        get_error().map(|_| { assert!(!proc.is_null()); proc })
    }
}