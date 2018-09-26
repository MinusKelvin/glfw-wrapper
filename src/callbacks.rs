pub mod monitor {
    use libc::c_int;
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
        unsafe {
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
        }
    }

    pub(crate) fn initialize() {
        unsafe { ffi::glfwSetMonitorCallback(Some(callback)) };
    }
}
