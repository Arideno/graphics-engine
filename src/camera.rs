use crate::{point::Point, vector::Vector, ray::Ray};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub origin: Point,
    pub width: u32,
    pub height: u32
}

impl Camera {
    pub fn new(origin: Point, width: u32, height: u32) -> Camera {
        Camera { origin, width, height }
    }
    
    pub fn ray_for_pixel(self, x: u32, y: u32) -> Ray {
        let x_factor = (x as f64 + 0.5) / self.width as f64;
        let y_factor = (y as f64 + 0.5) / self.height as f64;
        let world_x = x_factor * 2. - 1.;
        let world_y = y_factor * 2. - 1.;
        let world_z = -1.;
        let direction = Vector::new(world_x, world_y, world_z);
        Ray::new(self.origin, direction)
    }
}