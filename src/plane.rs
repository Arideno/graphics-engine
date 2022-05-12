use crate::{point::Point, vector::Vector, ray::Ray};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub normal: Vector,
    pub point: Point
}

impl Plane {
    pub fn new(normal: Vector, point: Point) -> Plane {
        Plane { normal, point }
    }

    pub fn intersect(&self, ray: Ray) -> Option<f64> {
        let denominator = self.normal.dot(ray.direction);
        if denominator > 0. {
            let numerator = self.normal.dot(ray.origin - self.point);
            let t = numerator / denominator;
            if t >= 0.0 {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    }
}