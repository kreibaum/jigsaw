// To check if two points are basically equal.
const EPSILON: f32 = 0.0001;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl Point {
    pub fn abs(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn basically_equal(&self, other: Point) -> bool {
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
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
