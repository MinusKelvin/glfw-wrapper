use std::os::raw::{ c_int, c_uchar };
use ffi;

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub enum Action {
        Release = ffi::GLFW_RELEASE,
        Press = ffi::GLFW_PRESS,
        Repeat = ffi::GLFW_REPEAT
    }
}

enum_from_primitive! {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Key {
    Known(KeyCode),
    Unknown(i32)
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

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
    pub enum WindowBorder {
        Left = ffi::GLFW_WINDOW_LEFT,
        Top = ffi::GLFW_WINDOW_TOP,
        Right = ffi::GLFW_WINDOW_RIGHT,
        Bottom = ffi::GLFW_WINDOW_BOTTOM,
        TopLeft = ffi::GLFW_WINDOW_TOPLEFT,
        TopRight = ffi::GLFW_WINDOW_TOPRIGHT,
        BottomLeft = ffi::GLFW_WINDOW_BOTTOMLEFT,
        BottomRight = ffi::GLFW_WINDOW_BOTTOMRIGHT
    }
}

#[derive(Copy, Clone, Debug)]
pub enum WindowHint<'a> {
    Resizable(bool),
    Visible(bool),
    Decorated(bool),
    Focused(bool),
    AutoIconify(bool),
    Floating(bool),
    Maximized(bool),
    CenterCursor(bool),
    TransparentFramebuffer(bool),
    // Missing from http://www.glfw.org/docs/3.3/window_guide.html#window_hints_values
    // FocusOnShow(bool),
    RedBits(Option<i32>),
    GreenBits(Option<i32>),
    BlueBits(Option<i32>),
    AlphaBits(Option<i32>),
    DepthBits(Option<i32>),
    StencilBits(Option<i32>),
    AccumRedBits(Option<i32>),
    AccumGreenBits(Option<i32>),
    AccumBlueBits(Option<i32>),
    AccumAlphaBits(Option<i32>),
    AuxiliaryBuffers(Option<i32>),
    Samples(Option<i32>),
    RefreshRate(Option<i32>),
    Stereo(bool),
    SrgbCapable(bool),
    DoubleBuffer(bool),
    ClientApi(ClientApi),
    ContextCreationApi(ContextCreationApi),
    ContextVersionMajor(i32),
    ContextVersionMinor(i32),
    ContextRobustness(ContextRobustness),
    ContextReleaseBehavior(ContextReleaseBehavior),
    OpenGlForwardCompat(bool),
    OpenGlDebugContext(bool),
    OpenGlProfile(OpenGlProfile),
    // Missing from http://www.glfw.org/docs/3.3/window_guide.html#window_hints_values
    // but present elsewhere in the documentation, so we will expose it
    ContextNoError(bool),
    CocoaRetinaFramebuffer(bool),
    CocoaFrameName(&'a str),
    CocoaGraphicsSwitching(bool),
    X11ClassName(&'a str),
    X11InstanceName(&'a str)
}

#[derive(Copy, Clone, Debug)]
pub enum InitHint {
    JoystickHatButtons(bool),
    CocoaChDirResources(bool),
    CocoaMenubar(bool)
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
    pub struct Modifiers: c_int {
        const Shift = ffi::GLFW_MOD_SHIFT;
        const Control = ffi::GLFW_MOD_CONTROL;
        const Alt = ffi::GLFW_MOD_ALT;
        const Super = ffi::GLFW_MOD_SUPER;
        const CapsLock = ffi::GLFW_MOD_CAPS_LOCK;
        const NumLock = ffi::GLFW_MOD_NUM_LOCK;
    }
}

bitflags! {
    pub struct JoystickHat: c_uchar {
        const Centered = ffi::GLFW_HAT_CENTERED as u8;

        const Up = ffi::GLFW_HAT_UP as u8;
        const Right = ffi::GLFW_HAT_RIGHT as u8;
        const Down = ffi::GLFW_HAT_DOWN as u8;
        const Left = ffi::GLFW_HAT_LEFT as u8;

        const RightUp =   ffi::GLFW_HAT_RIGHT_UP as u8;
        const RightDown = ffi::GLFW_HAT_RIGHT_DOWN as u8;
        const LeftUp =    ffi::GLFW_HAT_LEFT_UP as u8;
        const LeftDown =  ffi::GLFW_HAT_LEFT_DOWN as u8;
    }
}

enum_from_primitive! {
    #[repr(i32)]
    #[derive(Copy, Clone, Hash, Debug)]
    pub enum InputMode {
        StickyKeys = ffi::GLFW_STICKY_KEYS,
        StickyMouseButtons = ffi::GLFW_STICKY_MOUSE_BUTTONS,
        LockKeyMods = ffi::GLFW_LOCK_KEY_MODS
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SetInputMode {
    Cursor(CursorMode),
    StickyKeys(bool),
    StickyMouseButtons(bool),
    LockKeyMods(bool)
}