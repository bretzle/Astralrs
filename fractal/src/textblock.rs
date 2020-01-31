//! Utilities for having blocks of text on the screen

use crate::codepage437::string_to_cp437;
use crate::color;
use crate::color::RGB;
use crate::console::Console;
use crate::console::Tile;

/// A block of text
pub struct TextBlock {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    fg: RGB,
    bg: RGB,
    buffer: Vec<Tile>,
    cursor: (i32, i32),
}

impl TextBlock {
    /// Constructor
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> TextBlock {
        TextBlock {
            x,
            y,
            width,
            height,
            fg: color::WHITE,
            bg: color::BLACK,
            buffer: vec![
                Tile {
                    glyph: 0,
                    fg: color::WHITE,
                    bg: color::BLACK,
                };
                width as usize * height as usize
            ],
            cursor: (0, 0),
        }
    }

    /// Sets the foreground
    pub fn fg(&mut self, fg: RGB) {
        self.fg = fg;
    }

    /// Sets the background
    pub fn bg(&mut self, bg: RGB) {
        self.bg = bg;
    }

    /// Moves the cursor to a new location
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.cursor = (x, y);
    }

    fn at(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    /// Render the textblock onto a console
    pub fn render(&self, mut console: impl AsMut<dyn Console>) {
        for y in 0..self.height {
            for x in 0..self.width {
                console.as_mut().set(
                    x + self.x,
                    y + self.y,
                    self.buffer[self.at(x, y)].fg,
                    self.buffer[self.at(x, y)].bg,
                    self.buffer[self.at(x, y)].glyph,
                );
            }
        }
    }

    /// Updates the current TextBlock with the commands from a TextBuilder
    pub fn print(&mut self, text: &TextBuilder) {
        for cmd in &text.commands {
            match cmd {
                CommandType::Text { block: t } => {
                    for c in t {
                        let idx = self.at(self.cursor.0, self.cursor.1);
                        self.buffer[idx].glyph = *c;
                        self.buffer[idx].fg = self.fg;
                        self.buffer[idx].bg = self.bg;
                        self.cursor.0 += 1;
                        if self.cursor.0 >= self.width {
                            self.cursor.0 = 0;
                            self.cursor.1 += 1;
                        }
                    }
                }

                CommandType::Centered { block: t } => {
                    let text_width = t.len() as i32;
                    let half_width = text_width / 2;
                    self.cursor.0 = (self.width / 2) - half_width;
                    for c in t {
                        let idx = self.at(self.cursor.0, self.cursor.1);
                        self.buffer[idx].glyph = *c;
                        self.buffer[idx].fg = self.fg;
                        self.buffer[idx].bg = self.bg;
                        self.cursor.0 += 1;
                        if self.cursor.0 >= self.width {
                            self.cursor.0 = 0;
                            self.cursor.1 += 1;
                        }
                    }
                }

                CommandType::NewLine {} => {
                    self.cursor.0 = 0;
                    self.cursor.1 += 1;
                }

                CommandType::Foreground { col } => self.fg = *col,
                CommandType::Background { col } => self.bg = *col,
                CommandType::Reset {} => {
                    self.cursor = (0, 0);
                    self.fg = color::WHITE;
                    self.bg = color::BLACK;
                }

                CommandType::TextWrapper { block: t } => {
                    for word in t.split(' ') {
                        let mut chrs = string_to_cp437(&word);
                        chrs.push(32);
                        if self.cursor.0 + chrs.len() as i32 >= self.width {
                            self.cursor.0 = 0;
                            self.cursor.1 += 1;
                        }
                        for c in chrs {
                            let idx = self.at(self.cursor.0, self.cursor.1);
                            self.buffer[idx].glyph = c;
                            self.buffer[idx].fg = self.fg;
                            self.buffer[idx].bg = self.bg;
                            self.cursor.0 += 1;
                            if self.cursor.0 >= self.width {
                                self.cursor.0 = 0;
                                self.cursor.1 += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The types of commands that a TextBuilder can have
pub enum CommandType {
    /// TODO
    Text {
        /// TODO
        block: Vec<u8>,
    },
    /// TODO
    Centered {
        /// TODO
        block: Vec<u8>,
    },
    /// TODO
    NewLine {},
    /// TODO
    Foreground {
        /// TODO
        col: RGB,
    },
    /// TODO
    Background {
        /// TODO
        col: RGB,
    },
    /// TODO
    TextWrapper {
        /// TODO
        block: String,
    },
    /// TODO
    Reset {},
}

/// Struct to easily create TextBlocks
pub struct TextBuilder {
    commands: Vec<CommandType>,
}

impl TextBuilder {
    /// Appends a string to the buffer
    pub fn append(&mut self, text: &str) -> &mut Self {
        let chrs = string_to_cp437(&text);
        self.commands.push(CommandType::Text { block: chrs });
        self
    }

    /// Appends a centered string to the buffer
    pub fn centered(&mut self, text: &str) -> &mut Self {
        let chrs = string_to_cp437(&text);
        self.commands.push(CommandType::Centered { block: chrs });
        self
    }

    /// Move cursor to (0, 0) and reset colors to default
    pub fn reset(&mut self) -> &mut Self {
        self.commands.push(CommandType::Reset {});
        self
    }

    /// Adds a new line to the text block
    pub fn ln(&mut self) -> &mut Self {
        self.commands.push(CommandType::NewLine {});
        self
    }

    /// Sets the foreground color
    pub fn fg(&mut self, col: RGB) -> &mut Self {
        self.commands.push(CommandType::Foreground { col });
        self
    }

    /// Sets the background color
    pub fn bg(&mut self, col: RGB) -> &mut Self {
        self.commands.push(CommandType::Background { col });
        self
    }

    /// Append a string. will wrap around to the next line if there is not enough space
    pub fn line_wrap(&mut self, text: &str) -> &mut Self {
        self.commands.push(CommandType::TextWrapper {
            block: text.to_string(),
        });
        self
    }
}

impl Default for TextBuilder {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}
