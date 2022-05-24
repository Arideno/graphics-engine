use crate::{point::Point, intersection::Intersection, ray::Ray, vector::Vector, matrix::Matrix, EPSILON, aabb::{Bounded, AABB}};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub n1: Option<Vector>,
    pub n2: Option<Vector>,
    pub n3: Option<Vector>,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point) -> Triangle {
        Triangle { v0, v1, v2, n1: None, n2: None, n3: None }
    }

    pub fn with_normals(v0: Point, v1: Point, v2: Point, n1: Vector, n2: Vector, n3: Vector) -> Triangle {
        Triangle { v0, v1, v2, n1: Some(n1.normalize()), n2: Some(n2.normalize()), n3: Some(n3.normalize()) }
    }

    pub fn intersect(self, ray: Ray) -> Option<Intersection> {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        let p = ray.direction.cross(e2);
        let det = e1.dot(p);

        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let t = ray.origin - self.v0;
        let u = t.dot(p) * inv_det;
        if u < 0. || u > 1. {
            return None;
        }

        let q = t.cross(e1);
        let v = ray.direction.dot(q) * inv_det;
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = e2.dot(q) * inv_det;

        if t > EPSILON {
            return Some(Intersection {
                t,
                object: self.into(),
                point: ray.at(t),
            });
        }

        None
    }

    pub fn normal_at_point(self, point: Point) -> Vector {
        if let Some(n1) = self.n1 {
            if let Some(n2) = self.n2 {
                if let Some(n3) = self.n3 {
                    let v0 = self.v0 - point;
                    let v1 = self.v1 - point;
                    let v2 = self.v2 - point;
                    let u = v1.cross(v2).dot(v0) / n1.dot(n1);
                    let v = v2.cross(v0).dot(v1) / n2.dot(n2);
                    let w = 1.0 - u - v;
                    return n1 * u + n2 * v + n3 * w;
                }
            }
        }
        
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        e1.cross(e2).normalize()
    }

    pub fn apply_transform(self, transform: &Matrix) -> Triangle {
        Triangle {
            v0: (&transform.multiply(&self.v0.into())).into(),
            v1: (&transform.multiply(&self.v1.into())).into(),
            v2: (&transform.multiply(&self.v2.into())).into(),
            n1: self.n1.map(|n| Vector::from(&transform.multiply(&n.into()))),
            n2: self.n2.map(|n| Vector::from(&transform.multiply(&n.into()))),
            n3: self.n3.map(|n| Vector::from(&transform.multiply(&n.into())))
        }
    }
}

impl Bounded for Triangle {
    fn aabb(&self) -> AABB {
        let mut min = self.v0;
        let mut max = self.v0;

        min.x = min.x.min(self.v1.x).min(self.v2.x);
        min.y = min.y.min(self.v1.y).min(self.v2.y);
        min.z = min.z.min(self.v1.z).min(self.v2.z);

        max.x = max.x.max(self.v1.x).max(self.v2.x);
        max.y = max.y.max(self.v1.y).max(self.v2.y);
        max.z = max.z.max(self.v1.z).max(self.v2.z);

        AABB::with_bounds(min, max)
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
        let triangle = Triangle::new(v0, v1, v2);
        assert_eq!(v0, triangle.v0);
        assert_eq!(v1, triangle.v1);
        assert_eq!(v2, triangle.v2);
    }

    #[test]
    fn test_intersect() {
        let v0 = Point::new(-0.5, 0., 0.);
        let v1 = Point::new(0., 1., 0.);
        let v2 = Point::new(0.5, 0., 0.);
        let triangle = Triangle::new(v0, v1, v2);
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
        let triangle = Triangle::new(v0, v1, v2);
        let point = Point::new(5., 5., 9.);
        let result = triangle.normal_at_point(point);
        assert_eq!(Vector::new(0., 0., 1.), result);
    }
}