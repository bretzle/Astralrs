#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
)]

//! The Fractal Engine

/// Implement this trait on your state struct so the fractal will know what to
/// call tick on
pub trait GameState: 'static {
    /// Takes the current state and generate the next game state
    fn tick(&mut self);
}
