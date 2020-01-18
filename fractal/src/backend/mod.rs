use crate::backend::framebuffer::Framebuffer;

pub mod font;
pub mod framebuffer;
pub mod init;
pub mod mainloop;
pub mod quadrender;
pub mod shader;
pub mod shader_strings;
pub mod simple_console_backing;

pub struct FractalPlatform {
    pub platform: PlatformGL,
}

pub struct PlatformGL {
    pub gl: glow::Context,
    pub quad_vao: u32,
    pub context_wrapper: Option<WrappedContext>,
    pub backing_buffer: Framebuffer,
}

pub struct WrappedContext {
    pub el: glutin::event_loop::EventLoop<()>,
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}
