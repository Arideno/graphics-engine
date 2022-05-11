use crate::{point::Point, vector::Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction: direction.normalize() }
    }

    pub fn at(self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}