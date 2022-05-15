use crate::{ray::Ray, sphere::Sphere, plane::Plane, intersection::Intersection, impl_froms, point::Point, vector::Vector, triangle::Triangle};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Intersectable {
    Sphere(Sphere),
    Plane(Plane),
    Triangle(Triangle)
}

impl Intersectable {
    pub fn intersect(self, ray: Ray) -> Option<Intersection> {
        match self {
            Intersectable::Sphere(sphere) => sphere.intersect(ray),
            Intersectable::Plane(plane) => plane.intersect(ray),
            Intersectable::Triangle(triangle) => triangle.intersect(ray)
        }
    }

    pub fn normal_at_point(self, point: Point) -> Vector {
        match self {
            Intersectable::Sphere(sphere) => sphere.normal_at_point(point),
            Intersectable::Plane(plane) => plane.normal_at_point(point),
            Intersectable::Triangle(triangle) => triangle.normal_at_point(point)
        }
    }
}

impl_froms!(Intersectable: Sphere, Plane, Triangle);