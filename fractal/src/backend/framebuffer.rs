//! TODO

#![allow(unsafe_code)]

use glow::HasContext;

/// A frame buffer
pub struct Framebuffer {
    fbo: u32,
    /// a texture
    pub texture: u32,
}

impl Framebuffer {
    /// builds a frame buffer
    pub fn build_fbo(gl: &glow::Context, width: i32, height: i32) -> Framebuffer {
        let fbo;
        let buffer;

        unsafe {
            fbo = gl.create_framebuffer().unwrap();
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
            buffer = gl.create_texture().unwrap();

            gl.bind_texture(glow::TEXTURE_2D, Some(buffer));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::FLOAT,
                None,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            // attach texture to framebuffer
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(buffer),
                0,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        Framebuffer {
            fbo,
            texture: buffer,
        }
    }

    /// Binds the frame buffer with OpenGL
    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.fbo));
        }
    }

    /// Removes the frame buffer from OpenGL
    pub fn default(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }
}
