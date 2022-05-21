use std::fs;

use crate::{triangle::{Triangle}, point::Point, vector::Vector, matrix::Matrix};

#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn from_model<'a>(name: &'a str) -> Option<Self> {
        let contents = fs::read_to_string(name);
        if contents.is_err() {
            return None;
        }
        let contents = contents.unwrap();
        let mut points = vec![];
        let mut normals = vec![];
        let mut triangles = vec![];
        for line in contents.lines() {
            let parsed_line: Vec<&str> = line.split(" ").collect();
            if parsed_line[0] == "v" {
                let x = parsed_line[1].parse::<f32>().unwrap();
                let y = parsed_line[2].parse::<f32>().unwrap();
                let z = parsed_line[3].parse::<f32>().unwrap();
                points.push(Point::new(x, y, z));
            } else if parsed_line[0] == "vn" {
                let x = parsed_line[1].parse::<f32>().unwrap();
                let y = parsed_line[2].parse::<f32>().unwrap();
                let z = parsed_line[3].parse::<f32>().unwrap();
                normals.push(Vector::new(x, y, z));
            } else if parsed_line[0] == "f" {
                let mut triangle_data = Vec::with_capacity(3);
                for i in 1..=3 {
                    let parsed_indexes: Vec<&str> = parsed_line[i].split("/").collect();
                    if parsed_indexes.len() == 1 {
                        let point_index = parsed_indexes[0].parse::<usize>().unwrap();
                        triangle_data.push((points[point_index-1], None));
                    } else {
                        let point_index = parsed_indexes[0].parse::<usize>().unwrap();
                        let normal_index = parsed_indexes[2].parse::<usize>().unwrap();
                        triangle_data.push((points[point_index-1], Some(normals[normal_index-1])));
                    }
                }
                if triangle_data[0].1.is_none() {
                    let triangle = Triangle::new(triangle_data[0].0, triangle_data[1].0, triangle_data[2].0);
                    triangles.push(triangle);
                } else {
                    let triangle = Triangle::with_normals(
                        triangle_data[0].0,
                        triangle_data[1].0,
                        triangle_data[2].0,
                        triangle_data[0].1.unwrap(),
                        triangle_data[1].1.unwrap(),
                        triangle_data[2].1.unwrap(),
                    );
                    triangles.push(triangle);
                }
            }
        }
        Some(Mesh {
            triangles
        })
    }

    pub fn apply_transform(&self, transform: &Matrix) -> Mesh {
        let mut new_triangles = vec![];
        for triangle in &self.triangles {
            let new_triangle = triangle.apply_transform(transform);
            new_triangles.push(new_triangle);
        }
        Mesh {
            triangles: new_triangles
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_model() {
        Mesh::from_model("k.obj");
    }
}