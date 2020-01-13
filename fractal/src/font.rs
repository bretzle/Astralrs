#![allow(unsafe_code)]

//! Font

use crate::embedding;
use crate::Platform;
use image::{ColorType, GenericImageView};
use glow::HasContext;

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

    pub fn setup_gl_texture(&mut self, platform: &Platform) -> u32 {
        let gl = &platform.platform.gl;
        let texture;

        unsafe {
            texture = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            ); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            // set texture filtering parameters
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );

            let img_orig = Font::load_image(&self.bitmap_file);
            let img = img_orig.flipv();
            let data = img.raw_pixels();
            let format = match img.color() {
                ColorType::RGB(_) => glow::RGB,
                ColorType::RGBA(_) => glow::RGBA,
                _ => {
                    panic!(
                        "unexpected image format {:?} for {}",
                        img.color(),
                        self.bitmap_file
                    );
                }
            };
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format,
                glow::UNSIGNED_BYTE,
                Some(&data),
            );
        }

        self.gl_id = Some(texture);

        texture
    }

    pub fn bind_texture(&self, platform: &Platform) {
        let gl = &platform.platform.gl;

        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, self.gl_id);
        }
    }
}
