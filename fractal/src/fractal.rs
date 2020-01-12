//! The main engine

use super::Platform;
use super::Shader;
use super::VirtualKeyCode;
use crate::console::Console;
use crate::font::Font;

/// A display console used internally to provide console render support.
pub struct DisplayConsole {
    /// 
    pub console: Box<dyn Console>,
    /// 
    pub shader_index: usize,
    /// 
    pub font_index: usize,
}

/// Acts as a context for the engine
pub struct Fractal {
    ///
    pub backend: Platform,
    ///
    pub width_pixels: u32,
    ///
    pub height_pixels: u32,
    ///
    pub fonts: Vec<Font>,
    ///
    pub shaders: Vec<Shader>,
    ///
    pub consoles: Vec<DisplayConsole>,
    ///
    pub fps: f32,
    ///
    pub frame_time_ms: f32,
    ///
    pub active_console: usize,
    ///
    pub key: Option<VirtualKeyCode>,
    ///
    pub mouse_pos: (i32, i32),
    ///
    pub left_click: bool,
    ///
    pub shift: bool,
    ///
    pub control: bool,
    ///
    pub alt: bool,
    ///
    pub web_button: Option<String>,
    ///
    pub quitting: bool,
    ///
    pub post_scanlines: bool,
    ///
    pub post_screenburn: bool,
}

impl Fractal {}
