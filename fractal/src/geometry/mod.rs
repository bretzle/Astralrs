//! Module for geometry

pub mod circle_bresenham;
pub mod line_vector;
pub mod point;

use point::Point;
use std::cmp::{max, min};

/// Implement this trait to support path-finding functions.
pub trait BaseMap {
    /// True is you can see through the tile, false otherwise.
    fn is_opaque(&self, idx: i32) -> bool;

    /// Return a vector of tile indices to which one can path from the idx.
    /// These do NOT have to be contiguous - if you want to support teleport pads, that's awesome.
    fn get_available_exits(&self, idx: i32) -> Vec<(i32, f32)>;

    /// Return the distance you would like to use for path-finding. Generally, Pythagoras distance (implemented in geometry)
    /// is fine, but you might use Manhattan or any other heuristic that fits your problem.
    fn get_pathing_distance(&self, idx1: i32, idx2: i32) -> f32;
}

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm2D: BaseMap {
    /// Convert a Point (x/y) to an array index.
    fn point2d_to_index(&self, pt: Point) -> i32;

    /// Convert an array index to a point.
    fn index_to_point2d(&self, idx: i32) -> Point;

    /// Optional - check that an x/y coordinate is within the map bounds
    fn in_bounds(&self, _pos: Point) -> bool {
        true
    }
}

/// Enumeration of available 2D Distance algorithms
pub enum DistanceAlg {
    /// todo Document me
    Pythagoras,
}

impl DistanceAlg {
    /// Provides a 2D distance between points, using the specified algorithm.
    pub fn distance2d(self, start: Point, end: Point) -> f32 {
        match self {
            DistanceAlg::Pythagoras => distance2d_pythagoras(start, end),
        }
    }
}

/// Calculates a Pythagoras distance between two points.
fn distance2d_pythagoras(start: Point, end: Point) -> f32 {
    let dsq = distance2d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
fn distance2d_pythagoras_squared(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    (dx * dx) + (dy * dy)
}
