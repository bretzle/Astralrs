//! The Actual core of the fractal engine

use crate::backend::font::Font;
use crate::backend::init::init_raw;
use crate::backend::mainloop;
use crate::backend::shader::Shader;
use crate::backend::FractalPlatform;
use crate::color::RGB;
use crate::console::Console;
use crate::geometry::Rect;
use crate::rex::XpFile;
use crate::rex::XpLayer;
use crate::simple_console::SimpleConsole;
use crate::GameState;
use glutin::event::VirtualKeyCode;
use std::any::Any;

/// A display console, used internally to provide console render support.
/// Public in case you want to play with it, or access it directly.
pub struct DisplayConsole {
    /// Instance of a console that lives on the heap
    pub console: Box<dyn Console>,
    /// shader index
    pub shader_index: usize,
    /// font index
    pub font_index: usize,
}

/// An FRACTAL context.
pub struct Fractal {
    /// A OpenGL backend context
    pub backend: FractalPlatform,
    /// Width of the window in pixels
    pub width_pixels: u32,
    /// Height of the window in pixels
    pub height_pixels: u32,
    /// A Vector of fonts the game will use
    pub fonts: Vec<Font>,
    /// A Vector of shaders the game will use
    pub shaders: Vec<Shader>,
    /// A Vector of Consoles the game can display
    pub consoles: Vec<DisplayConsole>,
    /// The current frames per second
    pub fps: f32,
    /// The time in milliseconds it took to process the last frame
    pub frame_time_ms: f32,
    /// Index to the current console being rendered
    pub active_console: usize,
    /// Contains a keycode if a key was pressed
    pub key: Option<VirtualKeyCode>,
    /// The position of the mouse
    pub mouse_pos: (i32, i32),
    /// True if the mouse was left clicked
    pub left_click: bool,
    /// True if a shift was held down
    pub shift: bool,
    /// True if Control was held down
    pub control: bool,
    /// True if alt was held down
    pub alt: bool,
    /// Link to a website ? TODO
    pub web_button: Option<String>,
    /// True if the game is shutting down
    pub quitting: bool,
    /// Should console use included scanlines shader
    pub post_scanlines: bool,
    /// Should console use included screenburn shader
    pub post_screenburn: bool,
}

impl Fractal {
    /// Initializes an OpenGL context and a window, stores the info in the Fractal structure.
    pub fn init_raw<S: ToString>(
        width_pixels: u32,
        height_pixels: u32,
        window_title: S,
    ) -> Fractal {
        init_raw(width_pixels, height_pixels, window_title)
    }

    /// Quick initialization for when you just want an 8x8 font terminal
    pub fn init_simple8x8<S: ToString>(
        width_chars: u32,
        height_chars: u32,
        window_title: S,
        path_to_shaders: S,
    ) -> Fractal {
        let font_path = format!("{}/terminal8x8.png", &path_to_shaders.to_string());
        let mut context = Fractal::init_raw(width_chars * 8, height_chars * 8, window_title);
        let font = context.register_font(Font::load(&font_path, (8, 8)));
        context.register_console(
            SimpleConsole::init(width_chars, height_chars, &context.backend),
            font,
        );
        context
    }

    /// Quick initialization for when you just want an 8x16 VGA font terminal
    pub fn init_simple8x16<S: ToString>(
        width_chars: u32,
        height_chars: u32,
        window_title: S,
        path_to_shaders: S,
    ) -> Fractal {
        let font_path = format!("{}/vga8x16.png", &path_to_shaders.to_string());
        let mut context = Fractal::init_raw(width_chars * 8, height_chars * 16, window_title);
        let font = context.register_font(Font::load(&font_path, (8, 16)));
        context.register_console(
            SimpleConsole::init(width_chars, height_chars, &context.backend),
            font,
        );
        context
    }

    /// Registers a font, and returns its handle number. Also loads it into OpenGL.
    pub fn register_font(&mut self, mut font: Font) -> usize {
        font.setup_gl_texture(&self.backend);
        font.bind_texture(&self.backend);
        self.fonts.push(font);
        self.fonts.len() - 1
    }

    /// Registers a new console terminal for output, and returns its handle number.
    pub fn register_console(&mut self, new_console: Box<dyn Console>, font_index: usize) -> usize {
        self.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 0,
        });
        self.consoles.len() - 1
    }

    /// Registers a new console terminal for output, and returns its handle number. This variant requests
    /// that the new console not render background colors, so it can be layered on top of other consoles.
    pub fn register_console_no_bg(
        &mut self,
        new_console: Box<dyn Console>,
        font_index: usize,
    ) -> usize {
        self.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 1,
        });
        self.consoles.len() - 1
    }

    /// Sets the currently active console number.
    pub fn set_active_console(&mut self, id: usize) {
        self.active_console = id;
    }

    /// Applies the current physical mouse position to the active console, and translates the coordinates into that console's coordinate space.
    pub fn mouse_pos(&self) -> (i32, i32) {
        let max_sizes = self.consoles[self.active_console].console.get_char_size();

        (
            iclamp(
                self.mouse_pos.0 * max_sizes.0 as i32 / self.width_pixels as i32,
                0,
                max_sizes.0 as i32 - 1,
            ),
            iclamp(
                self.mouse_pos.1 * max_sizes.1 as i32 / self.height_pixels as i32,
                0,
                max_sizes.1 as i32 - 1,
            ),
        )
    }

    /// Tells the game to quit
    pub fn quit(&mut self) {
        self.quitting = true;
    }

    /// Render a REX Paint (https://www.gridsagegames.com/rexpaint/) file as a sprite.
    /// The sprite will be offset by offset_x and offset_y.
    /// Transparent cells will not be rendered.
    // pub fn render_xp_sprite(&mut self, xp: &XpFile, x: i32, y: i32) {
    //     rex::xp_to_console(xp, &mut self.consoles[self.active_console].console, x, y);
    // }

    /// Saves the entire console stack to a REX Paint xp file. If your consoles are of
    /// varying sizes, the file format supports it - but REX doesn't. So you may want to
    /// avoid that. You can also get individual layers with to_xp_layer.
    pub fn to_xp_file(&self, width: usize, height: usize) -> XpFile {
        let mut xp = XpFile::new(width, height);

        xp.layers
            .push(self.consoles[self.active_console].console.to_xp_layer());

        if self.consoles.len() > 1 {
            for layer in self.consoles.iter().skip(1) {
                xp.layers.push(layer.console.to_xp_layer());
            }
        }

        xp
    }

    /// Enable scanlines post-processing effect.
    pub fn with_post_scanlines(&mut self, with_burn: bool) {
        self.post_scanlines = true;
        self.post_screenburn = with_burn;
    }
}

impl Console for Fractal {
    // A couple of ones we'll never use
    fn rebuild_if_dirty(&mut self, _platform: &FractalPlatform) {}
    fn gl_draw(&mut self, _font: &Font, _shader: &Shader, _platform: &FractalPlatform) {}

    fn get_char_size(&self) -> (u32, u32) {
        self.consoles[self.active_console].console.get_char_size()
    }

    fn resize_pixels(&mut self, width: u32, height: u32) {
        self.width_pixels = width;
        self.height_pixels = height;

        for c in self.consoles.iter_mut() {
            c.console.resize_pixels(width, height);
        }
    }

    // Implement pass-through to active console

    fn at(&self, x: i32, y: i32) -> usize {
        self.consoles[self.active_console].console.at(x, y)
    }
    fn cls(&mut self) {
        self.consoles[self.active_console].console.cls();
    }
    fn cls_bg(&mut self, background: RGB) {
        self.consoles[self.active_console]
            .console
            .cls_bg(background);
    }
    fn print(&mut self, x: i32, y: i32, output: &str) {
        self.consoles[self.active_console]
            .console
            .print(x, y, output);
    }
    fn print_color(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, output: &str) {
        self.consoles[self.active_console]
            .console
            .print_color(x, y, fg, bg, output);
    }
    fn set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        self.consoles[self.active_console]
            .console
            .set(x, y, fg, bg, glyph);
    }
    fn set_bg(&mut self, x: i32, y: i32, bg: RGB) {
        self.consoles[self.active_console].console.set_bg(x, y, bg);
    }
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .draw_box(x, y, width, height, fg, bg);
    }
    fn draw_box_double(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .draw_box_double(x, y, width, height, fg, bg);
    }
    fn draw_hollow_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .draw_hollow_box(x, y, width, height, fg, bg);
    }
    fn draw_hollow_box_double(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGB,
        bg: RGB,
    ) {
        self.consoles[self.active_console]
            .console
            .draw_hollow_box_double(x, y, width, height, fg, bg);
    }
    fn draw_bar_horizontal(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        self.consoles[self.active_console]
            .console
            .draw_bar_horizontal(x, y, width, n, max, fg, bg);
    }
    fn draw_bar_vertical(
        &mut self,
        x: i32,
        y: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        self.consoles[self.active_console]
            .console
            .draw_bar_vertical(x, y, height, n, max, fg, bg);
    }
    fn fill_region(&mut self, target: Rect, glyph: u8, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .fill_region(target, glyph, fg, bg);
    }
    fn get(&self, x: i32, y: i32) -> Option<(&u8, &RGB, &RGB)> {
        self.consoles[self.active_console].console.get(x, y)
    }
    fn print_centered(&mut self, y: i32, text: &str) {
        self.consoles[self.active_console]
            .console
            .print_centered(y, text);
    }
    fn print_color_centered(&mut self, y: i32, fg: RGB, bg: RGB, text: &str) {
        self.consoles[self.active_console]
            .console
            .print_color_centered(y, fg, bg, text);
    }
    fn to_xp_layer(&self) -> XpLayer {
        self.consoles[self.active_console].console.to_xp_layer()
    }
    fn set_offset(&mut self, x: f32, y: f32) {
        self.consoles[self.active_console].console.set_offset(x, y);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Runs the FRACTAL application, calling into the provided gamestate handler every tick.
pub fn main_loop<GS: GameState>(fractal: Fractal, gamestate: GS) {
    mainloop::main_loop(fractal, gamestate);
}

/// For A-Z menus, translates the keys A through Z into 0..25
pub fn letter_to_option(key: VirtualKeyCode) -> i32 {
    match key {
        VirtualKeyCode::A => 0,
        VirtualKeyCode::B => 1,
        VirtualKeyCode::C => 2,
        VirtualKeyCode::D => 3,
        VirtualKeyCode::E => 4,
        VirtualKeyCode::F => 5,
        VirtualKeyCode::G => 6,
        VirtualKeyCode::H => 7,
        VirtualKeyCode::I => 8,
        VirtualKeyCode::J => 9,
        VirtualKeyCode::K => 10,
        VirtualKeyCode::L => 11,
        VirtualKeyCode::M => 12,
        VirtualKeyCode::N => 13,
        VirtualKeyCode::O => 14,
        VirtualKeyCode::P => 15,
        VirtualKeyCode::Q => 16,
        VirtualKeyCode::R => 17,
        VirtualKeyCode::S => 18,
        VirtualKeyCode::T => 19,
        VirtualKeyCode::U => 20,
        VirtualKeyCode::V => 21,
        VirtualKeyCode::W => 22,
        VirtualKeyCode::X => 23,
        VirtualKeyCode::Y => 24,
        VirtualKeyCode::Z => 25,
        _ => -1,
    }
}

// Since num::clamp is still experimental, this is a simple integer clamper.
fn iclamp(val: i32, min: i32, max: i32) -> i32 {
    i32::max(min, i32::min(val, max))
}
