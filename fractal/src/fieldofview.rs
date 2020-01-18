use crate::geometry::circle_bresenham::BresenhamCircle;
use crate::geometry::line_vector::VectorLine;
use crate::geometry::point::Point;
use crate::pathfinding::Algorithm2D;
use std::collections::HashSet;

/// Calculates field-of-view for a map that supports Algorithm2D, returning a HashSet. This is a bit faster
/// than coercing the results into a vector, since internally it uses the set for de-duplication.
pub fn field_of_view_set(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> HashSet<Point> {
    let mut visible_points: HashSet<Point> =
        HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

    BresenhamCircle::new(start.x, start.y, range).for_each(|point| {
        scan_fov_line(start, point, fov_check, &mut visible_points);
    });

    visible_points
}

/// Calculates field-of-view for a map that supports Algorithm2D.
pub fn field_of_view(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> Vec<Point> {
    field_of_view_set(start, range, fov_check)
        .into_iter()
        .collect()
}

/// Helper method to scan along a line.
fn scan_fov_line(
    start: Point,
    end: Point,
    fov_check: &dyn Algorithm2D,
    visible_points: &mut HashSet<Point>,
) {
    let line = VectorLine::new(start, end);

    for target in line {
        if !fov_check.in_bounds(target) {
            // We're outside of the map
            break;
        }
        visible_points.insert(target);
        if fov_check.is_opaque(fov_check.point2d_to_index(target)) {
            // FoV is blocked
            break;
        }
    }
}
