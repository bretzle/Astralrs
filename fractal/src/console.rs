//! Console

use crate::color::Color;

/// Represents an internal storage type for a character in a console
pub struct Tile {
    /// The glyph that will be drawn to represent the tile
    pub glyph: u8,
    /// The color of the glyph.
    pub fg: Color,
    /// The color of the background behind the glyph.
    pub bg: Color,
}

/// Trait that must be implemented by console types
pub trait Console {
    /// Clears the screen.
    fn cls(&mut self);

    /// Sets a single cell in the console.
    fn set(&mut self, x: i32, y: i32, fg: Color, bg: Color, glyph: u8);

    /// Translate an (x, y) coordinate into an array index.
    fn at(&self, x: i32, y: i32) -> usize {
        unimplemented!();
    }
}

pub fn log<S: ToString>(message: S) {
    println!("{}", message.to_string());
}