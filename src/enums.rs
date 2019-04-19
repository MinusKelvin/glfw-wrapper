use std::os::raw::{ c_int, c_uchar };
use ffi;

enum_from_primitive! {
    /// Named keys.
    /// 
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__keys.html
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum KeyCode {
        Space = ffi::GLFW_KEY_SPACE,
        Apostrophe = ffi::GLFW_KEY_APOSTROPHE,
        Comma = ffi::GLFW_KEY_COMMA,
        Minus = ffi::GLFW_KEY_MINUS,
        Period = ffi::GLFW_KEY_PERIOD,
        Slash = ffi::GLFW_KEY_SLASH,
        Zero = ffi::GLFW_KEY_0,
        One = ffi::GLFW_KEY_1,
        Two = ffi::GLFW_KEY_2,
        Three = ffi::GLFW_KEY_3,
        Four = ffi::GLFW_KEY_4,
        Five = ffi::GLFW_KEY_5,
        Six = ffi::GLFW_KEY_6,
        Seven = ffi::GLFW_KEY_7,
        Eight = ffi::GLFW_KEY_8,
        Nine = ffi::GLFW_KEY_9,
        Semicolon = ffi::GLFW_KEY_SEMICOLON,
        Equal = ffi::GLFW_KEY_EQUAL,
        A = ffi::GLFW_KEY_A,
        B = ffi::GLFW_KEY_B,
        C = ffi::GLFW_KEY_C,
        D = ffi::GLFW_KEY_D,
        E = ffi::GLFW_KEY_E,
        F = ffi::GLFW_KEY_F,
        G = ffi::GLFW_KEY_G,
        H = ffi::GLFW_KEY_H,
        I = ffi::GLFW_KEY_I,
        J = ffi::GLFW_KEY_J,
        K = ffi::GLFW_KEY_K,
        L = ffi::GLFW_KEY_L,
        M = ffi::GLFW_KEY_M,
        N = ffi::GLFW_KEY_N,
        O = ffi::GLFW_KEY_O,
        P = ffi::GLFW_KEY_P,
        Q = ffi::GLFW_KEY_Q,
        R = ffi::GLFW_KEY_R,
        S = ffi::GLFW_KEY_S,
        T = ffi::GLFW_KEY_T,
        U = ffi::GLFW_KEY_U,
        V = ffi::GLFW_KEY_V,
        W = ffi::GLFW_KEY_W,
        X = ffi::GLFW_KEY_X,
        Y = ffi::GLFW_KEY_Y,
        Z = ffi::GLFW_KEY_Z,
        LeftBracket = ffi::GLFW_KEY_LEFT_BRACKET,
        Backslash = ffi::GLFW_KEY_BACKSLASH,
        RightBracket = ffi::GLFW_KEY_RIGHT_BRACKET,
        GraveAccent = ffi::GLFW_KEY_GRAVE_ACCENT,
        World1 = ffi::GLFW_KEY_WORLD_1,
        World2 = ffi::GLFW_KEY_WORLD_2,

        Escape = ffi::GLFW_KEY_ESCAPE,
        Enter = ffi::GLFW_KEY_ENTER,
        Tab = ffi::GLFW_KEY_TAB,
        Backspace = ffi::GLFW_KEY_BACKSPACE,
        Insert = ffi::GLFW_KEY_INSERT,
        Delete = ffi::GLFW_KEY_DELETE,
        Right = ffi::GLFW_KEY_RIGHT,
        Left = ffi::GLFW_KEY_LEFT,
        Down = ffi::GLFW_KEY_DOWN,
        Up = ffi::GLFW_KEY_UP,
        PageUp = ffi::GLFW_KEY_PAGE_UP,
        PageDown = ffi::GLFW_KEY_PAGE_DOWN,
        Home = ffi::GLFW_KEY_HOME,
        End = ffi::GLFW_KEY_END,
        CapsLock = ffi::GLFW_KEY_CAPS_LOCK,
        ScrollLock = ffi::GLFW_KEY_SCROLL_LOCK,
        NumLock = ffi::GLFW_KEY_NUM_LOCK,
        PrintScreen = ffi::GLFW_KEY_PRINT_SCREEN,
        Pause = ffi::GLFW_KEY_PAUSE,
        F1 = ffi::GLFW_KEY_F1,
        F2 = ffi::GLFW_KEY_F2,
        F3 = ffi::GLFW_KEY_F3,
        F4 = ffi::GLFW_KEY_F4,
        F5 = ffi::GLFW_KEY_F5,
        F6 = ffi::GLFW_KEY_F6,
        F7 = ffi::GLFW_KEY_F7,
        F8 = ffi::GLFW_KEY_F8,
        F9 = ffi::GLFW_KEY_F9,
        F10 = ffi::GLFW_KEY_F10,
        F11 = ffi::GLFW_KEY_F11,
        F12 = ffi::GLFW_KEY_F12,
        F13 = ffi::GLFW_KEY_F13,
        F14 = ffi::GLFW_KEY_F14,
        F15 = ffi::GLFW_KEY_F15,
        F16 = ffi::GLFW_KEY_F16,
        F17 = ffi::GLFW_KEY_F17,
        F18 = ffi::GLFW_KEY_F18,
        F19 = ffi::GLFW_KEY_F19,
        F20 = ffi::GLFW_KEY_F20,
        F21 = ffi::GLFW_KEY_F21,
        F22 = ffi::GLFW_KEY_F22,
        F23 = ffi::GLFW_KEY_F23,
        F24 = ffi::GLFW_KEY_F24,
        F25 = ffi::GLFW_KEY_F25,
        Kp0 = ffi::GLFW_KEY_KP_0,
        Kp1 = ffi::GLFW_KEY_KP_1,
        Kp2 = ffi::GLFW_KEY_KP_2,
        Kp3 = ffi::GLFW_KEY_KP_3,
        Kp4 = ffi::GLFW_KEY_KP_4,
        Kp5 = ffi::GLFW_KEY_KP_5,
        Kp6 = ffi::GLFW_KEY_KP_6,
        Kp7 = ffi::GLFW_KEY_KP_7,
        Kp8 = ffi::GLFW_KEY_KP_8,
        Kp9 = ffi::GLFW_KEY_KP_9,
        KpDecimal = ffi::GLFW_KEY_KP_DECIMAL,
        KpDivide = ffi::GLFW_KEY_KP_DIVIDE,
        KpMultiply = ffi::GLFW_KEY_KP_MULTIPLY,
        KpSubtract = ffi::GLFW_KEY_KP_SUBTRACT,
        KpAdd = ffi::GLFW_KEY_KP_ADD,
        KpEnter = ffi::GLFW_KEY_KP_ENTER,
        KpEqual = ffi::GLFW_KEY_KP_EQUAL,
        LeftShift = ffi::GLFW_KEY_LEFT_SHIFT,
        LeftControl = ffi::GLFW_KEY_LEFT_CONTROL,
        LeftAlt = ffi::GLFW_KEY_LEFT_ALT,
        LeftSuper = ffi::GLFW_KEY_LEFT_SUPER,
        RightShift = ffi::GLFW_KEY_RIGHT_SHIFT,
        RightControl = ffi::GLFW_KEY_RIGHT_CONTROL,
        RightAlt = ffi::GLFW_KEY_RIGHT_ALT,
        RightSuper = ffi::GLFW_KEY_RIGHT_SUPER,
        Menu = ffi::GLFW_KEY_MENU
    }
}

/// Keyboard key
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Key {
    Named(KeyCode),
    Unnamed(i32)
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum CursorMode {
        Normal = ffi::GLFW_CURSOR_NORMAL,
        Hidden = ffi::GLFW_CURSOR_HIDDEN,
        Disabled = ffi::GLFW_CURSOR_DISABLED
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum MouseButton {
        Left = ffi::GLFW_MOUSE_BUTTON_LEFT,
        Right = ffi::GLFW_MOUSE_BUTTON_RIGHT,
        Middle = ffi::GLFW_MOUSE_BUTTON_MIDDLE,
        Four = ffi::GLFW_MOUSE_BUTTON_4,
        Five = ffi::GLFW_MOUSE_BUTTON_5,
        Six = ffi::GLFW_MOUSE_BUTTON_6,
        Seven = ffi::GLFW_MOUSE_BUTTON_7,
        Eight = ffi::GLFW_MOUSE_BUTTON_8,
    }
}

enum_from_primitive! {
    /// Joystick ID
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum Joystick {
        One = ffi::GLFW_JOYSTICK_1,
        Two = ffi::GLFW_JOYSTICK_2,
        Three = ffi::GLFW_JOYSTICK_3,
        Four = ffi::GLFW_JOYSTICK_4,
        Five = ffi::GLFW_JOYSTICK_5,
        Six = ffi::GLFW_JOYSTICK_6,
        Seven = ffi::GLFW_JOYSTICK_7,
        Eight = ffi::GLFW_JOYSTICK_8,
        Nine = ffi::GLFW_JOYSTICK_9,
        Ten = ffi::GLFW_JOYSTICK_10,
        Eleven = ffi::GLFW_JOYSTICK_11,
        Twelve = ffi::GLFW_JOYSTICK_12,
        Thirteen = ffi::GLFW_JOYSTICK_13,
        Fourteen = ffi::GLFW_JOYSTICK_14,
        Fifteen = ffi::GLFW_JOYSTICK_15,
        Sixteen = ffi::GLFW_JOYSTICK_16,
    }
}

enum_from_primitive! {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__errors.html
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum ErrorKind {
        NoCurrentContext = ffi::GLFW_NO_CURRENT_CONTEXT,
        InvalidValue = ffi::GLFW_INVALID_VALUE,
        OutOfMemory = ffi::GLFW_OUT_OF_MEMORY,
        ApiUnavailable = ffi::GLFW_API_UNAVAILABLE,
        VersionUnavailable = ffi::GLFW_VERSION_UNAVAILABLE,
        PlatformError = ffi::GLFW_PLATFORM_ERROR,
        FormatUnavailable = ffi::GLFW_FORMAT_UNAVAILABLE,
        NoWindowContext = ffi::GLFW_NO_WINDOW_CONTEXT
    }
}

enum_from_primitive! {
    /// [GLFW reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__shapes.html
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum StandardCursorShape {
        Arrow = ffi::GLFW_ARROW_CURSOR,
        IBeam = ffi::GLFW_IBEAM_CURSOR,
        Crosshair = ffi::GLFW_CROSSHAIR_CURSOR,
        Hand = ffi::GLFW_HAND_CURSOR,
        HResize = ffi::GLFW_HRESIZE_CURSOR,
        VResize = ffi::GLFW_VRESIZE_CURSOR,
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug)]
    pub enum ClientApi {
        OpenGl = ffi::GLFW_OPENGL_API,
        OpenGlEs = ffi::GLFW_OPENGL_ES_API,
        NoApi = ffi::GLFW_NO_API
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug)]
    pub enum ContextRobustness {
        NoRobustness = ffi::GLFW_NO_ROBUSTNESS,
        NoResetNotification = ffi::GLFW_NO_RESET_NOTIFICATION,
        LoseContextOnReset = ffi::GLFW_LOSE_CONTEXT_ON_RESET
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug)]
    pub enum OpenGlProfile {
        Any = ffi::GLFW_OPENGL_ANY_PROFILE,
        Core = ffi::GLFW_OPENGL_CORE_PROFILE,
        Compatibility = ffi::GLFW_OPENGL_COMPAT_PROFILE
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug)]
    pub enum ContextReleaseBehavior {
        Any = ffi::GLFW_ANY_RELEASE_BEHAVIOR,
        Flush = ffi::GLFW_RELEASE_BEHAVIOR_FLUSH,
        None = ffi::GLFW_RELEASE_BEHAVIOR_NONE,
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug)]
    pub enum ContextCreationApi {
        Native = ffi::GLFW_NATIVE_CONTEXT_API,
        Egl = ffi::GLFW_EGL_CONTEXT_API,
        OsMesa = ffi::GLFW_OSMESA_CONTEXT_API
    }
}

/// GLFW window creation hints.
/// 
/// Some window hints are hard constraints. If the available capabilities cannot match the requested
/// hard constraints exactly, window and context creation will fail. Soft constraints are matched as
/// closely as possible, but the resulting window and context may differ from what was requested.
/// 
/// The following hints are hard constraints:
/// * [`stereo`](#structfield.stereo)
/// * [`double_buffer`](#structfield.double_buffer)
/// * [`client_api`](#structfield.client_api)
/// * [`context_creation_api`](#structfield.context_creation_api)
/// 
/// The following hints are hard constraints when requesting an OpenGL context, but ignored when
/// requesting an OpenGL ES context:
/// * [`opengl_forwad_compatible`](#structfield.opengl_forward_compatible)
/// * [`opengl_profile`](#structfield.opengl_profile)
/// 
/// # See Also
/// 
/// * [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#window_hints)
#[derive(Copy, Clone, Debug)]
pub struct WindowHints<'a> {
    /// Specifies whether the user will be able to resize the window.
    /// This hint is ignored for fullscreen an undecorated windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_RESIZABLE_hint)
    pub resizable: bool,
    /// Specifies whether the window will be shown when it is created.
    /// This hint is ignored for fullscreen windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_VISIBLE_hint)
    pub visible: bool,
    /// Specifies whether the window will have decorations, usually a system title bar, borders,
    /// close, maximise, and minimize buttons, etc.
    /// This hint is ignored for fullscreen windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_DECORATED_hint)
    pub decorated: bool,
    /// Specifies whether the window will be given input focus when created.
    /// This hint is ignored for fullscreen and initially hidden windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_FOCUSED_hint)
    pub focused: bool,
    /// Specifies whether the fullscreen window will automatically iconify the window when input
    /// focus is lost. This hint is ignored for windowed mode windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_AUTO_ICONIFY_hint)
    pub auto_iconify: bool,
    /// Specifies whether the window should be topmost or always-on-top. This is intended primarily
    /// for debugging purposes. This hint is ignored for fullscreen windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_FLOATING_hint)
    pub floating: bool,
    /// Specifies whether the window will be maximised when created.
    /// This hint is ignored for fullscreen windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_MAXIMIZED_hint)
    pub maximized: bool,
    /// Specifies whether the cursor should be centered over newly created fullscreen windows.
    /// This hint is ignored for windowed mode windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_CENTER_CURSOR_hint)
    pub center_cursor: bool,
    /// Specifies whether the window framebuffer should be transparent. If supported by the system,
    /// the alpha channel of the framebuffer will be used to combine the framebuffer with the
    /// background. This does not affect window decorations.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_TRANSPARENT_FRAMEBUFFER_hint)
    pub transparent_framebuffer: bool,
    // Missing from http://www.glfw.org/docs/3.3/window_guide.html#window_hints_values
    // FocusOnShow: bool,
    /// Specifies the desired bit depth of the default framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_RED_BITS)
    pub red_bits: Option<i32>,
    /// Specifies the desired bit depth of the default framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_GREEN_BITS)
    pub green_bits: Option<i32>,
    /// Specifies the desired bit depth of the default framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_BLUE_BITS)
    pub blue_bits: Option<i32>,
    /// Specifies the desired bit depth of the default framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_ALPHA_BITS)
    pub alpha_bits: Option<i32>,
    /// Specifies the desired bit depth of the default framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_DEPTH_BITS)
    pub depth_bits: Option<i32>,
    /// Specifies the desired bit depth of the default framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_STENCIL_BITS)
    pub stencil_bits: Option<i32>,
    /// Specifies the desired bit depth of the accumulation framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// Accumulation buffers are a legacy OpenGL feature and should not be used in new code.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_ACCUM_RED_BITS)
    pub accum_red_bits: Option<i32>,
    /// Specifies the desired bit depth of the accumulation framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// Accumulation buffers are a legacy OpenGL feature and should not be used in new code.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_ACCUM_GREEN_BITS)
    pub accum_green_bits: Option<i32>,
    /// Specifies the desired bit depth of the accumulation framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// Accumulation buffers are a legacy OpenGL feature and should not be used in new code.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_ACCUM_BLUE_BITS)
    pub accum_blue_bits: Option<i32>,
    /// Specifies the desired bit depth of the accumulation framebuffer. `None` indicates that the
    /// application has no preference.
    /// 
    /// Accumulation buffers are a legacy OpenGL feature and should not be used in new code.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_ACCUM_ALPHA_BITS)
    pub accum_alpha_bits: Option<i32>,
    /// Specifies the desired number of auxiliary buffers. `None` indicates that the application
    /// has no preference.
    /// 
    /// Auxiliary buffers are a legacy OpenGL feature and should not be used in new code.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_AUX_BUFFERS)
    pub auxiliary_buffers: Option<i32>,
    /// Specifies the desired number of samples to use for multisampling. `None` indicates that the
    /// application has no preference.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_SAMPLES)
    pub samples: Option<i32>,
    /// Specifies the desired refresh rate for fullscreen windows. `None` means the highest
    /// available refresh rate will be used. This hint is ignored for windowed mode windows.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_REFRESH_RATE)
    pub refresh_rate: Option<i32>,
    /// Specifies whether to use OpenGL stereoscopic rendering. This is a hard constraint.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_STEREO)
    pub stereo: bool,
    /// Specifies whether the framebuffer should be sRGB capable.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_SRGB_CAPABLE)
    pub srgb_capable: bool,
    /// Specifies whether the framebuffer should be double buffered. This is nearly always
    /// desireable. This is a hard constraint.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_DOUBLEBUFFER)
    pub double_buffer: bool,
    /// Specifies which client API to create the context for. This is a hard constraint.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_CLIENT_API_hint)
    pub client_api: ClientApi,
    /// Specifies which context creation API to use to create the context. If no client API is
    /// requested, this hint is ignored. This is a hard constraint.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_CONTEXT_CREATION_API_hint)
    pub context_creation_api: ContextCreationApi,
    /// Specifies the client API version that the created context must be compatible with. This is
    /// not a hard constraint, but creation will fail if the created context is less than the one
    /// requested. If a version of OpenGL version 1.0 is requested, GLFW will attempt to provide the
    /// highest supported version. If OpenGL ES 1.x is requested, creation will fail if OpenGL ES
    /// 2.0 or later is returned, since OpenGL ES 2.0 is not backwards compatible with 1.x.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_CONTEXT_VERSION_MAJOR_hint)
    pub context_version: (i32, i32),
    /// Specifies the robustness strategy to be used by the context.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_CONTEXT_ROBUSTNESS_hint)
    pub context_robustness: ContextRobustness,
    /// Specifies the release behaviour to be used by the context.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_CONTEXT_RELEASE_BEHAVIOR_hint)
    pub context_release_behavior: ContextReleaseBehavior,
    /// Specifies whether the created OpenGL context should be forward-compatible, i.e. one where
    /// all functionality deprecated in the requested version is removed. If OpenGL ES is requested,
    /// this hint is ignored.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_OPENGL_FORWARD_COMPAT_hint)
    pub opengl_forward_compatible: bool,
    /// Specifies whether to create a debug OpenGL context, which may have additional error and
    /// performance issue reporting functionality. This hint is ignored if OpenGL ES is requested.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_OPENGL_DEBUG_CONTEXT_hint)
    pub opengl_debug_context: bool,
    /// Specifies which OpenGL profile to create the context for.
    /// This hint is ignored if OpenGL ES is requested.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_OPENGL_PROFILE_hint)
    pub opengl_profile: OpenGlProfile,
    // Missing from http://www.glfw.org/docs/3.3/window_guide.html#window_hints_values
    // but present elsewhere in the documentation, so we will expose it
    /// Specifies whether errors should be generated by the context. If enabled, situations that
    /// would have generated errors instead cause undefined behavior.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_CONTEXT_NO_ERROR_hint)
    pub context_no_error: bool,
    /// Specifies whether to use full resolution framebuffers on Retina displays. This hint is
    /// ignored on platforms other than macOS.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_COCOA_RETINA_FRAMEBUFFER_hint)
    pub cocoa_retina_framebuffer: bool,
    /// Specifies the name to use when autosaving the window frame, or if empty disables frame
    /// autosaving for the window. This hint is ignored on platforms other than macOS.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_COCOA_FRAME_NAME_hint)
    pub cocoa_frame_name: &'a str,
    /// Specifies whether to allow Automatic Graphics Switching, i.e. to allow the system to choose
    /// the integrated GPU for the OpenGL context and move it between GPUs if necessary or whether
    /// to force it to always run on the discrete GPU. This hint is ignored on platforms other than
    /// macOS.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_COCOA_GRAPHICS_SWITCHING_hint)
    pub cocoa_graphics_switching: bool,
    /// Specifies the desired (ASCII-encoded) class part of the ICCCM `WM_CLASS` property. This hint
    /// is ignored for windowing systems other than X11.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_X11_CLASS_NAME)
    pub x11_class_name: &'a str,
    /// Specifies the desired (ASCII-encoded) instance part of the ICCCM `WM_CLASS` property. This
    /// hint is ignored for windowing systems other than X11.
    /// 
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/window_guide.html#GLFW_X11_INSTANCE_NAME)
    pub x11_instance_name: &'a str
}

impl<'a> Default for WindowHints<'a> {
    fn default() -> Self {
        WindowHints {
            resizable: true,
            visible: true,
            decorated: true,
            focused: true,
            auto_iconify: true,
            floating: false,
            maximized: false,
            center_cursor: true,
            transparent_framebuffer: false,
            red_bits: Some(8),
            green_bits: Some(8),
            blue_bits: Some(8),
            alpha_bits: Some(8),
            depth_bits: Some(24),
            stencil_bits: Some(8),
            accum_red_bits: Some(0),
            accum_green_bits: Some(0),
            accum_blue_bits: Some(0),
            accum_alpha_bits: Some(0),
            auxiliary_buffers: Some(0),
            samples: Some(0),
            refresh_rate: None,
            stereo: false,
            srgb_capable: false,
            double_buffer: true,
            client_api: ClientApi::OpenGl,
            context_creation_api: ContextCreationApi::Native,
            context_version: (1, 0),
            context_robustness: ContextRobustness::NoRobustness,
            context_release_behavior: ContextReleaseBehavior::Any,
            opengl_forward_compatible: false,
            opengl_debug_context: false,
            opengl_profile: OpenGlProfile::Any,
            context_no_error: false,
            cocoa_retina_framebuffer: true,
            cocoa_frame_name: "",
            cocoa_graphics_switching: false,
            x11_class_name: "",
            x11_instance_name: ""
        }
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug)]
    pub enum WindowAttribute {
        Focused = ffi::GLFW_FOCUSED,
        Iconified = ffi::GLFW_ICONIFIED,
        Maximized = ffi::GLFW_MAXIMIZED,
        Hovered = ffi::GLFW_HOVERED,
        Visible = ffi::GLFW_VISIBLE,
        Resizable = ffi::GLFW_RESIZABLE,
        Decorated = ffi::GLFW_DECORATED,
        AutoIconify = ffi::GLFW_AUTO_ICONIFY,
        Floating = ffi::GLFW_FLOATING,
        TransparentFramebuffer = ffi::GLFW_TRANSPARENT_FRAMEBUFFER,
        OpenGlForwardCompat = ffi::GLFW_OPENGL_FORWARD_COMPAT,
        OpenGlDebugContext = ffi::GLFW_OPENGL_DEBUG_CONTEXT,
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SetWindowAttribute {
    Decorated(bool),
    Resizable(bool),
    Floating(bool),
    AutoIconify(bool)
}

bitflags! {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__mods.html
    pub struct Modifiers: c_int {
        const SHIFT = ffi::GLFW_MOD_SHIFT;
        const CONTROL = ffi::GLFW_MOD_CONTROL;
        const ALT = ffi::GLFW_MOD_ALT;
        const SUPER = ffi::GLFW_MOD_SUPER;
        const CAPSLOCK = ffi::GLFW_MOD_CAPS_LOCK;
        const NUMLOCK = ffi::GLFW_MOD_NUM_LOCK;
    }
}

bitflags! {
    /// [GLFW Reference][glfw]
    /// 
    /// [glfw]: http://www.glfw.org/docs/3.3/group__hat__state.html
    pub struct JoystickHatState: c_uchar {
        const CENTERED = ffi::GLFW_HAT_CENTERED as u8;

        const UP = ffi::GLFW_HAT_UP as u8;
        const RIGHT = ffi::GLFW_HAT_RIGHT as u8;
        const DOWN = ffi::GLFW_HAT_DOWN as u8;
        const LEFT = ffi::GLFW_HAT_LEFT as u8;

        const RIGHT_UP =   ffi::GLFW_HAT_RIGHT_UP as u8;
        const RIGHT_DOWN = ffi::GLFW_HAT_RIGHT_DOWN as u8;
        const LEFT_UP =    ffi::GLFW_HAT_LEFT_UP as u8;
        const LEFT_DOWN =  ffi::GLFW_HAT_LEFT_DOWN as u8;
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug)]
    pub enum InputMode {
        StickyKeys = ffi::GLFW_STICKY_KEYS,
        StickyMouseButtons = ffi::GLFW_STICKY_MOUSE_BUTTONS,
        LockKeyMods = ffi::GLFW_LOCK_KEY_MODS,
        RawMouseMotion = ffi::GLFW_RAW_MOUSE_MOTION
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SetInputMode {
    Cursor(CursorMode),
    StickyKeys(bool),
    StickyMouseButtons(bool),
    LockKeyMods(bool),
    RawMouseMotion(bool)
}

/// GLFW library initialization hints.
#[derive(Debug, Copy, Clone)]
pub struct InitHints {
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/intro_guide.html#GLFW_COCOA_CHDIR_RESOURCES)
    pub cocoa_chdir_resources: bool,
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/intro_guide.html#GLFW_COCOA_MENUBAR)
    pub cocoa_menubar: bool,
    /// [GLFW Reference](http://www.glfw.org/docs/3.3/intro_guide.html#GLFW_JOYSTICK_HAT_BUTTONS)
    pub joystick_hat_buttons: bool,
    _private: ()
}

impl Default for InitHints {
    fn default() -> Self {
        InitHints {
            cocoa_chdir_resources: true,
            cocoa_menubar: true,
            joystick_hat_buttons: true,
            _private: ()
        }
    }
}
