use graphics_engine::{camera::Camera, point::Point, scene::Scene, sphere::Sphere, plane::Plane, vector::Vector, light::{Directional}, renderer::{Renderer, Png}, triangle::Triangle};

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

fn main() {
    let mut scene = Scene::new(Camera::new(Point::new(0., 0., 1.), 90., WIDTH as f64 / HEIGHT as f64, HEIGHT), vec![], vec![]);

    scene.objects.push(Sphere::new(Point::new(0., 0., -1.5), 0.7).into());
    scene.objects.push(Plane::new(Vector::new(0., 0., 1.), Point::new(0., 0., -1.)).into());
    scene.objects.push(Triangle::new(Point::new(-0.5, 0., -0.5), Point::new(0., 1., -1.), Point::new(0.5, 0., -1.5)).into());
    scene.lights.push(Directional { direction: Vector::new(-1., -1., -1.).normalize() }.into());

    let png_renderer: Renderer = Png::new(&scene, "test.png".to_owned(), WIDTH, HEIGHT).into();
    png_renderer.render();
}
