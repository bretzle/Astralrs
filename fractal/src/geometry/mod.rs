use std::cmp::{max, min};

mod circle_bresenham;
mod line_bresenham;
mod line_vector;
mod lines;
mod point;
mod point3;
mod rect;

pub use circle_bresenham::BresenhamCircle;
pub use line_bresenham::Bresenham;
pub use line_vector::VectorLine;
pub use lines::*;
pub use point::Point;
pub use point3::Point3;
pub use rect::Rect;

/// Enumeration of available 2D Distance algorithms
pub enum DistanceAlg {
    Pythagoras,
    PythagorasSquared,
    Manhattan,
    Chebyshev,
}

impl DistanceAlg {
    /// Provides a 2D distance between points, using the specified algorithm.
    pub fn distance2d(self, start: Point, end: Point) -> f32 {
        match self {
            DistanceAlg::Pythagoras => distance2d_pythagoras(start, end),
            DistanceAlg::PythagorasSquared => distance2d_pythagoras_squared(start, end),
            DistanceAlg::Manhattan => distance2d_manhattan(start, end),
            DistanceAlg::Chebyshev => distance2d_chebyshev(start, end),
        }
    }
    /// Provides a 3D distance between points, using the specified algorithm.
    pub fn distance3d(self, start: Point3, end: Point3) -> f32 {
        match self {
            DistanceAlg::Pythagoras => distance3d_pythagoras(start, end),
            DistanceAlg::PythagorasSquared => distance3d_pythagoras_squared(start, end),
            DistanceAlg::Manhattan => distance3d_manhattan(start, end),
            DistanceAlg::Chebyshev => distance3d_pythagoras(start, end),
        }
    }
}

/// Enumeration of available 2D Distance algorithms
pub enum LineAlg {
    Bresenham,
    Vector,
}

/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
fn distance2d_pythagoras_squared(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    (dx * dx) + (dy * dy)
}

/// Calculates a Manhattan distance between two points
fn distance2d_manhattan(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    dx + dy
}

/// Calculates a Manhattan distance between two 3D points
fn distance3d_manhattan(start: Point3, end: Point3) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    let dz = (max(start.z, end.z) - min(start.z, end.z)) as f32;
    dx + dy + dz
}

/// Calculates a Chebyshev distance between two points
/// See: http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html
fn distance2d_chebyshev(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    if dx > dy {
        (dx - dy) + 1.0 * dy
    } else {
        (dy - dx) + 1.0 * dx
    }
}

/// Calculates a Pythagoras distance between two 3D points.
fn distance3d_pythagoras_squared(start: Point3, end: Point3) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    let dz = (max(start.z, end.z) - min(start.z, end.z)) as f32;
    (dx * dx) + (dy * dy) + (dz * dz)
}

/// Calculates a Pythagoras distance between two points.
fn distance2d_pythagoras(start: Point, end: Point) -> f32 {
    let dsq = distance2d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

/// Calculates a Pythagoras distance between two 3D points.
fn distance3d_pythagoras(start: Point3, end: Point3) -> f32 {
    let dsq = distance3d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

/// From a given start point, project forward radius units at an angle of angle_radians degrees.
/// 0 Degrees is north (negative Y), 90 degrees is east (positive X)
pub fn project_angle(start: Point, radius: f32, angle_radians: f32) -> Point {
    let degrees_radians = angle_radians + std::f32::consts::PI;
    Point::new(
        (0.0 - (start.x as f32 + radius * f32::sin(degrees_radians))) as i32,
        (start.y as f32 + radius * f32::cos(degrees_radians)) as i32,
    )
}
