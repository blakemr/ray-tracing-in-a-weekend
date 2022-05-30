use crate::camera::Camera;
use crate::scene::Scene;
use crate::utilities;

use image::{Rgb, RgbImage};
use nalgebra::Vector3;
use rand::{prelude::ThreadRng, thread_rng, Rng};

pub struct Render {
    image: RgbImage,

    width: u32,
    height: u32,
    samples_per_pixel: u32,

    rng: ThreadRng,
}

impl Render {
    pub fn new(width: u32, height: u32, samples: u32) -> Self {
        Render {
            image: RgbImage::new(width, height),
            width: width,
            height: height,
            samples_per_pixel: samples,
            rng: thread_rng(),
        }
    }

    pub fn save(&self, path: &str) {
        self.image.save(path).expect("Failed to save image.");
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        for (x, y, pixel) in self.image.enumerate_pixels_mut() {
            let mut pixel_sum = Vector3::<f32>::zeros();

            for _ in 0..self.samples_per_pixel {
                let u = (x as f32 + self.rng.gen_range(0.0..1.0)) / self.width as f32;
                let v = (y as f32 + self.rng.gen_range(0.0..1.0)) / self.height as f32;

                let ray = camera.cast_ray(u, v);

                pixel_sum += scene.ray_color(&ray);
            }

            *pixel = Rgb(utilities::vec_to_color_array(
                &(pixel_sum / self.samples_per_pixel as f32),
            ));

            if x == 0 {
                println!("{:.2} %", y as f32 / self.height as f32 * 100.0);
            }
        }
    }
}
