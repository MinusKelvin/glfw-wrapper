use std::ptr;
use libc::{ c_int, c_uchar };
use ffi;

pub(crate) fn bool_to_cint(b: bool) -> c_int {
    if b {
        1
    } else {
        0
    }
}

pub(crate) fn cint_to_bool(b: c_int) -> bool {
    b != 0
}

pub(crate) fn cuchar_to_bool(b: c_uchar) -> bool {
    b != 0
}

pub(crate) trait DontCare {
    fn or_dont_care(self) -> i32;
}

impl DontCare for Option<i32> {
    fn or_dont_care(self) -> i32 {
        self.unwrap_or(ffi::GLFW_DONT_CARE)
    }
}

pub(crate) trait MaybePtr<T> {
    fn ptr(self) -> *const T;
}

impl<'a, T> MaybePtr<T> for Option<&'a T> {
    fn ptr(self) -> *const T {
        self.map(|b| b as *const T).unwrap_or(ptr::null())
    }
}

pub(crate) trait MaybeMutPtr<T> {
    fn ptr(self) -> *mut T;
}

impl<'a, T> MaybeMutPtr<T> for Option<&'a mut T> {
    fn ptr(self) -> *mut T {
        self.map(|b| b as *mut T).unwrap_or(ptr::null_mut())
    }
}