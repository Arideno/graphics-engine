use graphics_engine::{camera::Camera, point::Point, scene::Scene, sphere::Sphere, intersection::Intersection, plane::Plane, vector::Vector, light::{Light, Directional}};

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;

fn main() {
    let mut scene = Scene::new(Camera::new(Point::new(0., 0., 0.), 90., WIDTH as f64 / HEIGHT as f64, HEIGHT), vec![], vec![]);

    scene.objects.push(Sphere::new(Point::new(0., 0., -1.), 0.5).into());
    scene.objects.push(Plane::new(Vector::new(0., 0., -1.), Point::new(0., 0., -1.)).into());
    scene.lights.push(Directional { direction: (1., 1., 1.).into() }.into());
    
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
                        match scene.lights[0] {
                            Light::Directional(light) => {
                                let normal = object.normal_at_point(ray.at(t));
                                let product = light.direction.dot(normal);
                                if product < 0. {
                                    symbol = ' ';
                                } else if product < 0.2 {
                                    symbol = '.';
                                } else if product < 0.5 {
                                    symbol = '*';
                                } else if product < 0.8 {
                                    symbol = 'O';
                                } else {
                                    symbol = '#';
                                }
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
