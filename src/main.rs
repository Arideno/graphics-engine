use graphics_engine::{camera::Camera, point::Point, scene::Scene, sphere::Sphere};

fn main() {
    let mut scene = Scene::new(Camera::new(Point::new(0., 0., 0.), 90., 1., 100), vec![]);

    scene.objects.push(Sphere::new(Point::new(0., 0., -1.), 0.5).into());
    
    let mut buffer = ['#'; 100*100];

    for y in 0..100 {
        for x in 0..100 {
            let ray = scene.ray_for_pixel(x, y);

            for object in &scene.objects {
                if let Some(_) = object.intersect(ray) {
                    buffer[(y * 100 + x) as usize] = '*';
                }
            }
        }
    }

    for y in 0..100 {
        for x in 0..100 {
            print!("{}", buffer[(y * 100 + x) as usize]);
        }
        println!();
    }
}
