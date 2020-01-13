//! Console

use crate::color::Color;

/// Trait that must be implemented by console types
pub trait Console {
    /// Clears the screen.
    fn cls(&mut self);

    /// Sets a single cell in the console.
    fn set(&mut self, x: i32, y: i32, fg: Color, bg: Color, glyph: u8);
}

pub fn log<S: ToString>(message: S) {
    println!("{}", message.to_string());
}