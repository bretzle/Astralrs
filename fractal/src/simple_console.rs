//! A simple implementation of [Console]

use crate::backend::font::Font;
use crate::backend::shader::Shader;
use crate::backend::simple_backend::SimpleConsoleBackend;
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
    /// is the console dirty
    pub is_dirty: bool,
    // To handle offset tiles for people who want thin walls between tiles
    offset_x: f32,
    offset_y: f32,
    /// OpenGL backend
    pub backend: SimpleConsoleBackend,
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
            is_dirty: true,
            offset_x: 0.0,
            offset_y: 0.0,
            backend: SimpleConsoleBackend::new(platform, width as usize, height as usize),
        })
    }

    fn rebuild_vertices(&mut self, platform: &Platform) {
        self.backend.rebuild_vertices(
            platform,
            self.height,
            self.width,
            &self.tiles,
            self.offset_x,
            self.offset_y,
        );
    }
}

impl Console for SimpleConsole {
    fn cls(&mut self) {
        self.is_dirty = true;
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

    fn draw(&mut self, font: &Font, shader: &Shader, platform: &Platform) {
        self.backend
            .draw(font, shader, platform, self.width, self.height);
        self.is_dirty = false;
    }

    fn resize_pixels(&mut self, _width: u32, _height: u32) {
        self.is_dirty = true;
    }

    fn rebuild_if_dirty(&mut self, platform: &Platform) {
        if self.is_dirty {
            self.rebuild_vertices(platform);
            self.is_dirty = false;
        }
    }
}
