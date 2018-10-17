//! A safe Rust wrapper around the [GLFW] library.
//! 
//! See [the repository] for examples.
//! 
//! [GLFW]: http://www.glfw.org/
//! [the repository]: https://github.com/MinusKelvin/glfw-wrapper

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate scopeguard;

#[cfg(all(
    any(feature = "expose-win32", feature = "expose-wgl"),
    target_os="windows"
))] extern crate winapi;

#[cfg(all(
    any(feature = "expose-x11", feature = "expose-glx"),
    any(target_os="linux", target_os="freebsd", target_os="dragonfly")
))] extern crate x11;

use std::borrow::Cow;
use std::ffi::{ CStr, CString };
use std::sync::atomic::{ AtomicBool, ATOMIC_BOOL_INIT, Ordering };
use std::marker::PhantomData;
use std::ptr;
use std::cell::{ RefCell, Cell };
use std::slice;
use std::ops::Deref;
use std::os::raw::{ c_int, c_char };

use enum_primitive::FromPrimitive;

mod enums;
mod callbacks;
mod window;
mod monitor;
mod misc;

pub use enums::*;
pub use window::*;
pub use monitor::*;
pub use misc::*;
pub use ffi::GLFWglproc as GlProc;
pub use callbacks::*;

mod ffi;
mod util;

use util::*;

pub const VERSION: (i32, i32, i32) =
        (ffi::GLFW_VERSION_MAJOR, ffi::GLFW_VERSION_MINOR, ffi::GLFW_VERSION_REVISION);

/// Represents a GLFW error.
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub description: String
}

/// Specialized `Result` type for GLFW errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Retrieves the runtime version of GLFW.
/// 
/// This may differ from the [`VERSION`] constant, as that represents the version compiled against.
/// This functions is intended for checking that GLFW is at least a minimum required version when
/// linked to GLFW as a shared library.
/// 
/// [GLFW Reference][glfw]
/// 
/// [glfw]: http://www.glfw.org/docs/3.3/group__init.html#ga9f8ffaacf3c269cc48eafbf8b9b71197
/// [`VERSION`]: constant.VERSION.html
pub fn get_version() -> (i32, i32, i32) {
    let mut triplet = (0, 0, 0);
    unsafe { ffi::glfwGetVersion(&mut triplet.0, &mut triplet.1, &mut triplet.2) };
    triplet
}

/// Retrieves a string describing the version, platform, compiler, and platform-specific
/// compile-time options used to compile GLFW.
/// 
/// [GLFW Reference][glfw]
/// 
/// [glfw]: http://www.glfw.org/docs/3.3/group__init.html#ga23d47dc013fce2bf58036da66079a657
pub fn get_version_string() -> Cow<'static, str> {
    unsafe { CStr::from_ptr(ffi::glfwGetVersionString()) }.to_string_lossy()
}

/// Retrieves and clears the last GLFW error that occured on the calling thread.
/// 
/// Currently, all functions that the documentation says could result in errors other than
/// `GLFW_NOT_INITIALIZED` and `GLFW_INVALID_ENUM` return a result obtained from calling this
/// function, clearing the error. Until this changes, this function should always return `Ok`.
/// 
/// [GLFW Reference][glfw]
/// 
/// [glfw]: http://www.glfw.org/docs/3.3/group__init.html#ga944986b4ec0b928d488141f92982aa18
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
/// 
/// * `false` represents uninitialized
/// * `true` represents initialized
/// 
/// If a [`Glfw`] exists, this will be `true`.
/// 
/// [`Glfw`]: struct.Glfw.html
static INIT_STATE: AtomicBool = ATOMIC_BOOL_INIT;

#[derive(Debug)]
pub enum InitError {
    AlreadyInitialized,
    Failed(Error)
}

/// Initializes GLFW.
/// 
/// Calls to this function while a [`Glfw`] instance exists results in an
/// `Err(InitError::AlreadyInitialized)`.
/// 
/// This function must be called on the first thread of the process, at least on Mac OS. All of the
/// restrictions about what can and can't be done from the main thread should be encoded in the type
/// system.
/// 
/// # Deviations from GLFW
/// 
/// Instead of setting initialization hints through a separate (stateful) function, this function
/// takes an [`InitHints`] and doubles as `glfwInitHint`. We do this because `glfwInitHint` must
/// only be called from the main thread, and we cannot enforce that without an instance of a type
/// that can only live on the main thread. This design is also preferred because it eliminates some
/// hidden state.
/// 
/// # See Also
/// 
/// * [`glfwInit()`]
/// * [`glfwInitHint()`]
/// 
/// [`Glfw`]: struct.Glfw.html
/// [`InitHints`]: struct.InitHints.html
/// [`glfwInit()`]: http://www.glfw.org/docs/3.3/group__init.html#ga317aac130a235ab08c6db0834907d85e
/// [`glfwInitHint()`]: http://www.glfw.org/docs/3.3/group__init.html#ga110fd1d3f0412822b4f1908c026f724a
pub fn init(init_hints: InitHints) -> std::result::Result<Glfw, InitError> {
    extern "C" fn err_cb(code: c_int, _: *const c_char) {
        if ErrorKind::from_i32(code).is_some() { return }
        eprintln!("{} error occured. This should not be possible.", match code {
            ffi::GLFW_NOT_INITIALIZED => "GLFW_NOT_INITIALIZED",
            ffi::GLFW_INVALID_ENUM => "GLFW_INVALID_ENUM",
            ffi::GLFW_NO_ERROR => "GLFW_NO_ERROR (???)",
            _ => "??? (REALLY BAD)"
        });
        std::process::abort()
    }

    if INIT_STATE.swap(true, Ordering::SeqCst) == false {
        unsafe {
            ffi::glfwSetErrorCallback(Some(err_cb));
            ffi::glfwInitHint(
                ffi::GLFW_COCOA_CHDIR_RESOURCES,
                bool_to_cint(init_hints.cocoa_chdir_resources)
            );
            ffi::glfwInitHint(ffi::GLFW_COCOA_MENUBAR, bool_to_cint(init_hints.cocoa_menubar));
            ffi::glfwInitHint(
                ffi::GLFW_JOYSTICK_HAT_BUTTONS,
                bool_to_cint(init_hints.joystick_hat_buttons)
            );
        }
        if cint_to_bool(unsafe { ffi::glfwInit() }) {
            callbacks::monitor::initialize();
            Ok(Glfw {
                shared: SharedGlfw(PhantomData),
                _phantom: PhantomData
            })
        } else {
            INIT_STATE.store(false, Ordering::SeqCst);
            Err(InitError::Failed(get_error().unwrap_err()))
        }
    } else {
        Err(InitError::AlreadyInitialized)
    }
}

thread_local! {
    // The only things that can affect these live on the main thread
    pub(crate) static PROCESSING_EVENTS: Cell<bool> = Cell::new(false);
    pub(crate) static REENTRANCE_AVOIDANCE: RefCell<Vec<ReentranceAvoidanceCommand>> =
            RefCell::new(Vec::new());
}

enum ReentranceAvoidanceCommand {
    DestroyWindow(*mut ffi::GLFWwindow),
    DestroyCursor(*mut ffi::GLFWcursor)
}

/// Represents ownership of the GLFW library.
/// 
/// Only one of these can exist at any point in time. Terminates the GLFW library when dropped.
pub struct Glfw {
    shared: SharedGlfw,
    _phantom: PhantomData<*const ()>
}

impl Drop for Glfw {
    fn drop(&mut self) {
        self.process_reentrance_avoidance();
        invalidate_all_monitors();
        unsafe { ffi::glfwTerminate() };
        INIT_STATE.store(false, Ordering::SeqCst);
    }
}

impl Glfw {
    pub(crate) fn destroy_window(&self, ptr: *mut ffi::GLFWwindow) {
        PROCESSING_EVENTS.with(|v| if v.get() {
            use ReentranceAvoidanceCommand::DestroyWindow;
            REENTRANCE_AVOIDANCE.with(|v| v.borrow_mut().push(DestroyWindow(ptr)));
            // Delayed window destruction; it may be a while before an event processing call occurs
            // (how? some callbacks can be called outside of poll/wait events) but we don't want
            // the user to see a window still active that's not supposed to be there.
            unsafe { ffi::glfwHideWindow(ptr) };
        } else {
            unsafe {
                ffi::glfwDestroyWindow(ptr);
            }
        })
    }

    pub(crate) fn destroy_cursor(&self, ptr: *mut ffi::GLFWcursor) {
        PROCESSING_EVENTS.with(|v| if v.get() {
            use ReentranceAvoidanceCommand::DestroyCursor;
            REENTRANCE_AVOIDANCE.with(|v| v.borrow_mut().push(DestroyCursor(ptr)))
        } else {
            unsafe {
                ffi::glfwDestroyCursor(ptr);
            }
        })
    }

    fn process_reentrance_avoidance(&self) {
        REENTRANCE_AVOIDANCE.with(|v| {
            let mut list = v.borrow_mut();
            for todo in list.drain(..) {
                use ReentranceAvoidanceCommand::*;
                match todo {
                    DestroyWindow(w) => unsafe { ffi::glfwDestroyWindow(w) }
                    DestroyCursor(c) => unsafe { ffi::glfwDestroyCursor(c) }
                }
            }
        })
    }

    /// Gets a type allowing access to the parts of GLFW accessible from any thread.
    pub fn shared(&self) -> &SharedGlfw {
        &self.shared
    }

    /// Creates a window and its associated OpenGL or OpenGL ES context.
    /// 
    /// This function does not change what context is current. To make the context for this window
    /// current, call [`make_context_current()`].
    /// 
    /// The created OpenGL or OpenGL ES context can optionally be shared with another window's
    /// context. For more information, see the [GLFW Reference][share].
    /// 
    /// The created window, framebuffer, and context may not match what you requested, as some not
    /// all window hints are hard constraints. See the [`WindowHints`] struct for more details. Use
    /// the relevant window querying functions to obtain the actual configuration.
    /// 
    /// To create a fullscreen window, specify the monitor to create it on; it will be windowed mode
    /// otherwise. It is recommended that you pick the primary monitor. For fullscreen windows, the
    /// specified size becomes the resolution of the window's desired video mode. If the closest
    /// available match to the window's desired video mode is the current one, GLFW will create a
    /// "windowed fullscreen" or "borderless fullscreen" window. For more information, see the
    /// [GLFW Reference][window-full].
    /// 
    /// # Deviations from GLFW
    /// 
    /// Instead of setting window hints through a separate (stateful) function, this function takes
    /// a [`WindowHints`] and doubles as `glfwWindowHint` and `glfwWindowHintString`. This was done
    /// to match our API for [`init()`] and because this design eliminates some hidden state.
    /// 
    /// # See Also
    /// 
    /// * [`glfwCreateWindow()`][glfw]
    /// * [Window Creation Guide]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga5c336fddf2cbb5b92f65f10fb6043344
    /// [share]: http://www.glfw.org/docs/3.3/context_guide.html#context_sharing
    /// [window-full]: http://www.glfw.org/docs/3.3/window_guide.html#window_windowed_full_screen
    /// [Window Creation Guide]: http://www.glfw.org/docs/3.3/window_guide.html#window_creation
    /// [`make_context_current()`]: struct.Window.html#method.make_context_current
    /// [`WindowHints`]: struct.WindowHints.html
    /// [`init()`]: fn.init.html
    pub fn create_window(
        &self,
        window_hints: &WindowHints,
        width: i32,
        height: i32,
        title: &str,
        monitor: Option<Monitor>,
        share: Option<&Window>
    ) -> Result<Window> {
        unsafe {
            ffi::glfwWindowHint(ffi::GLFW_RESIZABLE,     bool_to_cint(window_hints.resizable));
            ffi::glfwWindowHint(ffi::GLFW_VISIBLE,       bool_to_cint(window_hints.visible));
            ffi::glfwWindowHint(ffi::GLFW_DECORATED,     bool_to_cint(window_hints.decorated));
            ffi::glfwWindowHint(ffi::GLFW_FOCUSED,       bool_to_cint(window_hints.focused));
            ffi::glfwWindowHint(ffi::GLFW_AUTO_ICONIFY,  bool_to_cint(window_hints.auto_iconify));
            ffi::glfwWindowHint(ffi::GLFW_FLOATING,      bool_to_cint(window_hints.floating));
            ffi::glfwWindowHint(ffi::GLFW_MAXIMIZED,     bool_to_cint(window_hints.maximized));
            ffi::glfwWindowHint(ffi::GLFW_CENTER_CURSOR, bool_to_cint(window_hints.center_cursor));
            ffi::glfwWindowHint(ffi::GLFW_STEREO,        bool_to_cint(window_hints.stereo));
            ffi::glfwWindowHint(ffi::GLFW_SRGB_CAPABLE,  bool_to_cint(window_hints.srgb_capable));
            ffi::glfwWindowHint(ffi::GLFW_DOUBLEBUFFER,  bool_to_cint(window_hints.double_buffer));
            ffi::glfwWindowHint(ffi::GLFW_TRANSPARENT_FRAMEBUFFER,
                    bool_to_cint(window_hints.transparent_framebuffer));

            ffi::glfwWindowHint(ffi::GLFW_RED_BITS,     window_hints.red_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_GREEN_BITS,   window_hints.green_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_BLUE_BITS,    window_hints.blue_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_ALPHA_BITS,   window_hints.alpha_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_DEPTH_BITS,   window_hints.depth_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_STENCIL_BITS, window_hints.stencil_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_ACCUM_RED_BITS,
                    window_hints.accum_red_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_ACCUM_GREEN_BITS,
                    window_hints.accum_green_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_ACCUM_BLUE_BITS,
                    window_hints.accum_blue_bits.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_ACCUM_ALPHA_BITS,
                    window_hints.accum_alpha_bits.or_dont_care());

            ffi::glfwWindowHint(ffi::GLFW_AUX_BUFFERS,
                    window_hints.auxiliary_buffers.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_SAMPLES, window_hints.samples.or_dont_care());
            ffi::glfwWindowHint(ffi::GLFW_REFRESH_RATE, window_hints.refresh_rate.or_dont_care());

            ffi::glfwWindowHint(ffi::GLFW_CLIENT_API, window_hints.client_api as i32);
            ffi::glfwWindowHint(ffi::GLFW_CONTEXT_CREATION_API,
                    window_hints.context_creation_api as i32);
            ffi::glfwWindowHint(ffi::GLFW_CONTEXT_ROBUSTNESS,
                    window_hints.context_robustness as i32);
            ffi::glfwWindowHint(ffi::GLFW_OPENGL_PROFILE, window_hints.opengl_profile as i32);
            ffi::glfwWindowHint(ffi::GLFW_CONTEXT_RELEASE_BEHAVIOR,
                    window_hints.context_release_behavior as i32);

            ffi::glfwWindowHint(ffi::GLFW_CONTEXT_VERSION_MAJOR, window_hints.context_version.0);
            ffi::glfwWindowHint(ffi::GLFW_CONTEXT_VERSION_MINOR, window_hints.context_version.1);

            ffi::glfwWindowHint(ffi::GLFW_OPENGL_FORWARD_COMPAT,
                    bool_to_cint(window_hints.opengl_forward_compatible));
            ffi::glfwWindowHint(ffi::GLFW_OPENGL_DEBUG_CONTEXT,
                    bool_to_cint(window_hints.opengl_debug_context));
            ffi::glfwWindowHint(ffi::GLFW_CONTEXT_NO_ERROR,
                    bool_to_cint(window_hints.context_no_error));

            ffi::glfwWindowHint(ffi::GLFW_COCOA_RETINA_FRAMEBUFFER,
                    bool_to_cint(window_hints.cocoa_retina_framebuffer));
            ffi::glfwWindowHint(ffi::GLFW_COCOA_GRAPHICS_SWITCHING,
                    bool_to_cint(window_hints.cocoa_graphics_switching));
            let cstr = CString::new(window_hints.cocoa_frame_name).unwrap();
            ffi::glfwWindowHintString(ffi::GLFW_COCOA_FRAME_NAME, cstr.as_ptr());

            let cstr = CString::new(window_hints.x11_class_name).unwrap();
            ffi::glfwWindowHintString(ffi::GLFW_X11_CLASS_NAME, cstr.as_ptr());
            let cstr = CString::new(window_hints.x11_instance_name).unwrap();
            ffi::glfwWindowHintString(ffi::GLFW_X11_INSTANCE_NAME, cstr.as_ptr());
        }
        let title = CString::new(title).unwrap();
        let ptr = unsafe { ffi::glfwCreateWindow(
            width,
            height,
            title.as_ptr(),
            monitor.map_or(ptr::null_mut(), |m| m.get_ptr()),
            share.map_or(ptr::null_mut(), |w| w.ptr)
        )};
        get_error().map(|_| {
            assert!(!ptr.is_null());
            Window::init(Some(self), ptr)
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga37bd57223967b4211d60ca1a0bf3c832
    pub fn poll_events(&self) -> Result<()> {
        PROCESSING_EVENTS.with(|v| if v.get() {
            panic!("Call to non-rentrant function during event processing.");
        });
        self.process_reentrance_avoidance();
        unsafe { ffi::glfwPollEvents() };
        let e = get_error();
        self.process_reentrance_avoidance();
        e
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga554e37d781f0a997656c26b2c56c835e
    pub fn wait_events(&self) -> Result<()> {
        PROCESSING_EVENTS.with(|v| if v.get() {
            panic!("Call to non-rentrant function during event processing.");
        });
        // Avoids blocking if the last window was dropped in a callback outside poll/wait events
        self.process_reentrance_avoidance();
        unsafe { ffi::glfwWaitEvents() };
        let e = get_error();
        self.process_reentrance_avoidance();
        e
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga605a178db92f1a7f1a925563ef3ea2cf
    pub fn wait_events_timeout(&self, timeout: f64) -> Result<()> {
        PROCESSING_EVENTS.with(|v| if v.get() {
            panic!("Call to non-rentrant function during event processing.");
        });
        // Avoids blocking if the last window was dropped in a callback outside poll/wait events
        self.process_reentrance_avoidance();
        unsafe { ffi::glfwWaitEventsTimeout(timeout) };
        let e = get_error();
        self.process_reentrance_avoidance();
        e
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
        unsafe {
            ffi::glfwGetPrimaryMonitor().as_mut().map(|p| Monitor::create_from(p))
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

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga237a182e5ec0b21ce64543f3b5e7e2be
    pub fn get_key_name(&self, key: Key) -> Result<Option<String>> {
        let ptr = unsafe { match key {
            Key::Named(kc) => ffi::glfwGetKeyName(kc as i32, 0),
            Key::Unnamed(sc) => ffi::glfwGetKeyName(ffi::GLFW_KEY_UNKNOWN, sc)
        } };
        get_error().map(|_| unsafe {
            ptr.as_ref().map(|p| CStr::from_ptr(p).to_string_lossy().into_owned())
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga67ddd1b7dcbbaff03e4a76c0ea67103a
    pub fn get_key_scancode(&self, keycode: KeyCode) -> Result<i32> {
        let sc = unsafe { ffi::glfwGetKeyScancode(keycode as i32) };
        get_error().map(|_| sc)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gafca356935e10135016aa49ffa464c355
    pub fn create_cursor(&self, image: &Image, xhot: i32, yhot: i32) -> Result<Cursor> {
        let image = image.as_glfw_image();
        let ptr = unsafe { ffi::glfwCreateCursor(&image, xhot, yhot) };
        get_error().map(|_| {
            assert!(!ptr.is_null());
            Cursor {
                ptr: ptr,
                glfw: self
            }
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaa65f416d03ebbbb5b8db71a489fcb894
    pub fn create_standard_cursor(&self, shape: StandardCursorShape) -> Result<Cursor> {
        let ptr = unsafe { ffi::glfwCreateStandardCursor(shape as i32) };
        get_error().map(|_| {
            assert!(!ptr.is_null());
            Cursor {
                ptr: ptr,
                glfw: self
            }
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaed0966cee139d815317f9ffcba64c9f1
    pub fn is_joystick_present(&self, joystick: Joystick) -> Result<bool> {
        let v = unsafe { ffi::glfwJoystickPresent(joystick as i32) };
        get_error().map(|_| cint_to_bool(v))
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaa8806536731e92c061bc70bcff6edbd0
    pub fn get_joystick_axes(&self, joystick: Joystick) -> Result<Option<Vec<f32>>> {
        let mut count = 0;
        let ptr = unsafe { ffi::glfwGetJoystickAxes(joystick as i32, &mut count) };
        get_error().map(|_| unsafe {
            ptr.as_ref().map(|p| slice::from_raw_parts(p, count as usize).to_owned())
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gadb3cbf44af90a1536f519659a53bddd6
    pub fn get_joystick_buttons(&self, joystick: Joystick) -> Result<Option<Vec<bool>>> {
        let mut count = 0;
        let ptr = unsafe { ffi::glfwGetJoystickButtons(joystick as i32, &mut count) };
        get_error().map(|_| unsafe {
            ptr.as_ref().map(|p| slice::from_raw_parts(p, count as usize)
                    .iter().map(|c| cuchar_to_bool(*c)).collect())
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga2d8d0634bb81c180899aeb07477a67ea
    pub fn get_joystick_hats(&self, joystick: Joystick) -> Result<Option<Vec<JoystickHatState>>> {
        let mut count = 0;
        let ptr = unsafe { ffi::glfwGetJoystickHats(joystick as i32, &mut count) };
        get_error().map(|_| unsafe {
            ptr.as_ref().map(|p| slice::from_raw_parts(p, count as usize)
                    .iter().map(|c| JoystickHatState::from_bits(*c).unwrap()).collect())
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gafbe3e51f670320908cfe4e20d3e5559e
    pub fn get_joystick_name(&self, joystick: Joystick) -> Result<Option<String>> {
        let ptr = unsafe { ffi::glfwGetJoystickName(joystick as i32) };
        get_error().map(|_| unsafe {
            ptr.as_ref().map(|p| CStr::from_ptr(p).to_string_lossy().into_owned())
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gae168c2c0b8cf2a1cb67c6b3c00bdd543
    pub fn get_joystick_guid(&self, joystick: Joystick) -> Result<Option<String>> {
        let ptr = unsafe { ffi::glfwGetJoystickGUID(joystick as i32) };
        get_error().map(|_| unsafe {
            ptr.as_ref().map(|p| CStr::from_ptr(p).to_string_lossy().into_owned())
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gad0f676860f329d80f7e47e9f06a96f00
    pub fn is_joystick_gamepad(&self, joystick: Joystick) -> bool {
        cint_to_bool(unsafe { ffi::glfwJoystickIsGamepad(joystick as i32) })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gab1dc8379f1b82bb660a6b9c9fa06ca07
    pub fn set_joystick_callback(&self, callback: Box<JoystickCallback>) {
        joystick::set(callback);
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gab1dc8379f1b82bb660a6b9c9fa06ca07
    pub fn unset_joystick_callback(&self) {
        joystick::unset();
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaed5104612f2fa8e66aa6e846652ad00f
    pub fn update_gamepad_mappings(&self, mapping: &str) -> Result<()> {
        let cstr = CString::new(mapping).unwrap();
        unsafe { ffi::glfwUpdateGamepadMappings(cstr.as_ptr()) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga5c71e3533b2d384db9317fcd7661b210
    pub fn get_gamepad_name(&self, joystick: Joystick) -> Option<String> {
        unsafe {
            let ptr = ffi::glfwGetGamepadName(joystick as i32);
            ptr.as_ref().map(|p| CStr::from_ptr(p).to_string_lossy().into_owned())
        }
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gadccddea8bce6113fa459de379ddaf051
    pub fn get_gamepad_state(&self, joystick: Joystick) -> Option<GamepadState> {
        let mut gamepad = ffi::GLFWgamepadstate::default();
        if cint_to_bool(unsafe { ffi::glfwGetGamepadState(joystick as i32, &mut gamepad) }) {
            Some(gamepad.into())
        } else {
            None
        }
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaba1f022c5eb07dfac421df34cdcd31dd
    pub fn set_clipboard_string(&self, string: &str) -> Result<()> {
        let cstr = CString::new(string).unwrap();
        unsafe { ffi::glfwSetClipboardString(ptr::null_mut(), cstr.as_ptr()) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga5aba1d704d9ab539282b1fbe9f18bb94
    pub fn get_clipboard_string(&self) -> Result<String> {
        let ptr = unsafe { ffi::glfwGetClipboardString(ptr::null_mut()) };
        get_error().map(|_| unsafe {
            assert!(!ptr.is_null());
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        })
    }
}

impl Deref for Glfw {
    type Target = SharedGlfw;

    fn deref(&self) -> &SharedGlfw {
        &self.shared
    }
}

/// Encapsulates GLFW library calls that can be called from any thread.
/// 
/// Can only be obtained from [`Glfw::shared()`].
/// 
/// [`Glfw::shared()`]: struct.Glfw.html#method.shared
pub struct SharedGlfw(PhantomData<*const ()>);
unsafe impl Sync for SharedGlfw {}

impl SharedGlfw {
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
    pub unsafe fn clear_current_context(&self) -> Result<()> {
        ffi::glfwMakeContextCurrent(ptr::null_mut());
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
        get_error().map(|_| proc)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaa6cf4e7a77158a3b8fd00328b1720a4a
    pub fn get_time(&self) -> f64 {
        unsafe { ffi::glfwGetTime() }
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaf59589ef6e8b8c8b5ad184b25afd4dc0
    pub fn set_time(&self, time: f64) -> Result<()> {
        unsafe { ffi::glfwSetTime(time) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga09b2bd37d328e0b9456c7ec575cc26aa
    pub fn get_timer_value(&self) -> u64 {
        unsafe { ffi::glfwGetTimerValue() }
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga3289ee876572f6e91f06df3a24824443
    pub fn get_timer_frequency(&self) -> u64 {
        unsafe { ffi::glfwGetTimerFrequency() }
    }
}

#[cfg(all(
    feature = "expose-x11",
    any(target_os="linux", target_os="freebsd", target_os="dragonfly")
))]
impl SharedGlfw {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#ga8519b66594ea3ef6eeafaa2e3ee37406
    pub unsafe fn get_x11_display(&self) -> *mut x11::xlib::Display {
        ffi::x11::glfwGetX11Display()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#ga55f879ab02d93367f966186b6f0133f7
    pub unsafe fn set_x11_selection_string(&self, string: &str) -> Result<()> {
        let string = CString::new(string).unwrap();
        ffi::x11::glfwSetX11SelectionString(string.as_ptr());
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#ga72f23e3980b83788c70aa854eca31430
    pub unsafe fn get_x11_selection_string(&self) -> Result<String> {
        let ptr = ffi::x11::glfwGetX11SelectionString();
        get_error().map(|_| {
            assert!(!ptr.is_null());
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        })
    }
}
