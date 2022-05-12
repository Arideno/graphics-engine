use crate::{point::Point, vector::Vector, ray::Ray, intersection::Intersection};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub normal: Vector,
    pub point: Point
}

impl Plane {
    pub fn new(normal: Vector, point: Point) -> Plane {
        Plane { normal, point }
    }

    pub fn intersect(self, ray: Ray) -> Option<Intersection> {
        let denominator = self.normal.dot(ray.direction);
        if denominator > 0. {
            let numerator = self.normal.dot(self.point - ray.origin);
            let t = numerator / denominator;
            if t >= 0.0 {
                Some(Intersection {
                    t,
                    object: self.into()
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}