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
    /// Check to see if the internal OpenGL representation needs to be rebuilt, and do so if required.
    fn rebuild_if_dirty(&mut self, platform: &hal::RltkPlatform);

    /// Gets the dimensions of the console in characters
    fn get_char_size(&mut self) -> (u32, u32);

    // Resizes the viewport
    fn resize_pixels(&mut self, width: u32, height: u32);

    /// Tells the console to draw itself via OpenGL.
    fn gl_draw(&mut self, font: &Font, shader: &Shader, platform: &hal::RltkPlatform);

    /// Converts an x/y coordinate to a console index number.
    fn at(&self, x: i32, y: i32) -> usize;

    /// Clear the console.
    fn cls(&mut self);

    /// Clear the console to a set background color, if supported.
    fn cls_bg(&mut self, background: Color);

    /// Print a string at the specified x/y coordinate.
    fn print(&mut self, x: i32, y: i32, output: &str);

    /// Print a string in color at the specified x/y coordinate, with specified foreground and background.
    fn print_color(&mut self, x: i32, y: i32, fg: Color, bg: Color, output: &str);

    /// Sets a single cell to a color/glyph combination.
    fn set(&mut self, x: i32, y: i32, fg: Color, bg: Color, glyph: u8);

    /// Sets a single cell's background color.
    fn set_bg(&mut self, x: i32, y: i32, bg: Color);

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: Color, bg: Color);

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters,
    /// without filling in the middle
    fn draw_hollow_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: Color, bg: Color);

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_box_double(&mut self, x: i32, y: i32, width: i32, height: i32, fg: Color, bg: Color);

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters,
    /// without filling in the middle
    fn draw_hollow_box_double(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: Color,
        bg: Color,
    );

    /// Fills a rectangle-defined region with a given glyph
    // fn fill_region(&mut self, target: Rect, glyph: u8, fg: Color, bg: Color);

    /// Retrieve a given cell in the console, if present
    fn get(&self, x: i32, y: i32) -> Option<(&u8, &Color, &Color)>;

    /// Draws a horizontal progress bar.
    fn draw_bar_horizontal(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: Color,
        bg: Color,
    );

    /// Draws a vertical progress bar.
    fn draw_bar_vertical(
        &mut self,
        x: i32,
        y: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: Color,
        bg: Color,
    );

    /// Prints text, centered to the whole console width, at vertical location y.
    fn print_centered(&mut self, y: i32, text: &str);

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered(&mut self, y: i32, fg: Color, bg: Color, text: &str);

    /// Serializes the console layer to an XpFile
    // fn to_xp_layer(&self) -> XpLayer;

    /// Specify a global offset (by character count, so 0.5 is half a character). Useful for
    /// drawing walls between tiles.
    fn set_offset(&mut self, x: f32, y: f32);

    // Produces the implementor as an Any that can be matched to determine type and access
    // natively.
    // fn as_any(&self) -> &dyn Any;
}

pub fn log<S: ToString>(message: S) {
    println!("{}", message.to_string());
}
