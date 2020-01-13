//! Console

use crate::color::Color;
use crate::font::Font;
use crate::graphics::shader::Shader;
use crate::Platform;

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

    /// Tells the console to draw itself to the screen.
    fn draw(&mut self, font: &Font, shader: &Shader, platform: &Platform);

    /// Resizes pixels
    fn resize_pixels(&mut self, width: u32, height: u32);

    /// Check to see if the internal OpenGL representation needs to be rebuilt, and do so if required.
    fn rebuild_if_dirty(&mut self, platform: &Platform) {
        unimplemented!();
    }
}

pub fn log<S: ToString>(message: S) {
    println!("{}", message.to_string());
}
