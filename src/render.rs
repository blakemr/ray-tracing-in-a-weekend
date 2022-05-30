use crate::camera::Camera;
use crate::scene::Scene;
use crate::utilities;

use image::{Rgb, RgbImage};
use nalgebra::Vector3;
use rand::{prelude::ThreadRng, thread_rng, Rng};

pub struct Render {
    image: RgbImage,

    width: u64,
    height: u64,
    samples_per_pixel: u64,

    rng: ThreadRng,

    max_ray_depth: u64,
}

impl Render {
    pub fn new(width: u64, height: u64, samples: u64, depth: u64) -> Self {
        Render {
            image: RgbImage::new(width as u32, height as u32),
            width: width,
            height: height,
            samples_per_pixel: samples,
            rng: thread_rng(),
            max_ray_depth: depth,
        }
    }

    pub fn save(&self, path: &str) {
        self.image.save(path).expect("Failed to save image.");
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        for (x, y, pixel) in self.image.enumerate_pixels_mut() {
            let mut pixel_sum = Vector3::<f64>::zeros();

            for _ in 0..self.samples_per_pixel {
                let u = (x as f64 + self.rng.gen_range(0.0..1.0)) / self.width as f64;
                let v = (y as f64 + self.rng.gen_range(0.0..1.0)) / self.height as f64;

                let ray = camera.cast_ray(u, v);

                pixel_sum += scene.ray_color(&ray, self.max_ray_depth);
            }

            *pixel = Rgb(utilities::vec_to_color_array(
                &(pixel_sum / self.samples_per_pixel as f64),
            ));

            // Debug output
            if x == 0 {
                println!("{:.2} %", y as f64 / self.height as f64 * 100.0);
            }
        }
    }
}
