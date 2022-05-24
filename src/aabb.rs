use crate::{point::Point, vector::Vector, ray::Ray};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub min: Point,
    pub max: Point
}

impl AABB {
    pub fn with_bounds(min: Point, max: Point) -> AABB {
        AABB { min, max }
    }

    pub fn empty() -> AABB {
        AABB {
            min: Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Point::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY)
        }
    }

    pub fn full() -> AABB {
        AABB {
            min: Point::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            max: Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY)
        }
    }

    pub fn contains(self, p: Point) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    pub fn size(self) -> Vector {
        self.max - self.min
    }

    pub fn center(self) -> Point {
        self.min + (self.size() / 2.0)
    }

    pub fn merge(self, other: AABB) -> AABB {
        AABB::with_bounds(
            Point::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z)
            ),
            Point::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z)
            )
        )
    }

    pub fn longest_axis(self) -> usize {
        let size = self.size();
        if size.x > size.y && size.x > size.z {
            0
        } else if size.y > size.z {
            1
        } else {
            2
        }
    }

    pub fn intersect(self, ray: Ray) -> bool {
        let mut tmin = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.max.x - ray.origin.x) / ray.direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.max.y - ray.origin.y) / ray.direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if tmin > tymax || tymin > tmax {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.max.z - ray.origin.z) / ray.direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if tmin > tzmax || tzmin > tmax {
            return false;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        tmin >= 0.0 && tmax >= 0.0
    }
}

pub trait Bounded {
    fn aabb(&self) -> AABB;
}