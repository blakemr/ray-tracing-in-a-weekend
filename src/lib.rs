pub mod camera;
pub mod hittable;
pub mod ray;
pub mod shapes;

use camera::Camera;
use hittable::HitList;
use image::{Pixel, Rgb, RgbImage};
use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use ray::Ray;
use shapes::Sphere;

#[test]
fn test() -> std::io::Result<()> {
    make_image()
}

pub fn vec_color(color: &Vector3<f32>) -> Rgb<u8> {
    fn scale_color(col: f32) -> u8 {
        (col.clamp(0.0, 1.0) * u8::MAX as f32) as u8
    }

    Rgb([
        scale_color(color.x),
        scale_color(color.y),
        scale_color(color.z),
    ])
}

pub fn ray_color(ray: &Ray, world: &HitList) -> Rgb<u8> {
    let color: Vector3<f32>;

    match world.hit(ray, 0.0, f32::INFINITY) {
        Some(h) => color = 0.5 * h.normal,
        None => {
            let unit_dir: Vector3<f32> = ray.direction.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            color = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        }
    }

    vec_color(&color)
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

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        for _ in 0..SAMPLES_PP {
            let u = (x as f32 + rng.gen_range(0.01..1.0)) / IMAGE_WIDTH as f32;
            let v = (y as f32 + rng.gen_range(0.01..1.0)) / IMAGE_HEIGHT as f32;

            let ray = camera.cast_ray(u, v);

            pixel.blend(&ray_color(&ray, &world));
        }

        if x == 0 {
            println!("{:.2} %", y as f32 / IMAGE_HEIGHT as f32 * 100.0)
        }
    }

    img.save("docs/testimage.png")
        .expect("Failed to save as png.");

    Ok(())
}
