use std::ops::{Sub, Add};

use crate::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

impl From <(f64, f64, f64)> for Point {
    fn from (tuple: (f64, f64, f64)) -> Point {
        Point { x: tuple.0, y: tuple.1, z: tuple.2 }
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let point = Point::new(5., 4., 3.);
        assert_eq!(5., point.x);
        assert_eq!(4., point.y);
        assert_eq!(3., point.z);
    }

    #[test]
    fn test_from() {
        let a = Point::from((1., 2., 3.));
        assert_eq!(1., a.x);
        assert_eq!(2., a.y);
        assert_eq!(3., a.z);
        let b: Point = (2., 3., 5.).into();
        assert_eq!(2., b.x);
        assert_eq!(3., b.y);
        assert_eq!(5., b.z);
    }
    #[test]
    fn test_add_vector() {
        let point = Point::new(5., 4., 3.);
        let vector = Vector::new(3., 2., 8.);
        let result = point + vector;
        assert_eq!(8., result.x);
        assert_eq!(6., result.y);
        assert_eq!(11., result.z);
    }

    #[test]
    fn test_sub_point() {
        let point1 = Point::new(5., 4., 3.);
        let point2  = Point::new(3., 2., 8.);
        let result = point1 - point2;
        assert_eq!(2., result.x);
        assert_eq!(2., result.y);
        assert_eq!(-5., result.z);
    }

    #[test]
    fn test_sub_vector() {
        let point = Point::new(5., 4., 3.);
        let vector = Vector::new(3., 2., 8.);
        let result = point - vector;
        assert_eq!(2., result.x);
        assert_eq!(2., result.y);
        assert_eq!(-5., result.z);
    }
}