#![allow(unsafe_code)]

//! The main engine

use crate::graphics::setup_quad;
use crate::graphics::WrappedContext;
use crate::graphics::PlatformGL;
use crate::color::Color;
use crate::console::Console;
use crate::font::Font;
use crate::graphics::shader::Shader;
use crate::Platform;
use crate::SimpleConsole;
use crate::graphics::shader_strings;
use crate::graphics::framebuffer::Framebuffer;
use glutin::event::VirtualKeyCode;
use glutin::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};

/// A display console used internally to provide console render support.
pub struct DisplayConsole {
    ///
    pub console: Box<dyn Console>,
    ///
    pub font_index: usize,
    ///
    pub shader_index: usize,
}

/// Acts as a context for the engine
pub struct Fractal {
    ///
    pub backend: Platform,
    ///
    pub width_pixels: u32,
    ///
    pub height_pixels: u32,
    ///
    pub fonts: Vec<Font>,
    ///
    pub shaders: Vec<Shader>,
    ///
    pub consoles: Vec<DisplayConsole>,
    ///
    pub fps: f32,
    ///
    pub frame_time_ms: f32,
    ///
    pub active_console: usize,
    ///
    pub key: Option<VirtualKeyCode>,
    ///
    pub mouse_pos: (i32, i32),
    ///
    pub left_click: bool,
    ///
    pub shift: bool,
    ///
    pub control: bool,
    ///
    pub alt: bool,
    ///
    pub web_button: Option<String>,
    ///
    pub quitting: bool,
    ///
    pub post_scanlines: bool,
    ///
    pub post_screenburn: bool,
}

impl Fractal {
    /// Quick initialization for when you want a basic setup
    pub fn init_simple<S: ToString>(
        width_chars: u32,
        height_chars: u32,
        title: S,
        shaders: S,
    ) -> Self {
        let font_path = format!("{}/terminal8x8.png", &shaders.to_string());
        let mut context = Fractal::init_raw(width_chars * 8, height_chars * 8, title);
        let font = context.register_font(Font::load(&font_path.to_string(), (8, 8)));

        context.register_console(
            SimpleConsole::init(width_chars, height_chars, &context.backend),
            font,
        );

        context
    }

    /// Initializes the Engine.
    pub fn init_raw<S: ToString>(width_pixels: u32, height_pixels: u32, window_title: S) -> Self {
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

        let gl = glow::Context::from_loader_function(|ptr| {
            windowed_context.get_proc_address(ptr)// as *const _
        });

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
            backend: Platform {
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

    /// Registers a font with the engine.
    pub fn register_font(&mut self, mut font: Font) -> usize {
        font.setup_gl_texture(&self.backend);
        font.bind_texture(&self.backend);
        self.fonts.push(font);
        self.fonts.len() - 1
    }

    /// Regjsters a console with the engine.
    pub fn register_console(&mut self, new_console: Box<dyn Console>, font_index: usize) -> usize {
        self.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 0,
        });
        self.consoles.len() - 1
    }
}

impl Console for Fractal {
    fn cls(&mut self) {
        self.consoles[self.active_console].console.cls();
    }

    fn set(&mut self, x: i32, y: i32, fg: Color, bg: Color, glyph: u8) {
        self.consoles[self.active_console].console.set(x, y, fg, bg, glyph);
    }

    fn draw(&mut self, font: &Font, shader: &Shader, platform: &Platform) {
        unimplemented!();
    }

    fn resize_pixels(&mut self, width: u32, height: u32) {
        self.width_pixels = width;
        self.height_pixels = height;

        for c in self.consoles.iter_mut() {
            c.console.resize_pixels(width, height);
        }
    }
}
