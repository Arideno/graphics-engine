use crate::{point::Point, ray::Ray, intersection::Intersection, vector::Vector, matrix::Matrix};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
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
                    point: ray.at(t),
                    object: self.into()
                })
            } else {
                let t = (-half_b + root) / a;
                if t > 0.0 {
                    Some(Intersection {
                        t,
                        point: ray.at(t),
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

    pub fn apply_transform(self, transform: &Matrix) -> Sphere {
        Sphere {
            center: (&transform.multiply(&self.center.into())).into(),
            radius: self.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intersectable::Intersectable;

    use super::*;

    #[test]
    fn test_new() {
        let center = Point::new(5., 5., 4.);
        let radius = 5.;
        let sphere = Sphere::new(center, radius);
        assert_eq!(center, sphere.center);
        assert_eq!(radius, sphere.radius);
    }

    #[test]
    fn test_intersect() {
        let center = Point::new(0., 0., 0.);
        let radius = 1.;
        let sphere = Sphere::new(center, radius);
        let origin = Point::new(0., 2., 0.);
        let direction = Vector::new(0., -1., 0.);
        let ray = Ray::new(origin, direction);
        let intersection = sphere.intersect(ray);
        if let Some(intersection) = intersection {
            assert_eq!(Point::new(0., 1., 0.), intersection.point);
            assert_eq!(1., intersection.t);
            assert_eq!(Intersectable::from(sphere), intersection.object);
        } else {
            panic!("No intersection");
        }
    }

    #[test]
    fn test_normal_at_point() {
        let center = Point::new(5., 5., 4.);
        let radius = 5.;
        let sphere = Sphere::new(center, radius);
        let point = Point::new(5., 5., 9.);
        let result  = sphere.normal_at_point(point);
        assert_eq!(Vector::new(0., 0., 1.), result);
    }
}