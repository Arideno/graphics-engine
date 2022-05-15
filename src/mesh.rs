use crate::{triangle::Triangle, ray::Ray, intersection::Intersection};

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let mut min = std::f64::MAX;
        let mut min_intersection = None;
        for triangle in &self.triangles {
            if let Some(intersection) = triangle.intersect(ray) {
                if intersection.t < min {
                    min = intersection.t;
                    min_intersection = Some(intersection);
                }
            }
        }
        min_intersection
    }
}