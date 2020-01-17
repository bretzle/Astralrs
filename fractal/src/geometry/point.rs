//! A Helper class

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
/// Helper struct defining a 2D point in space.
pub struct Point {
    /// The x position
    pub x: i32,
    /// the y position
    pub y: i32,
}

impl Point {
    #[inline]
    /// Create a new point from an x/y coordinate.
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    #[inline]
    /// Create a zero point
    pub fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    #[inline]
    /// Create a point from a tuple of two i32s
    pub fn from_tuple(t: (i32, i32)) -> Self {
        Point { x: t.0, y: t.1 }
    }
}
