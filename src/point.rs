#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
    pub fn abs(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Implement subtraction for points.
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Implement addition for points.
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Implement scalar multiplication for points.
impl std::ops::Mul<f32> for Point {
    type Output = Point;
    fn mul(self, other: f32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
