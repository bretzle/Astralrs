#![deny(
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! The Fractal Engine

#[macro_use]
extern crate lazy_static;

use crate::fractal::Fractal;

pub mod backend;
pub mod codepage437;
pub mod color;
pub mod console;
pub mod embedding;
pub mod fastnoise;
pub mod fieldofview;
pub mod fractal;
pub mod geometry;
pub mod gui_helpers;
pub mod parsing;
pub mod pathfinding;
pub mod random;
pub mod rex;
pub mod simple_console;
pub mod textblock;

#[macro_export]
macro_rules! embedded_resource {
    ($resource_name : ident, $filename : expr) => {
        const $resource_name: &'static [u8] = include_bytes!($filename);
    };
}

#[macro_export]
macro_rules! link_resource {
    ($resource_name : ident, $filename : expr) => {
        fractal::embedding::EMBED
            .lock()
            .unwrap()
            .add_resource($filename.to_string(), $resource_name);
    };
}

pub use glutin::event::VirtualKeyCode;

/// Implement this trait on your state struct, so the engine knows what to call on each tick.
pub trait GameState: 'static {
    fn tick(&mut self, ctx: &mut Fractal);
}

pub fn log<S: ToString>(message: S) {
    println!("{}", message.to_string());
}
