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

#[cfg(test)]
mod tests {
    use crate::intersectable::Intersectable;

    use super::*;

    #[test]
    fn test_new() {
        let v0 = Point::new(-0.5, 0., 0.);
        let v1 = Point::new(0., 1., 0.);
        let v2 = Point::new(0.5, 0., 0.);
        let triangle = Triangle::new(v0, v1, v2, None);
        assert_eq!(v0, triangle.v0);
        assert_eq!(v1, triangle.v1);
        assert_eq!(v2, triangle.v2);
    }

    #[test]
    fn test_intersect() {
        let v0 = Point::new(-0.5, 0., 0.);
        let v1 = Point::new(0., 1., 0.);
        let v2 = Point::new(0.5, 0., 0.);
        let triangle = Triangle::new(v0, v1, v2, None);
        let origin = Point::new(0., 0.5, 1.);
        let direction = Vector::new(0., 0., -1.);
        let ray = Ray::new(origin, direction);
        let intersection = triangle.intersect(ray);
        if let Some(intersection) = intersection {
            assert_eq!(Point::new(0., 0.5, 0.), intersection.point);
            assert_eq!(1., intersection.t);
            assert_eq!(Intersectable::from(triangle), intersection.object);
        } else {
            panic!("No intersection");
        }
    }

    #[test]
    fn test_normal_at_point() {
        let v0 = Point::new(-0.5, 0., 0.);
        let v1 = Point::new(0., 1., 0.);
        let v2 = Point::new(0.5, 0., 0.);
        let triangle = Triangle::new(v0, v1, v2, None);
        let point = Point::new(5., 5., 9.);
        let result = triangle.normal_at_point(point);
        assert_eq!(Vector::new(0., 0., 1.), result);
    }
}