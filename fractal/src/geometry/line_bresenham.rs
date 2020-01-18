//! Original at: https://github.com/mbr/bresenham-rs/blob/master/src/lib.rs
//! Modified to use more FRACTAL-friendly types
use crate::geometry::point::Point;
use core::iter::Iterator;

/// Line-drawing iterator
pub struct Bresenham {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    x1: i32,
    diff: i32,
    octant: Octant,
}

struct Octant(u8);

impl Octant {
    /// adapted from http://codereview.stackexchange.com/a/95551
    #[inline]
    fn from_points(start: Point, end: Point) -> Octant {
        let mut dx = end.x - start.x;
        let mut dy = end.y - start.y;

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }

        Octant(octant)
    }

    #[inline]
    fn to_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => Point::new(p.x, p.y),
            1 => Point::new(p.y, p.x),
            2 => Point::new(p.y, -p.x),
            3 => Point::new(-p.x, p.y),
            4 => Point::new(-p.x, -p.y),
            5 => Point::new(-p.y, -p.x),
            6 => Point::new(-p.y, p.x),
            7 => Point::new(p.x, -p.y),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn from_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => Point::new(p.x, p.y),
            1 => Point::new(p.y, p.x),
            2 => Point::new(-p.y, p.x),
            3 => Point::new(-p.x, p.y),
            4 => Point::new(-p.x, -p.y),
            5 => Point::new(-p.y, -p.x),
            6 => Point::new(p.y, -p.x),
            7 => Point::new(p.x, -p.y),
            _ => unreachable!(),
        }
    }
}

impl Bresenham {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: Point, end: Point) -> Bresenham {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant0(start);
        let end = octant.to_octant0(end);

        let dx = end.x - start.x;
        let dy = end.y - start.y;

        Bresenham {
            x: start.x,
            y: start.y,
            dx: dx,
            dy: dy,
            x1: end.x,
            diff: dy - dx,
            octant: octant,
        }
    }
}

impl Iterator for Bresenham {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            return None;
        }

        let p = Point::new(self.x, self.y);

        if self.diff >= 0 {
            self.y += 1;
            self.diff -= self.dx;
        }

        self.diff += self.dy;

        // loop inc
        self.x += 1;

        Some(self.octant.from_octant0(p))
    }
}
