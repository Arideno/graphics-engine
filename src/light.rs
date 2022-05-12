use crate::{vector::Vector, impl_froms};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Light {
    Directional(Directional)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Directional {
    pub direction: Vector
}

impl_froms!(Light: Directional);