use crate::{intersectable::Intersectable, camera::Camera, ray::Ray, light::Light, intersection::Intersection, mesh::Mesh};

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

    pub fn add_intersectable(&mut self, intersectable: Intersectable) {
        self.objects.push(intersectable);
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        for triangle in mesh.triangles {
            self.objects.push(triangle.into());
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
}


#[cfg(test)]
mod tests {
    use crate::{intersectable::Intersectable, point::Point, sphere::Sphere, vector::Vector, plane::Plane};

    use super::*;

    #[test]
    fn test_intersect() {
        let center1 = Point::new(0., 5., 0.);
        let radius1 = 2.;
        let sphere1 = Sphere::new(center1, radius1);
        let center2 = Point::new(0., -5., 0.);
        let radius2 = 1.;
        let sphere2 = Sphere::new(center2, radius2);
        let point = Point::new(0., 0., 0.);
        let normal = Vector::new(0., 1., 0.);
        let plane = Plane::new(normal, point);
        let objects: Vec<Intersectable> = vec![sphere1.into(), sphere2.into(), plane.into()];
        let camera = Camera::new(Point::new(0., 0., 0.), 0., 0., 0);
        let scene = Scene::new(camera, objects, vec![]);
        let origin = Point::new(0., 20., 0.);
        let direction = Vector::new(0., -1., 0.);
        let ray = Ray::new(origin, direction);
        let intersection = scene.intersect(ray);
        if let Some(intersection) = intersection {
            assert_eq!(Point::new(0., 7., 0.), intersection.point);
            assert_eq!(13., intersection.t);
            assert_eq!(Intersectable::from(sphere1), intersection.object);
        } else {
            panic!("No intersection");
        }
    }
}