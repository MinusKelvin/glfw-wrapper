use std::marker::PhantomData;
use std::ffi::CString;
use std::ptr;

use libc::c_void;

use ffi;
use Glfw;
use Result;
use Monitor;
use Image;
use util::*;
use get_error;

pub struct Window<'a> {
    pub(crate) ptr: *mut ffi::GLFWwindow,
    _phantom: PhantomData<&'a Glfw>
}

impl<'a> Drop for Window<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::glfwDestroyWindow(self.ptr);
            Box::from_raw(ffi::glfwGetWindowUserPointer(self.ptr) as *mut WindowCallbacks);
        }
    }
}

impl<'a> Window<'a> {
    pub(crate) fn init(ptr: *mut ffi::GLFWwindow) -> Window<'a> {
        unsafe {
            ffi::glfwSetWindowUserPointer(ptr,
                    Box::into_raw(Box::new(WindowCallbacks::default())) as *mut c_void)
        }
        Window {
            ptr: ptr,
            _phantom: PhantomData
        }
    }

    pub fn shared<'b>(&'b self) -> SharedWindow<'a, 'b> {
        SharedWindow(self)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga5d877f09e968cef7a360b513306f17ff
    pub fn set_title(&self, title: &str) -> Result<()> {
        let title = CString::new(title).unwrap();
        unsafe { ffi::glfwSetWindowTitle(self.ptr, title.as_ptr()) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gadd7ccd39fe7a7d1f0904666ae5932dc5
    pub fn set_icon(&self, icons: &[Image]) -> Result<()> {
        unimplemented!()
        // unsafe { ffi::glfwSetWindowIcon(...) };
        // get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga73cb526c000876fd8ddf571570fdb634
    pub fn get_pos(&self) -> Result<(i32, i32)> {
        let mut p = (0, 0);
        unsafe { ffi::glfwGetWindowPos(self.ptr, &mut p.0, &mut p.1) };
        get_error().map(|_| p)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga1abb6d690e8c88e0c8cd1751356dbca8
    pub fn set_pos(&self, x: i32, y: i32) -> Result<()> {
        unsafe { ffi::glfwSetWindowPos(self.ptr, x, y) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gaeea7cbc03373a41fb51cfbf9f2a5d4c6
    pub fn get_window_size(&self) -> Result<(i32, i32)> {
        let mut p = (0, 0);
        unsafe { ffi::glfwGetWindowSize(self.ptr, &mut p.0, &mut p.1) };
        get_error().map(|_| p)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gac314fa6cec7d2d307be9963e2709cc90
    pub fn set_size_limits(
        &self,
        min_width: Option<i32>,
        min_height: Option<i32>,
        max_width: Option<i32>,
        max_height: Option<i32>
    ) -> Result<()> {
        unsafe { ffi::glfwSetWindowSizeLimits(
            self.ptr,
            min_width.or_dont_care(),
            min_height.or_dont_care(),
            max_width.or_dont_care(),
            max_height.or_dont_care()
        )}
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga72ac8cb1ee2e312a878b55153d81b937
    pub fn set_aspect_ratio(
        &self,
        numerator: Option<i32>,
        denominator: Option<i32>
    ) -> Result<()> {
        unsafe { ffi::glfwSetWindowAspectRatio(
            self.ptr,
            numerator.or_dont_care(),
            denominator.or_dont_care()
        )}
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga371911f12c74c504dd8d47d832d095cb
    pub fn set_size(&self, width: i32, height: i32) -> Result<()> {
        unsafe { ffi::glfwSetWindowSize(self.ptr, width, height) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga0e2637a4161afb283f5300c7f94785c9
    pub fn get_framebuffer_size(&self) -> Result<(i32, i32)> {
        let mut s = (0, 0);
        unsafe { ffi::glfwGetFramebufferSize(self.ptr, &mut s.0, &mut s.1) };
        get_error().map(|_| s)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// This uses references instead of returning a tuple because I wanted to avoid confusion as
    /// there is no convention for the order of left/right/top/bottom parameters and I didn't want
    /// to make and name a whole struct for this one function. Subject to change.
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga1a9fd382058c53101b21cf211898f1f1
    pub fn get_frame_size(
        &self,
        left: Option<&mut i32>,
        top: Option<&mut i32>,
        right: Option<&mut i32>,
        bottom: Option<&mut i32>
    ) -> Result<()> {
        unsafe { ffi::glfwGetWindowFrameSize(
            self.ptr,
            left.ptr(),
            top.ptr(),
            right.ptr(),
            bottom.ptr()
        )}
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gaf5d31de9c19c4f994facea64d2b3106c
    pub fn get_content_scale(&self) -> Result<(f32, f32)> {
        let mut s = (0.0, 0.0);
        unsafe { ffi::glfwGetWindowContentScale(self.ptr, &mut s.0, &mut s.1) };
        get_error().map(|_| s)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gad09f0bd7a6307c4533b7061828480a84
    pub fn get_opacity(&self) -> Result<f32> {
        let v = unsafe { ffi::glfwGetWindowOpacity(self.ptr) };
        get_error().map(|_| v)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gac31caeb3d1088831b13d2c8a156802e9
    pub fn set_opacity(&self, opacity: f32) -> Result<()> {
        unsafe { ffi::glfwSetWindowOpacity(self.ptr, opacity) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga1bb559c0ebaad63c5c05ad2a066779c4
    pub fn iconify(&self) -> Result<()> {
        unsafe { ffi::glfwIconifyWindow(self.ptr) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga52527a5904b47d802b6b4bb519cdebc7
    pub fn restore(&self) -> Result<()> {
        unsafe { ffi::glfwRestoreWindow(self.ptr) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga3f541387449d911274324ae7f17ec56b
    pub fn maximize(&self) -> Result<()> {
        unsafe { ffi::glfwMaximizeWindow(self.ptr) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga61be47917b72536a148300f46494fc66
    pub fn show(&self) -> Result<()> {
        unsafe { ffi::glfwShowWindow(self.ptr) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga49401f82a1ba5f15db5590728314d47c
    pub fn hide(&self) -> Result<()> {
        unsafe { ffi::glfwHideWindow(self.ptr) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga873780357abd3f3a081d71a40aae45a1
    pub fn focus(&self) -> Result<()> {
        unsafe { ffi::glfwFocusWindow(self.ptr) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga2f8d59323fc4692c1d54ba08c863a703
    pub fn request_attention(&self) -> Result<()> {
        unsafe { ffi::glfwRequestWindowAttention(self.ptr) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gaeac25e64789974ccbe0811766bd91a16
    pub fn get_monitor(&self) -> Option<Monitor> {
        let ptr = unsafe { ffi::glfwGetWindowMonitor(self.ptr) };
        if ptr.is_null() {
            None
        } else {
            Some(Monitor::create_from(ptr))
        }
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga81c76c418af80a1cce7055bccb0ae0a7
    pub fn set_monitor(
        &self,
        monitor: Option<&Monitor>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        refresh_rate: Option<i32>
    ) -> Result<()> {
        unsafe { ffi::glfwSetWindowMonitor(
            self.ptr,
            monitor.map_or(ptr::null_mut(), |m| m.get_ptr()),
            x,
            y,
            width,
            height,
            refresh_rate.or_dont_care()
        )}
        get_error()
    }

    // TODO glfwGetWindowAttrib
    // TODO glfwSetWindowAttrib

    // TODO callbacks
}

pub struct SharedWindow<'a: 'b, 'b>(&'b Window<'a>);
unsafe impl<'a: 'b, 'b> Send for SharedWindow<'a, 'b> {}
unsafe impl<'a: 'b, 'b> Sync for SharedWindow<'a, 'b> {}

impl<'a: 'b, 'b> SharedWindow<'a, 'b> {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga24e02fbfefbb81fc45320989f8140ab5
    pub fn should_close(&self) -> bool {
        let v = unsafe { ffi::glfwWindowShouldClose(self.0.ptr) };
        cint_to_bool(v)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga49c449dde2a6f87d996f4daaa09d6708
    pub fn set_should_close(&self, v: bool) {
        unsafe { ffi::glfwSetWindowShouldClose(self.0.ptr, bool_to_cint(v)) };
    }

    /// [GLFW Reference][glfw]
    /// 
    /// The EGL API requires that the context for the window be current.
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga15a5a1ee5b3c2ca6b15ca209a12efd14
    pub fn swap_buffers(&self) -> Result<()> {
        unsafe { ffi::glfwSwapBuffers(self.0.ptr) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__context.html#ga1c04dc242268f827290fe40aa1c91157
    pub unsafe fn make_context_current(&self) -> Result<()> {
        ffi::glfwMakeContextCurrent(self.0.ptr);
        get_error()
    }
}

#[derive(Default)]
struct WindowCallbacks {

}