//! Font

#[derive(Debug, Copy, Clone)]
/// Represents a font
pub struct Font {}

impl Font {
    /// Loads a font file.
    pub fn load<S: ToString>(filename: S, size: (u32, u32)) -> Self {
        unimplemented!()
    }
}
