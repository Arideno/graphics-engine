use graphics_engine::{camera::Camera, point::Point, scene::Scene, sphere::Sphere, intersection::Intersection, intersectable::Intersectable, plane::Plane, vector::Vector};

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;

fn main() {
    let mut scene = Scene::new(Camera::new(Point::new(0., 0., 0.), 90., WIDTH as f64 / HEIGHT as f64, HEIGHT), vec![]);

    scene.objects.push(Sphere::new(Point::new(0., 0., -1.), 0.5).into());
    scene.objects.push(Plane::new(Vector::new(0., 0., -1.), Point::new(0., 0., -1.)).into());
    
    let mut buffer = [' '; (WIDTH * HEIGHT) as usize];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = scene.ray_for_pixel(x, y);

            let mut min_t = std::f64::MAX;
            let mut symbol = ' ';

            for object in &scene.objects {
                if let Some(intersection) = object.intersect(ray) {
                    let Intersection { t, object } = intersection;
                    if t < min_t {
                        min_t = t;
                        match object {
                            Intersectable::Sphere { .. } => {
                                symbol = 'o';
                            },
                            Intersectable::Plane { .. } => {
                                symbol = '#';
                            }
                        }
                    }
                }
            }

            buffer[(y * WIDTH + x) as usize] = symbol;
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", buffer[(y * WIDTH + x) as usize]);
        }
        println!();
    }
}
