use graphics_engine::{camera::Camera, point::Point, scene::Scene, vector::Vector, light::{Directional}, renderer::{Renderer, Png}, mesh::Mesh, matrix::Matrix};
use clap::Parser;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value = "k.obj")]
    source: String,

    #[clap(long, default_value = "test.png")]
    output: String,
}

fn main() {
    let args = Args::parse();
   
    let mut scene = Scene::new(Camera::new(Point::new(0., 0., 1.), 90., WIDTH as f32 / HEIGHT as f32, HEIGHT), vec![], vec![]);

    // scene.add_intersectable(Sphere::new(Point::new(0., 0., -1.5), 0.7).into());
    // scene.add_intersectable(Plane::new(Vector::new(0., 0., 1.), Point::new(0., 0., -1.)).into());
    // scene.add_intersectable(Triangle::new(Point::new(-0.5, 0., -0.5), Point::new(0., 1., -1.), Point::new(0.5, 0., -1.5)).into());
    let mesh = Mesh::from_model(args.source.as_str()).unwrap();
    scene.add_mesh(mesh.apply_transform(&Matrix::scale(0.5, 0.5, 0.5)).apply_transform(&Matrix::rotate_y(3.14 / 6.)).apply_transform(&Matrix::rotate_x(-3.14 / 6.)));
    scene.add_light(Directional { direction: Vector::new(-1., -1., -1.).normalize() }.into());
    scene.add_light(Directional { direction: Vector::new(1., 1., -1.).normalize() }.into());

    let png_renderer: Renderer = Png::new(&scene, args.output, WIDTH, HEIGHT).into();
    png_renderer.render();
}
