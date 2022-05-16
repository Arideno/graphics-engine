use crate::triangle::Triangle;

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn from_model<'a>(name: &'a str) -> Self {
        loop {}
    }
}