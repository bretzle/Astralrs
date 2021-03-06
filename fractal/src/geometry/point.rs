use std::ops;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Serialize, Deserialize)]
/// Helper struct defining a 2D point in space.
pub struct Point {
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
}

impl Point {
    /// Create a new point from an x/y coordinate.
    #[inline]
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    /// Create a zero point
    #[inline]
    pub fn zero() -> Self {
        Point { x: 0, y: 0 }
    }

    #[inline]
    /// Create a point from a tuple of two i32s
    pub fn from_tuple(t: (i32, i32)) -> Self {
        Point { x: t.0, y: t.1 }
    }
}

///////////////////////////////////////////////////////////////////////////////////////
/// Overloads: We support basic point math

/// Support adding a point to a point
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(mut self, rhs: Point) -> Point {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

/// Support adding an int to a point
impl ops::Add<i32> for Point {
    type Output = Point;
    fn add(mut self, rhs: i32) -> Point {
        self.x += rhs;
        self.y += rhs;
        self
    }
}

/// Support subtracting a point from a point
impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(mut self, rhs: Point) -> Point {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

/// Support subtracting an int from a point
impl ops::Sub<i32> for Point {
    type Output = Point;
    fn sub(mut self, rhs: i32) -> Point {
        self.x -= rhs;
        self.y -= rhs;
        self
    }
}

/// Support multiplying a point by a point
impl ops::Mul<Point> for Point {
    type Output = Point;
    fn mul(mut self, rhs: Point) -> Point {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self
    }
}

/// Support multiplying a point by an int
impl ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(mut self, rhs: i32) -> Point {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

/// Support multiplying a point by an f32
impl ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(mut self, rhs: f32) -> Point {
        self.x = (self.x as f32 * rhs) as i32;
        self.y = (self.y as f32 * rhs) as i32;
        self
    }
}

/// Support dividing a point by a point
impl ops::Div<Point> for Point {
    type Output = Point;
    fn div(mut self, rhs: Point) -> Point {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self
    }
}

/// Support dividing a point by an int
impl ops::Div<i32> for Point {
    type Output = Point;
    fn div(mut self, rhs: i32) -> Point {
        self.x /= rhs;
        self.y /= rhs;
        self
    }
}

/// Support dividing a point by an f32
impl ops::Div<f32> for Point {
    type Output = Point;
    fn div(mut self, rhs: f32) -> Point {
        self.x = (self.x as f32 / rhs) as i32;
        self.y = (self.y as f32 / rhs) as i32;
        self
    }
}
