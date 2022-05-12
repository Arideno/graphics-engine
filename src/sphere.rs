use crate::{point::Point, ray::Ray, intersection::Intersection, vector::Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn intersect(self, ray: Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_sq();
        let half_b = oc.dot(ray.direction);
        let c = oc.len_sq() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t > 0.0 {
                Some(Intersection {
                    t,
                    object: self.into()
                })
            } else {
                let t = (-half_b + root) / a;
                if t > 0.0 {
                    Some(Intersection {
                        t,
                        object: self.into()
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn normal_at_point(self, point: Point) -> Vector {
        (point - self.center).normalize()
    }
}