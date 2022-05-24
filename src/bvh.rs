use crate::{aabb::{AABB, Bounded}, intersectable::Intersectable, ray::Ray, intersection::Intersection};

enum Node {
    Leaf(Vec<Box<Intersectable>>),
    Branch(AABB, Box<Node>, Box<Node>)
}

impl Node {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        match self {
            Node::Leaf(objects) => {
                let mut intersections = Vec::new();
                for object in objects {
                    if let Some(intersection) = object.intersect(ray) {
                        intersections.push(intersection);
                    }
                }
                if intersections.is_empty() {
                    None
                } else {
                    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
                    Some(intersections[0])
                }
            },
            Node::Branch(aabb, left, right) => {
                if !aabb.intersect(ray) {
                    None
                } else {
                    let left_intersection = left.intersect(ray);
                    let right_intersection = right.intersect(ray);
                    if let Some(left_intersection) = left_intersection {
                        if let Some(right_intersection) = right_intersection {
                            if left_intersection.t < right_intersection.t {
                                Some(left_intersection)
                            } else {
                                Some(right_intersection)
                            }
                        } else {
                            Some(left_intersection)
                        }
                    } else {
                        right_intersection
                    }
                }
            }
        }
    }
}

pub struct BVH {
    root: Box<Node>
}

impl BVH {
    pub fn new(objects: Vec<Box<Intersectable>>, depth: u32, max_depth: u32) -> BVH {
        BVH {
            root: Box::new(BVH::build_tree(objects, depth, max_depth))
        }
    }

    fn build_tree(objects: Vec<Box<Intersectable>>, depth: u32, max_depth: u32) -> Node {
        if depth >= max_depth || objects.len() <= 100_000 {
            Node::Leaf(objects)
        } else {
            let mut aabb = AABB::empty();
            for object in &objects {
                if object.aabb() != AABB::full() {
                    aabb = aabb.merge(object.aabb());
                }
            }
            let axis = aabb.longest_axis();

            let mut left = Vec::new();
            let mut right = Vec::new();

            let split_point = match axis {
                0 => aabb.min.x + (aabb.max.x - aabb.min.x) / 2.,
                1 => aabb.min.y + (aabb.max.y - aabb.min.y) / 2.,
                2 => aabb.min.z + (aabb.max.z - aabb.min.z) / 2.,
                _ => panic!("invalid axis")
            };

            for object in objects {
                if object.aabb() == AABB::full() {
                    left.push(object.clone());
                    right.push(object.clone());
                    continue;
                }
                if object.aabb().min[axis] <= split_point {
                    left.push(object);
                } else {
                    right.push(object);
                }
            }

            let (left_tree, right_tree) = rayon::join(
                || BVH::build_tree(left, depth + 1, max_depth),
                || BVH::build_tree(right, depth + 1, max_depth)
            );
            Node::Branch(aabb, Box::new(left_tree), Box::new(right_tree))
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        self.root.intersect(ray)
    }
}