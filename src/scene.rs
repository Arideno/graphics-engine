use crate::{intersectable::Intersectable, camera::Camera, ray::Ray, light::Light};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Intersectable>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Intersectable>, lights: Vec<Light>) -> Self {
        Self {
            camera,
            objects,
            lights
        }
    }

    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        self.camera.ray_for_pixel(x, y)
    }
}