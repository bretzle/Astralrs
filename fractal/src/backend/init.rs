//! This module contains a single helper function

#![allow(unsafe_code)]

use crate::backend::framebuffer::Framebuffer;
use crate::backend::quadrender::setup_quad;
use crate::backend::shader::Shader;
use crate::backend::shader_strings;
use crate::backend::FractalPlatform;
use crate::backend::PlatformGL;
use crate::backend::WrappedContext;
use crate::fractal::Fractal;
use glutin::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};

/// Creates a raw Fractal instance
pub fn init_raw<S: ToString>(width_pixels: u32, height_pixels: u32, window_title: S) -> Fractal {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(window_title.to_string())
        .with_inner_size(LogicalSize::new(
            f64::from(width_pixels),
            f64::from(height_pixels),
        ));
    let windowed_context = ContextBuilder::new()
        .with_gl(glutin::GlRequest::Latest)
        .with_gl_profile(glutin::GlProfile::Core)
        .with_hardware_acceleration(Some(true))
        .with_vsync(true)
        .with_srgb(true)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let gl = glow::Context::from_loader_function(|ptr| windowed_context.get_proc_address(ptr));

    // Load our basic shaders
    let mut shaders: Vec<Shader> = Vec::new();

    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_WITH_BG_VS,
        shader_strings::CONSOLE_WITH_BG_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_NO_BG_VS,
        shader_strings::CONSOLE_NO_BG_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::BACKING_VS,
        shader_strings::BACKING_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::SCANLINES_VS,
        shader_strings::SCANLINES_FS,
    ));

    // Build the backing frame-buffer
    let backing_fbo = Framebuffer::build_fbo(&gl, width_pixels as i32, height_pixels as i32);

    // Build a simple quad rendering vao
    let quad_vao = setup_quad(&gl);

    Fractal {
        backend: FractalPlatform {
            platform: PlatformGL {
                gl,
                quad_vao,
                context_wrapper: Some(WrappedContext {
                    el,
                    wc: windowed_context,
                }),
                backing_buffer: backing_fbo,
            },
        },
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders,
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        control: false,
        alt: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
    }
}
