//! A simple implementation of [Console]

use crate::color::Color;
use crate::console::Console;
use crate::Platform;

/// A simple console with background color.
pub struct SimpleConsole {
    /// the width in pixels
    pub width: u32,
    /// the width in pixels
    pub height: u32,
}

impl SimpleConsole {
    /// Initializes a Simple Console.
    pub fn init(width: u32, height: u32, platform: &Platform) -> Box<SimpleConsole> {
        unimplemented!()
    }
}

impl Console for SimpleConsole {
    fn cls(&mut self) {
        unimplemented!();
    }

    fn set(&mut self, x: i32, y: i32, fg: Color, bg: Color, glyph: u8) {
        unimplemented!();
    }
}
