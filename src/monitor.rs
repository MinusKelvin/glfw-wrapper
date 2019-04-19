use std::fmt;
use std::rc::{ Rc, Weak };
use std::cell::{ Cell, RefCell };
use std::ffi::CStr;
use std::slice;

use ffi;
use get_error;
use Result;
use util::*;

thread_local! {
    /// Global mutable reference to list of possibly active monitor references.
    /// 
    /// The lifetime of `GLFWmonitor`s is unenforcable, so we take the RefCell approach and panic
    /// whenever you do something bad, like attempt to do anything with a disconnected monitor.
    /// 
    /// The only things that can affect this list live on the main thread, so the library will only
    /// every access the list for a single thread.
    static MONITORS: RefCell<Vec<Weak<Cell<Option<*mut ffi::GLFWmonitor>>>>>
            = RefCell::new(Vec::new());
}

pub(crate) fn invalidate_all_monitors() {
    MONITORS.with(|v| {
        let mut v = v.borrow_mut();
        for cell in v.drain(..).filter_map(|w| w.upgrade()) {
            cell.set(None);
        }
    })
}

pub(crate) fn invalidate_monitor(ptr: *mut ffi::GLFWmonitor) {
    MONITORS.with(|v| {
        let mut v = v.borrow_mut();
        v.retain(|w| match w.upgrade() {
            Some(c) => {
                if let Some(p) = c.get() {
                    if p != ptr {
                        true
                    } else {
                        c.set(None);
                        false
                    }
                } else {
                    false
                }
            },
            None => false
        })
    })
}

pub type VideoMode = ffi::GLFWvidmode;

pub struct DisconnectedMonitor(pub(crate) *mut ffi::GLFWmonitor);

impl fmt::Debug for DisconnectedMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DisconnectedMonitor {{ ... }}")
    }
}

impl DisconnectedMonitor {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga79a34ee22ff080ca954a9663e4679daf
    pub fn get_name(&self) -> String {
        let name = unsafe { CStr::from_ptr(ffi::glfwGetMonitorName(self.0)) };
        name.to_string_lossy().into_owned()
    }
}

pub struct Monitor(Rc<Cell<Option<*mut ffi::GLFWmonitor>>>);

impl fmt::Debug for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monitor {{ ... }}")
    }
}

impl Monitor {
    pub(crate) fn create_from(ptr: *mut ffi::GLFWmonitor) -> Monitor {
        let this = Monitor(Rc::new(Cell::new(Some(ptr))));
        MONITORS.with(|v| {
            let mut v = v.borrow_mut();
            v.push(Rc::downgrade(&this.0));
            v.retain(|w| w.upgrade().is_some());
        });
        this
    }

    pub(crate) fn get_ptr(&self) -> *mut ffi::GLFWmonitor {
        self.0.get().unwrap_or_else(|| panic!("Monitor outlived its lifetime"))
    }

    pub fn is_valid(&self) -> bool {
        self.0.get().is_some()
    }

    pub fn is_same_as(&self, other: &Monitor) -> bool {
        self.0.get().and_then(|p1| other.0.get().map(|p2| p1 == p2)).unwrap_or(false)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga102f54e7acc9149edbcf0997152df8c9
    pub fn get_pos(&self) -> Result<(i32, i32)> {
        let mut p = (0, 0);
        unsafe { ffi::glfwGetMonitorPos(self.get_ptr(), &mut p.0, &mut p.1) };
        get_error().map(|_| p)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: https://www.glfw.org/docs/3.3/group__monitor.html#ga7387a3bdb64bfe8ebf2b9e54f5b6c9d0
    pub fn get_workarea(
        &self,
        xpos: Option<&mut i32>,
        ypos: Option<&mut i32>,
        width: Option<&mut i32>,
        height: Option<&mut i32>
    ) -> Result<()> {
        unsafe {
            ffi::glfwGetMonitorWorkarea(self.get_ptr(), xpos.ptr(), ypos.ptr(), width.ptr(), height.ptr());
        }
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga7d8bffc6c55539286a6bd20d32a8d7ea
    pub fn get_physical_size(&self) -> (i32, i32) {
        let mut s = (0, 0);
        unsafe { ffi::glfwGetMonitorPhysicalSize(self.get_ptr(), &mut s.0, &mut s.1) };
        s
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#gad3152e84465fa620b601265ebfcdb21b
    pub fn get_content_scale(&self) -> Result<(f32, f32)> {
        let mut s = (0.0, 0.0);
        unsafe { ffi::glfwGetMonitorContentScale(self.get_ptr(), &mut s.0, &mut s.1) };
        get_error().map(|_| s)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga79a34ee22ff080ca954a9663e4679daf
    pub fn get_name(&self) -> String {
        let name = unsafe { CStr::from_ptr(ffi::glfwGetMonitorName(self.get_ptr())) };
        name.to_string_lossy().into_owned()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga820b0ce9a5237d645ea7cbb4bd383458
    pub fn get_video_modes(&self) -> Result<Vec<VideoMode>> {
        let raw = unsafe {
            let mut count = 0;
            let ptr = ffi::glfwGetVideoModes(self.get_ptr(), &mut count);
            slice::from_raw_parts(ptr, count as usize)
        };
        get_error().map(|_| raw.into())
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#gafc1bb972a921ad5b3bd5d63a95fc2d52
    pub fn get_video_mode(&self) -> Result<VideoMode> {
        let mode = unsafe { ffi::glfwGetVideoMode(self.get_ptr()) };
        get_error().map(|_| unsafe { assert!(!mode.is_null()); *mode })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga6ac582625c990220785ddd34efa3169a
    pub fn set_gamma(&self, gamma: f32) -> Result<()> {
        unsafe { ffi::glfwSetGamma(self.get_ptr(), gamma) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#gab7c41deb2219bde3e1eb756ddaa9ec80
    pub fn get_gamma_ramp(&self) -> Result<GammaRamp> {
        let ramp = unsafe { ffi::glfwGetGammaRamp(self.get_ptr()) };
        get_error().map(|_| {
            assert!(!ramp.is_null());
            let ramp = unsafe { &*ramp };
            let (red, green, blue) = unsafe {(
                slice::from_raw_parts(ramp.red, ramp.size as usize),
                slice::from_raw_parts(ramp.green, ramp.size as usize),
                slice::from_raw_parts(ramp.blue, ramp.size as usize),
            )};
            GammaRamp(red.iter().zip(green).zip(blue).map(|((r, g), b)| (*r, *g, *b)).collect())
        })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__monitor.html#ga583f0ffd0d29613d8cd172b996bbf0dd
    pub fn set_gamma_ramp(&self, ramp: &GammaRamp) -> Result<()> {
        let mut red = vec![];
        let mut green = vec![];
        let mut blue = vec![];
        for (r, g, b) in &ramp.0 {
            red.push(*r);
            green.push(*g);
            blue.push(*b);
        }
        unsafe {
            let ramp = ffi::GLFWgammaramp {
                red: red.as_mut_ptr(),
                green: green.as_mut_ptr(),
                blue: blue.as_mut_ptr(),
                size: ramp.0.len() as u32
            };
            ffi::glfwSetGammaRamp(self.get_ptr(), &ramp);
        }
        get_error()
    }
}

pub struct GammaRamp(pub Vec<(u16, u16, u16)>);

#[cfg(all(
    feature = "expose-win32",
    target_os = "windows"
))]
impl Monitor {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#gac84f63a3f9db145b9435e5e0dbc4183d
    pub unsafe fn get_win32_adapter(&self) -> Option<String> {
        let ptr = ffi::win32::glfwGetWin32Adapter(self.get_ptr());
        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#gac408b09a330749402d5d1fa1f5894dd9
    pub unsafe fn get_win32_monitor(&self) -> Option<String> {
        let ptr = ffi::win32::glfwGetWin32Monitor(self.get_ptr());
        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    }
}

#[cfg(all(
    feature = "expose-x11",
    any(target_os="linux", target_os="freebsd", target_os="dragonfly")
))]
impl Monitor {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#ga088fbfa80f50569402b41be71ad66e40
    pub unsafe fn get_x11_adapter(&self) -> ::x11::xrandr::RRCrtc {
        ffi::x11::glfwGetX11Adapter(self.get_ptr())
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#gab2f8cc043905e9fa9b12bfdbbcfe874c
    pub unsafe fn get_x11_monitor(&self) -> ::x11::xrandr::RROutput {
        ffi::x11::glfwGetX11Monitor(self.get_ptr())
    }
}