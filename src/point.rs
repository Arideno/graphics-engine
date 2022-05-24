use std::ops::{Sub, Add, Index, IndexMut};

use crate::{vector::Vector, matrix::Matrix, m};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    pub fn apply_transform(self, transform: Matrix) -> Point {
        (&transform.multiply(&self.into())).into()
    }
}

impl From <(f32, f32, f32)> for Point {
    fn from (tuple: (f32, f32, f32)) -> Point {
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

impl From<&Matrix> for Point {
    fn from(matrix: &Matrix) -> Point {
        if matrix.rows == 4 {
            Point::new(matrix.get(0, 0), matrix.get(1, 0), matrix.get(2, 0))
        } else {
            Point::new(matrix.get(0, 0), matrix.get(0, 1), matrix.get(0, 2))
        }
    }
}

impl Into<Matrix> for Point {
    fn into(self) -> Matrix {
        m! [
            self.x;
            self.y;
            self.z;
            1.
        ]
    }
}

impl Index<usize> for Point {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds")
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds")
        }
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