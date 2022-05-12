use crate::{point::Point, vector::Vector, ray::Ray};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    pub vertical: Vector,
    width: u32,
    height: u32,
}

impl Camera {
    pub fn new(origin: Point, vfov: f64, aspect: f64, height: u32) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        
        let viewport_height = 2.0 * half_height;
        let viewport_width = aspect * viewport_height;

        let focal_length = 1.;

        let horizontal = Vector::new(viewport_width, 0., 0.);
        let vertical = Vector::new(0., viewport_height, 0.);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - Vector::new(0., 0., focal_length),
            height,
            width: (height as f64 * aspect) as u32,
        }
    }

    pub fn ray_for_pixel(self, x: u32, y: u32) -> Ray {
        let u = (x as f64 + 0.5) / self.width as f64;
        let v = (y as f64 + 0.5) / self.height as f64;

        let point_on_screen = self.lower_left_corner + u * self.horizontal + v * self.vertical;

        Ray::new(self.origin, point_on_screen - self.origin)
    }
}