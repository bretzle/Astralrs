//! The main engine

use super::Platform;
use super::Shader;
use super::SimpleConsole;
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

impl Fractal {
    /// Quick initialization for when you want a basic setup
    pub fn init_simple<S: ToString>(
        width_chars: u32,
        height_chars: u32,
        title: S,
        shaders: S,
    ) -> Self {
        let font_path = format!("{}/terminal8x8.png", &shaders.to_string());
        let mut context = Fractal::init_raw(width_chars * 8, height_chars * 8, title);
        let font = context.register_font(Font::load(&font_path.to_string(), (8, 8)));

        context.register_console(
            SimpleConsole::init(width_chars, height_chars, &context.backend),
            font,
        );

        context
    }

    /// Initializes the Engine.
    pub fn init_raw<S: ToString>(width_pixels: u32, height_pixels: u32, window_title: S) -> Self {
        unimplemented!()
    }

    /// Registers a font with the engine.
    pub fn register_font(&mut self, mut font: Font) -> usize {
        unimplemented!()
    }

    /// Regjsters a console with the engine.
    pub fn register_console(&mut self, new_console: Box<dyn Console>, font_index: usize) -> usize {
        unimplemented!()
    }
}
