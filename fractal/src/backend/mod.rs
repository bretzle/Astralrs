//! Contains all backend capabilities for OpenGL

use crate::backend::framebuffer::Framebuffer;

pub mod font;
pub mod framebuffer;
pub mod init;
pub mod mainloop;
pub mod quadrender;
pub mod shader;
pub mod shader_strings;
pub mod simple_console_backing;

// TODO merge [FractalPlatform] and [PlatformGL]

/// The platform to use
pub struct FractalPlatform {
    /// The opengl platform
    pub platform: PlatformGL,
}

/// The OpenGl platform
pub struct PlatformGL {
    /// TODO
    pub gl: glow::Context,
    /// TODO
    pub quad_vao: u32,
    /// TODO
    pub context_wrapper: Option<WrappedContext>,
    /// TODO
    pub backing_buffer: Framebuffer,
}

/// Wrapper for the windows event loop and inputs
pub struct WrappedContext {
    /// event loop
    pub el: glutin::event_loop::EventLoop<()>,
    /// window context
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}
