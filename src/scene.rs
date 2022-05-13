use crate::{intersectable::Intersectable, camera::Camera, ray::Ray, light::Light, intersection::Intersection};

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

    pub fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let mut closest_intersection: Option<Intersection> = None;

        for object in &self.objects {
            if let Some(intersection) = object.intersect(ray) {
                if closest_intersection.is_none() || intersection.t < closest_intersection.unwrap().t {
                    closest_intersection = Some(intersection);
                }
            }
        }

        closest_intersection
    }
}