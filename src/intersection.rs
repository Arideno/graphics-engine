use crate::intersectable::Intersectable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Intersectable,
}