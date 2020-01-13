//! A simple implementation of [Console]

use crate::color;
use crate::color::Color;
use crate::console::Console;
use crate::console::Tile;
use crate::Platform;

/// A simple console with background color.
pub struct SimpleConsole {
    /// the width in pixels
    pub width: u32,
    /// the width in pixels
    pub height: u32,
    /// Vector of tiles. Each tile can render one character.
    pub tiles: Vec<Tile>,
}

impl SimpleConsole {
    /// Initializes a Simple Console.
    pub fn init(width: u32, height: u32, platform: &Platform) -> Box<SimpleConsole> {
        let num_tiles = (width * height) as usize;
        let mut tiles = Vec::with_capacity(num_tiles);

        for _ in 0..num_tiles {
            tiles.push(Tile {
                glyph: 0,
                fg: color::WHITE,
                bg: color::BLACK,
            });
        }

        Box::new(SimpleConsole {
            width,
            height,
            tiles,
        })
    }
}

impl Console for SimpleConsole {
    fn cls(&mut self) {
        for tile in &mut self.tiles {
            tile.glyph = 32;
            tile.fg = color::WHITE;
            tile.bg = color::BLACK;
        }
    }

    fn set(&mut self, x: i32, y: i32, fg: Color, bg: Color, glyph: u8) {
        let location = self.at(x, y);

        self.tiles[location].glyph = glyph;
        self.tiles[location].fg = fg;
        self.tiles[location].bg = bg;
    }

    fn at(&self, x: i32, y: i32) -> usize {
        (((self.height - 1 - y as u32) * self.width) + x as u32) as usize
    }
}
