use std::{sync::{Arc, atomic::{AtomicU64, Ordering}, Mutex}, thread, time::Duration};

use image::{Rgb, RgbImage};
use pbr::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{scene::Scene, intersection::Intersection, light::Light, color::Color};

pub enum Renderer<'a> {
    Console(Console<'a>),
    Png(Png<'a>)
}

impl<'a> Renderer<'a> {
    pub fn render(&self) {
        match self {
            Renderer::Console(console) => console.render(),
            Renderer::Png(png) => png.render()
        }
    }
}

pub struct Console<'a> {
    scene: &'a Scene,
    width: u32,
    height: u32
}

impl<'a> Console<'a> {
    pub fn new(scene: &'a Scene, width: u32, height: u32) -> Console {
        Console { scene, width, height }
    }

    pub fn render(&self) {
        let mut buffer = vec![' '; (self.width * self.height) as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let ray = self.scene.ray_for_pixel(x, self.height - y - 1);

                let mut symbol = ' ';

                if let Some(intersection) = self.scene.intersect(ray) {
                    let Intersection { point, object, .. } = intersection;
                    match self.scene.lights[0] {
                        Light::Directional(light) => {
                            let normal = object.normal_at_point(point);
                            let product = -light.direction.dot(normal);
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

                buffer[(y * self.width + x) as usize] = symbol;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", buffer[(y * self.width + x) as usize]);
            }
            println!();
        }
    }
}

pub struct Png<'a> {
    scene: &'a Scene,
    filename: String,
    width: u32,
    height: u32
}

impl<'a> Png<'a> {
    pub fn new(scene: &'a Scene, filename: String, width: u32, height: u32) -> Png {
        Png { scene, filename, width, height }
    }

    pub fn render(&self) {
        let thread_progress = Arc::new(AtomicU64::new(0));
        let thread_pb = Arc::new(Mutex::new(ProgressBar::new((self.width * self.height) as u64)));
        let pb = Some(thread_pb.clone());
        let progress = Some(thread_progress.clone());
        let width = self.width;
        let height = self.height;
        thread::spawn(move || loop {
            let value = thread_progress.load(Ordering::Relaxed);
            if let Ok(mut pb) = thread_pb.lock() {
                pb.set(value);
            }

            if value >= (width * height) as u64 {
                break;
            }

            thread::sleep(Duration::from_millis(250));
        });

        let pixels: Vec<Rgb<u8>> = (0..(self.width * self.height))
            .into_par_iter()
            .map(|i| (i / self.height, i % self.height))
            .map(|(x, y)| {
                let ray = self.scene.ray_for_pixel(x, self.height - y - 1);

                let mut color = Color::new(255, 255, 255);

                if let Some(intersection) = self.scene.intersect(ray) {
                    let Intersection { point, object, .. } = intersection;
                    match self.scene.lights[0] {
                        Light::Directional(light) => {
                            let normal = object.normal_at_point(point);
                            let product = -light.direction.dot(normal);
                            color *= product;
                        }
                    }
                }

                if let Some(progress) = progress.as_ref() {
                    progress.fetch_add(1, Ordering::Relaxed);
                }

               color.into()
            })
            .collect();
        
        if let Some(pb) = pb {
            pb.lock().unwrap().finish();
        }
        
        let img = pixels
            .into_iter()
            .enumerate()
            .map(|(i, pixel)| {
                (
                    (i as u32) / self.height,
                    (i as u32) % self.height,
                    pixel
                )
            })
            .fold(RgbImage::new(self.width, self.height), 
                |mut image, (x, y, pixel)| {
                    image.put_pixel(x, y, pixel);
                    image
                }
            );

        img.save(self.filename.as_str()).unwrap();
    }
}

impl<'a> From<Console<'a>> for Renderer<'a> {
    fn from(console: Console<'a>) -> Self {
        Renderer::Console(console)
    }
}

impl<'a> From<Png<'a>> for Renderer<'a> {
    fn from(png: Png<'a>) -> Self {
        Renderer::Png(png)
    }
}
