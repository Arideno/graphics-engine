use crate::{point::Point, intersection::Intersection, ray::Ray, vector::Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    v0: Point,
    v1: Point,
    v2: Point,
    normal: Option<Vector>
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point, normal: Option<Vector>) -> Triangle {
        Triangle { v0, v1, v2, normal }
    }

    pub fn intersect(self, ray: Ray) -> Option<Intersection> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;

        let p = ray.direction.cross(e2);
        let det = e1.dot(p);

        if det == 0. {
            None
        } else {
            let inv_det = 1. / det;
            let t = ray.origin - self.v0;
            let u = t.dot(p) * inv_det;
            if u < 0. || u > 1. {
                None
            } else {
                let q = t.cross(e1);
                let v = ray.direction.dot(q) * inv_det;
                if v < 0. || u + v > 1. {
                    None
                } else {
                    let t = e2.dot(q) * inv_det;
                    Some(Intersection {
                        t: t,
                        point: ray.at(t),
                        object: self.into()
                    })
                }
            }
        }
    }

    pub fn normal_at_point(self, _point: Point) -> Vector {
        if let Some(normal) = self.normal {
            return normal;
        }

        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;

        let normal = e2.cross(e1).normalize();

        normal
    }
}