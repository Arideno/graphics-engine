use crate::{intersectable::Intersectable, camera::Camera, ray::Ray};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Intersectable>,
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Intersectable>) -> Self {
        Self {
            camera,
            objects,
        }
    }

    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        self.camera.ray_for_pixel(x, y)
    }
}