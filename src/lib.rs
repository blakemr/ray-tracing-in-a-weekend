pub mod camera;
pub mod hittable;
pub mod ray;
pub mod shapes;

use camera::Camera;
use hittable::HitList;
use image::{Rgb, RgbImage};
use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use ray::Ray;
use shapes::Sphere;

#[test]
fn test() -> std::io::Result<()> {
    make_image()
}

pub fn vec_color(color: &Vector3<f32>, samples: u32) -> [f32; 3] {
    fn scale_color(col: f32) -> f32 {
        col.clamp(0.0, 1.0) * u8::MAX as f32
    }

    let sam = 1.0 / samples as f32;

    [
        scale_color(color.x * sam),
        scale_color(color.y * sam),
        scale_color(color.z * sam),
    ]
}

pub fn ray_color(ray: &Ray, world: &HitList, samples: u32) -> [f32; 3] {
    let color: Vector3<f32>;

    match world.hit(ray, 0.0, f32::INFINITY) {
        Some(h) => color = 0.5 * h.normal,
        None => {
            let unit_dir: Vector3<f32> = ray.direction.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        }
    }

    vec_color(&color, samples)
}

pub fn make_image() -> std::io::Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PP: u32 = 100;

    // Sampling
    let mut rng = thread_rng();

    // World
    let mut world = HitList::new();

    world.add(Box::new(Sphere::new(
        Vector3::<f32>::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(Sphere::new(
        Vector3::<f32>::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let camera = Camera::default();

    // Render
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // I really hate this method of keeping it out of the pixel struct until the processing is done.
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pix: [f32; 3] = [0.0, 0.0, 0.0];
        for _ in 0..SAMPLES_PP {
            let u = (x as f32 + rng.gen_range(0.0..1.0)) / IMAGE_WIDTH as f32;
            let v = (y as f32 + rng.gen_range(0.0..1.0)) / IMAGE_HEIGHT as f32;

            let ray = camera.cast_ray(u, v);

            let sam = ray_color(&ray, &world, SAMPLES_PP);

            pix[0] += sam[0];
            pix[1] += sam[1];
            pix[2] += sam[2];
        }

        *pixel = Rgb([pix[0] as u8, pix[1] as u8, pix[2] as u8]);

        if x == 0 {
            println!("{:.2} %", y as f32 / IMAGE_HEIGHT as f32 * 100.0)
        }
    }

    img.save("docs/testimage.png")
        .expect("Failed to save as png.");

    Ok(())
}
