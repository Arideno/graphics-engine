use crate::{point::Point, ray::Ray};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn intersect(self, ray: Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_sq();
        let half_b = oc.dot(ray.direction);
        let c = oc.len_sq() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t > 0.0 {
                Some(t)
            } else {
                let t = (-half_b + root) / a;
                if t > 0.0 {
                    Some(t)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}