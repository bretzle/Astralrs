#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
)]

//! The Fractal Engine


pub mod color;
pub mod fractal;
pub mod font;
pub mod console;
pub mod graphics;

pub use glutin::event::VirtualKeyCode;
pub use graphics::shader::*;

/// Implement this trait on your state struct so the fractal will know what to
/// call tick on
pub trait GameState: 'static {
    /// Takes the current state and generate the next game state
    fn tick(&mut self);
}

#[derive(Debug, Copy, Clone)]
/// Wrapper for an OpenGl Platform
pub struct Platform {

}