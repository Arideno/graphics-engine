use crate::{point::Point, vector::Vector, ray::Ray, intersection::Intersection, matrix::Matrix};

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
        let denominator = -self.normal.dot(ray.direction);
        if denominator > 0. {
            let numerator = -self.normal.dot(self.point - ray.origin);
            let t = numerator / denominator;
            if t >= 0.0 {
                Some(Intersection {
                    t,
                    point: ray.at(t),
                    object: self.into()
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn normal_at_point(self, _point: Point) -> Vector {
        self.normal
    }

    pub fn apply_transform(self, transform: &Matrix) -> Plane {
        Plane {
            normal: Vector::from(&transform.multiply(&self.normal.into())).normalize(),
            point: (&transform.multiply(&self.point.into())).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intersectable::Intersectable;

    use super::*;

    #[test]
    fn test_new() {
        let point = Point::new(5., 5., 4.);
        let vector = Vector::new(3., 0., 4.);
        let plane = Plane::new(vector, point);
        assert_eq!(vector, plane.normal);
        assert_eq!(point, plane.point);
    }

    #[test]
    fn test_intersect() {
        let point = Point::new(0., 0., 0.);
        let normal = Vector::new(0., 1., 0.);
        let plane = Plane::new(normal, point);
        let origin = Point::new(0., 1., 1.);
        let direction = Vector::new(0., -1., -1.);
        let ray = Ray::new(origin, direction);
        let intersection = plane.intersect(ray);
        if let Some(intersection) = intersection {
            assert_eq!(Point::new(0., 0., 0.), intersection.point);
            assert_eq!((2f32).sqrt(), intersection.t);
            assert_eq!(Intersectable::from(plane), intersection.object);
        } else {
            panic!("No intersection");
        }
    }

    #[test]
    fn test_normal_at_point() {
        let point = Point::new(5., 5., 4.);
        let vector = Vector::new(3., 0., 4.);
        let plane = Plane::new(vector, point);
        let result  = plane.normal_at_point(point);
        assert_eq!(result, plane.normal);
    }
}