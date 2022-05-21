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

    pub fn at(self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let point = Point::new(5., 5., 4.);
        let vector = Vector::new(3., 0., 4.);
        let normalized_vector = vector.normalize();
        let ray = Ray::new(point, vector);
        assert_eq!(point, ray.origin);
        assert_eq!(normalized_vector, ray.direction);
    }

    #[test]
    fn test_at() {
        let point = Point::new(5., 5., 4.);
        let vector = Vector::new(3., 0., 4.);
        let ray = Ray::new(point, vector);
        let result = ray.at(5.);
        assert_eq!(Point::new(8., 5., 8.), result);
    }
}