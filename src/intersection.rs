use crate::{intersectable::Intersectable, point::Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub point: Point,
    pub object: Intersectable,
}