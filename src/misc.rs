use std::ops::{ Index, IndexMut };

use ffi;
use util::cuchar_to_bool;
use Glfw;

pub struct Image {
    pixels: Vec<Pixel>,
    width: i32,
    height: i32
}

#[repr(C)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl Image {
    pub(crate) fn as_glfw_image(&self) -> ffi::GLFWimage {
        ffi::GLFWimage {
            width: self.width,
            height: self.height,
            pixels: self.pixels.as_ptr() as *const u8
        }
    }

    pub fn new(width: i32, height: i32) -> Self {
        Image {
            width: width,
            height: height,
            pixels: (0..width*height).into_iter().map(|_| Pixel {
                red: 0, green: 0, blue: 0, alpha: 0
            }).collect()
        }
    }
}

impl Index<usize> for Image {
    type Output = [Pixel];

    fn index(&self, y: usize) -> &[Pixel] {
        let offset = y * self.width as usize;
        &self.pixels[offset..offset + self.width as usize]
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, y: usize) -> &mut [Pixel] {
        let offset = y * self.width as usize;
        &mut self.pixels[offset..offset + self.width as usize]
    }
}

pub struct Cursor<'a> {
    pub(crate) ptr: *mut ffi::GLFWcursor,
    pub(crate) glfw: &'a Glfw
}

impl<'a> Drop for Cursor<'a> {
    fn drop(&mut self) {
        self.glfw.destroy_cursor(self.ptr);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GamepadState {
    pub a_cross:      bool,
    pub b_circle:     bool,
    pub x_square:     bool,
    pub y_triangle:   bool,
    pub left_bumper:  bool,
    pub right_bumper: bool,
    pub back:         bool,
    pub start:        bool,
    pub guide:        bool,
    pub left_stick:   bool,
    pub right_stick:  bool,
    pub dpad_up:      bool,
    pub dpad_right:   bool,
    pub dpad_down:    bool,
    pub dpad_left:    bool,

    pub left_stick_x:  f32,
    pub left_stick_y:  f32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
    pub left_trigger:  f32,
    pub right_trigger: f32
}

impl From<ffi::GLFWgamepadstate> for GamepadState {
    fn from(c: ffi::GLFWgamepadstate) -> Self {
        GamepadState {
            a_cross:      cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_A as usize]),
            b_circle:     cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_B as usize]),
            x_square:     cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_X as usize]),
            y_triangle:   cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_Y as usize]),
            left_bumper:  cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_LEFT_BUMPER as usize]),
            right_bumper: cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER as usize]),
            back:         cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_BACK as usize]),
            start:        cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_START as usize]),
            guide:        cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_GUIDE as usize]),
            left_stick:   cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_LEFT_THUMB as usize]),
            right_stick:  cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_RIGHT_THUMB as usize]),
            dpad_up:      cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_DPAD_UP as usize]),
            dpad_right:   cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_DPAD_RIGHT as usize]),
            dpad_down:    cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_DPAD_DOWN as usize]),
            dpad_left:    cuchar_to_bool(c.buttons[ffi::GLFW_GAMEPAD_BUTTON_DPAD_LEFT as usize]),

            left_stick_x:  c.axes[ffi::GLFW_GAMEPAD_AXIS_LEFT_X as usize],
            left_stick_y:  c.axes[ffi::GLFW_GAMEPAD_AXIS_LEFT_Y as usize],
            right_stick_x: c.axes[ffi::GLFW_GAMEPAD_AXIS_RIGHT_X as usize],
            right_stick_y: c.axes[ffi::GLFW_GAMEPAD_AXIS_RIGHT_Y as usize],
            left_trigger:  c.axes[ffi::GLFW_GAMEPAD_AXIS_LEFT_TRIGGER as usize],
            right_trigger: c.axes[ffi::GLFW_GAMEPAD_AXIS_RIGHT_TRIGGER as usize],
        }
    }
}