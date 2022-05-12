use crate::{ray::Ray, sphere::Sphere, plane::Plane};

pub enum Intersectable {
    Sphere(Sphere),
    Plane(Plane)
}

impl Intersectable {
    pub fn intersect(&self, ray: Ray) -> Option<f64> {
        match self {
            Intersectable::Sphere(sphere) => sphere.intersect(ray),
            Intersectable::Plane(plane) => plane.intersect(ray)
        }
    }
}

macro_rules! impl_froms {
    ($n:ident: $($x:ident),*) => {
        $(
            impl From<$x> for $n {
                fn from(x: $x) -> $n {
                    $n::$x(x)
                }
            }
        )*
    }
}

impl_froms!(Intersectable: Sphere, Plane);