use std::ffi::CString;
use std::ptr;
use std::ops::Deref;
use std::hash::{ Hash, Hasher };

use enum_primitive::FromPrimitive;

use ffi;
use Glfw;
use Result;
use Monitor;
use Image;
use WindowAttribute;
use SetWindowAttribute;
use ContextCreationApi;
use ContextRobustness;
use OpenGlProfile;
use ClientApi;
use WindowBorder;
use InputMode;
use SetInputMode;
use CursorMode;
use KeyCode;
use MouseButton;
use Cursor;
use callbacks::*;
use util::*;
use get_error;

pub struct Window<'a> {
    pub(crate) ptr: *mut ffi::GLFWwindow,
    shared: SharedWindow,
    glfw: Option<&'a Glfw>
}

impl<'a> Drop for Window<'a> {
    fn drop(&mut self) {
        if let Some(glfw) = self.glfw {
            glfw.destroy_window(self.ptr);
        }
    }
}

impl<'a> Eq for Window<'a> {}
impl<'a> PartialEq for Window<'a> {
    fn eq(&self, other: &Window<'a>) -> bool {
        self.shared() == other.shared()
    }
}

impl<'a> Hash for Window<'a> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.shared().hash(hasher);
    }
}

impl<'a> Window<'a> {
    pub(crate) fn init(glfw: Option<&'a Glfw>, ptr: *mut ffi::GLFWwindow) -> Self {
        init_callbacks(ptr);
        Window {
            ptr: ptr,
            shared: SharedWindow(ptr),
            glfw: glfw,
        }
    }

    pub fn shared(&self) -> &SharedWindow {
        &self.shared
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
        let icons: Vec<_> = icons.iter().map(|i| i.as_glfw_image()).collect();
        unsafe { ffi::glfwSetWindowIcon(self.ptr, icons.len() as i32, icons.as_ptr()) };
        get_error()
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

    pub fn resize(&self, border: WindowBorder) -> Result<()> {
        unsafe { ffi::glfwResizeWindow(self.ptr, border as i32) };
        get_error()
    }

    pub fn drag(&self) -> Result<()> {
        unsafe { ffi::glfwDragWindow(self.ptr) };
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
        unsafe {
            ffi::glfwGetWindowMonitor(self.ptr).as_mut().map(|p| Monitor::create_from(p))
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

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gacccb29947ea4b16860ebef42c2cb9337
    pub fn get_attribute(&self, attrib: WindowAttribute) -> Result<bool> {
        let r = unsafe { ffi::glfwGetWindowAttrib(self.ptr, attrib as i32) };
        get_error().map(|_| cint_to_bool(r))
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gacccb29947ea4b16860ebef42c2cb9337
    pub fn get_client_api_attribute(&self) -> Result<ClientApi> {
        let r = unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::GLFW_CLIENT_API) };
        get_error().map(|_| ClientApi::from_i32(r).unwrap())
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gacccb29947ea4b16860ebef42c2cb9337
    pub fn get_context_creation_api_attribute(&self) -> Result<ContextCreationApi> {
        let r = unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::GLFW_CONTEXT_CREATION_API) };
        get_error().map(|_| ContextCreationApi::from_i32(r).unwrap())
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gacccb29947ea4b16860ebef42c2cb9337
    pub fn get_context_version_attribute(&self) -> Result<(i32, i32, i32)> {
        let t = unsafe {(
            ffi::glfwGetWindowAttrib(self.ptr, ffi::GLFW_CONTEXT_VERSION_MAJOR),
            ffi::glfwGetWindowAttrib(self.ptr, ffi::GLFW_CONTEXT_VERSION_MINOR),
            ffi::glfwGetWindowAttrib(self.ptr, ffi::GLFW_CONTEXT_REVISION),
        )};
        get_error().map(|_| t)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gacccb29947ea4b16860ebef42c2cb9337
    pub fn get_opengl_profile_attribute(&self) -> Result<OpenGlProfile> {
        let r = unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::GLFW_OPENGL_PROFILE) };
        get_error().map(|_| OpenGlProfile::from_i32(r).unwrap())
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gacccb29947ea4b16860ebef42c2cb9337
    pub fn get_context_robustness_attribute(&self) -> Result<ContextRobustness> {
        let r = unsafe { ffi::glfwGetWindowAttrib(self.ptr, ffi::GLFW_CONTEXT_ROBUSTNESS) };
        get_error().map(|_| ContextRobustness::from_i32(r).unwrap())
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#gace2afda29b4116ec012e410a6819033e
    pub fn set_attribute(&self, attrib: SetWindowAttribute) -> Result<()> {
        use SetWindowAttribute::*;
        unsafe { match attrib {
            Resizable(v) =>
                    ffi::glfwSetWindowAttrib(self.ptr, ffi::GLFW_RESIZABLE, bool_to_cint(v)),
            Decorated(v) =>
                    ffi::glfwSetWindowAttrib(self.ptr, ffi::GLFW_DECORATED, bool_to_cint(v)),
            Floating(v) =>
                    ffi::glfwSetWindowAttrib(self.ptr, ffi::GLFW_FLOATING, bool_to_cint(v)),
            AutoIconify(v) =>
                    ffi::glfwSetWindowAttrib(self.ptr, ffi::GLFW_AUTO_ICONIFY, bool_to_cint(v)),
        } }
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaf5b859dbe19bdf434e42695ea45cc5f4
    pub fn get_input_mode(&self, mode: InputMode) -> bool {
        cint_to_bool(unsafe { ffi::glfwGetInputMode(self.ptr, mode as i32) })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaf5b859dbe19bdf434e42695ea45cc5f4
    pub fn get_cursor_mode(&self) -> CursorMode {
        CursorMode::from_i32(unsafe { ffi::glfwGetInputMode(self.ptr, ffi::GLFW_CURSOR) }).unwrap()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gaa92336e173da9c8834558b54ee80563b
    pub fn set_input_mode(&self, mode: SetInputMode) -> Result<()> {
        use SetInputMode::*;
        unsafe { match mode {
            Cursor(c) => ffi::glfwSetInputMode(self.ptr, ffi::GLFW_CURSOR, c as i32),
            StickyKeys(v) => ffi::glfwSetInputMode(
                self.ptr, ffi::GLFW_STICKY_KEYS, bool_to_cint(v)
            ),
            StickyMouseButtons(v) => ffi::glfwSetInputMode(
                self.ptr, ffi::GLFW_STICKY_MOUSE_BUTTONS, bool_to_cint(v)
            ),
            LockKeyMods(v) => ffi::glfwSetInputMode(
                self.ptr, ffi::GLFW_LOCK_KEY_MODS, bool_to_cint(v)
            )
        } }
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gadd341da06bc8d418b4dc3a3518af9ad2
    pub fn get_key(&self, keycode: KeyCode) -> bool {
        cint_to_bool(unsafe { ffi::glfwGetKey(self.ptr, keycode as i32) })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gac1473feacb5996c01a7a5a33b5066704
    pub fn get_mouse_button(&self, button: MouseButton) -> bool {
        cint_to_bool(unsafe { ffi::glfwGetMouseButton(self.ptr, button as i32) })
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga01d37b6c40133676b9cea60ca1d7c0cc
    pub fn get_cursor_pos(&self) -> (f64, f64) {
        let mut p = (0.0, 0.0);
        unsafe { ffi::glfwGetCursorPos(self.ptr, &mut p.0, &mut p.1) };
        p
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#ga04b03af936d906ca123c8f4ee08b39e7
    pub fn set_cursor_pos(&self, x: f64, y: f64) -> Result<()> {
        unsafe { ffi::glfwSetCursorPos(self.ptr, x, y) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__input.html#gad3b4f38c8d5dae036bc8fa959e18343e
    pub fn set_cursor(&self, cursor: Option<&Cursor>) -> Result<()> {
        unsafe { ffi::glfwSetCursor(self.ptr, cursor.map_or(ptr::null_mut(), |c| c.ptr)) };
        get_error()
    }
}

impl<'a> Deref for Window<'a> {
    type Target = SharedWindow;

    fn deref(&self) -> &SharedWindow {
        &self.shared
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct SharedWindow(*mut ffi::GLFWwindow);
unsafe impl Sync for SharedWindow {}

impl SharedWindow {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga24e02fbfefbb81fc45320989f8140ab5
    pub fn should_close(&self) -> bool {
        let v = unsafe { ffi::glfwWindowShouldClose(self.0) };
        cint_to_bool(v)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga49c449dde2a6f87d996f4daaa09d6708
    pub fn set_should_close(&self, v: bool) {
        unsafe { ffi::glfwSetWindowShouldClose(self.0, bool_to_cint(v)) };
    }

    /// [GLFW Reference][glfw]
    /// 
    /// The EGL API requires that the context for the window be current.
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__window.html#ga15a5a1ee5b3c2ca6b15ca209a12efd14
    pub fn swap_buffers(&self) -> Result<()> {
        unsafe { ffi::glfwSwapBuffers(self.0) };
        get_error()
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__context.html#ga1c04dc242268f827290fe40aa1c91157
    pub unsafe fn make_context_current(&self) -> Result<()> {
        ffi::glfwMakeContextCurrent(self.0);
        get_error()
    }
}

#[cfg(all(
    feature = "expose-win32",
    target_os = "windows"
))]
impl SharedWindow {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#gafe5079aa79038b0079fc09d5f0a8e667
    pub unsafe fn get_win32_window(&self) -> ::winapi::shared::windef::HWND {
        ffi::win32::glfwGetWin32Window(self.0)
    }
}

#[cfg(all(
    feature = "expose-wgl",
    target_os = "windows"
))]
impl SharedWindow {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#gadc4010d91d9cc1134d040eeb1202a143
    pub unsafe fn get_wgl_context(&self) -> ::winapi::shared::windef::HGLRC {
        ffi::wgl::glfwGetWGLContext(self.0)
    }
}

#[cfg(all(
    feature = "expose-x11",
    any(target_os="linux", target_os="freebsd", target_os="dragonfly")
))]
impl SharedWindow {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#ga90ca676322740842db446999a1b1f21d
    pub unsafe fn get_x11_window(&self) -> ::x11::xlib::Window {
        ffi::x11::glfwGetX11Window(self.0)
    }
}

#[cfg(all(
    feature = "expose-glx",
    any(target_os="linux", target_os="freebsd", target_os="dragonfly")
))]
impl SharedWindow {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#ga90ca676322740842db446999a1b1f21d
    pub unsafe fn get_glx_context(&self) -> ::x11::glx::GLXContext {
        ffi::glx::glfwGetGLXContext(self.0)
    }

    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__native.html#ga90ca676322740842db446999a1b1f21d
    pub unsafe fn get_glx_window(&self) -> ::x11::glx::GLXWindow {
        ffi::glx::glfwGetGLXWindow(self.0)
    }
}