pub mod camera;
pub mod hittable;
pub mod ray;
pub mod shapes;

use camera::Camera;
use image::{Rgb, RgbImage};
use nalgebra::Vector3;
use ray::Ray;
use shapes::{sphere_hit, Sphere};

#[test]
fn test() -> std::io::Result<()> {
    make_image()
}

pub fn vec_color(vec: Vector3<f32>) -> Rgb<u8> {
    Rgb([
        (vec.x * 256.0) as u8,
        (vec.y * 256.0) as u8,
        (vec.z * 256.0) as u8,
    ])
}

pub fn ray_color(ray: &Ray) -> Rgb<u8> {
    let sphere = Sphere {
        position: Vector3::<f32>::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let t = sphere_hit(&sphere, &ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vector3::<f32>::new(0.0, 0.0, -1.0)).normalize();

        return vec_color(n);
    }

    let unit_dir: Vector3<f32> = ray.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    let color: Vector3<f32> =
        ((1.0 - t) * Vector3::new(1.0, 1.0, 1.0)) + (t * Vector3::new(0.5, 0.7, 1.0));

    vec_color(color)
}

pub fn make_image() -> std::io::Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // Camera
    let camera = Camera::default();

    let top_left = camera.top_left();
    let vertical = Vector3::<f32>::new(0.0, camera.height, 0.0);
    let horizontal = Vector3::<f32>::new(camera.width, 0.0, 0.0);

    // Render
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u: f32 = x as f32 / IMAGE_WIDTH as f32;
        let v: f32 = y as f32 / IMAGE_HEIGHT as f32;

        let origin: Vector3<f32> = camera.position;
        let direction: Vector3<f32> =
            top_left + (u as f32 * horizontal) - (v as f32 * vertical) - camera.position;

        let ray = Ray { origin, direction };

        *pixel = ray_color(&ray)
    }

    img.save("docs/testimage.png")
        .expect("Failed to save as png.");

    Ok(())
}
