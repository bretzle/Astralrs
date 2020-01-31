//! This module contains all kinds of pathfinding implementations

pub mod astar;
pub mod dijkstra;

use crate::geometry::Point;
use crate::geometry::Point3;

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

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm3D: BaseMap {
    /// Convert a Point (x/y) to an array index.
    fn point3d_to_index(&self, pt: Point3) -> i32;

    /// Convert an array index to a point.
    fn index_to_point3d(&self, idx: i32) -> Point3;
}
