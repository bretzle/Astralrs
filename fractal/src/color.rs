use crate::rex::XpColor;

#[derive(PartialEq, Copy, Clone, Default, Debug, Serialize, Deserialize)]
/// Represents an R/G/B triplet
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    /// Constructs a new, zeroed (black) RGB triplet.
    pub const fn new() -> RGB {
        RGB { r: 0, g: 0, b: 0 }
    }

    /// Constructs a new RGB color, from 3 bytes in the range 0..255
    pub const fn from_u8(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }

    /// Converts an xp file color component to an RGB
    pub fn from_xp(col: XpColor) -> RGB {
        RGB::from_u8(col.r, col.g, col.b)
    }

    /// Converts an RGB to an xp file color component
    pub fn to_xp(&self) -> XpColor {
        XpColor::new(self.r, self.g, self.b)
    }

    /// Applies a quick grayscale conversion to the color
    pub fn to_greyscale(&self) -> RGB {
        let linear = ((self.r as f32 * 0.2126 * 255.0) + (self.g as f32 * 0.7152 * 255.0) + (self.b as f32 * 0.0722 * 255.0)) as u8;
        RGB::from_u8(linear, linear, linear)
    }

    /// Gets the red component as a f32
    /// range 0..1
    pub fn rf(&self) -> f32 {
        self.r as f32 / 255.0
    }

    /// Gets the red component as a f32
    /// range 0..1
    pub fn gf(&self) -> f32 {
        self.g as f32 / 255.0
    }

    /// Gets the red component as a f32
    /// range 0..1
    pub fn bf(&self) -> f32 {
        self.b as f32 / 255.0
    }
}

pub const WHITE: RGB = RGB::from_u8(255, 255, 255);
pub const GRAY: RGB = RGB::from_u8(128, 128, 128);
pub const BLACK: RGB = RGB::from_u8(0, 0, 0);

pub const RED: RGB = RGB::from_u8(255, 0, 0);
pub const GREEN: RGB = RGB::from_u8(0, 255, 0);
pub const BLUE: RGB = RGB::from_u8(0, 0, 255);

pub const AQUA: RGB = RGB::from_u8(0, 255, 255);
pub const YELLOW: RGB = RGB::from_u8(255, 255, 0);
pub const PURPLE: RGB = RGB::from_u8(128, 0, 128);

pub const ORANGE: RGB = RGB::from_u8(255, 165, 0);
pub const MAGENTA: RGB = RGB::from_u8(255, 0, 255);
pub const CYAN: RGB = AQUA;
pub const PINK: RGB = RGB::from_u8(255, 192, 203);
