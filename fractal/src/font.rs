//! Font

use crate::embedding;
use image::{ColorType, GenericImageView};

#[derive(Debug, Clone, PartialEq)]
/// Represents a font
pub struct Font {
    /// name of file
    pub bitmap_file: String,
    /// width in pixels
    pub width: u32,
    /// height in pixels
    pub height: u32,
    /// glsl id
    pub gl_id: Option<u32>,
    /// size a a tile
    pub tile_size: (u32, u32),
}

impl Font {
    /// Creates an unloaded texture with filename and size parameters provided.
    pub fn new<S: ToString>(filename: S, width: u32, height: u32, tile_size: (u32, u32)) -> Font {
        Font {
            bitmap_file: filename.to_string(),
            width,
            height,
            gl_id: None,
            tile_size,
        }
    }

    /// Loads a font file.
    pub fn load<S: ToString>(filename: S, tile_size: (u32, u32)) -> Self {
        let img = Font::load_image(&filename.to_string());
        Font {
            bitmap_file: filename.to_string(),
            width: img.width(),
            height: img.height(),
            gl_id: None,
            tile_size,
        }
    }

    /// loads an image for further processing.
    fn load_image(filename: &str) -> image::DynamicImage {
        let resource = embedding::EMBED
            .lock()
            .unwrap()
            .get_resource(filename.to_string());
        match resource {
            None => image::open(std::path::Path::new(&filename.to_string()))
                .expect("Failed to load texture"),
            Some(res) => image::load_from_memory(res).expect("Failed to load texture from memory"),
        }
    }
}
