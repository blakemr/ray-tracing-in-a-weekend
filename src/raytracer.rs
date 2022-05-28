use image::{Rgb, RgbImage};
use nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + (t * self.direction)
    }
}

pub struct Camera {
    width: f32,
    height: f32,
    focal_length: f32,

    position: Vector3<f32>,
}

impl Camera {
    pub fn default() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        Camera {
            width: 2.0 * aspect_ratio,
            height: 2.0,
            focal_length: 1.0,
            position: Vector3::<f32>::zeros(),
        }
    }

    pub fn top_left(&self) -> Vector3<f32> {
        let horizontal = Vector3::<f32>::new(self.width, 0.0, 0.0);
        let vertical = Vector3::<f32>::new(0.0, self.height, 0.0);

        self.position
            - horizontal / 2.0
            - vertical / 2.0
            - Vector3::<f32>::new(0.0, 0.0, self.focal_length)
    }
}

pub fn ray_color(ray: &Ray) -> Rgb<u8> {
    let unit_dir: Vector3<f32> = ray.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    let color: Vector3<f32> =
        ((1.0 - t) * Vector3::new(1.0, 1.0, 1.0)) + (t * Vector3::new(0.5, 0.7, 1.0));

    Rgb([color.x as u8, color.y as u8, color.z as u8])
}

pub fn make_image() -> std::io::Result<()> {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // Camera
    let camera = Camera::default();

    let top_left = camera.top_left();
    let vertical = Vector3::<f32>::new(camera.width, 0.0, 0.0);
    let horizontal = Vector3::<f32>::new(0.0, camera.height, 0.0);

    // Render
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x / IMAGE_WIDTH;
        let v = y / IMAGE_HEIGHT;

        let origin: Vector3<f32> = camera.position;
        let mut direction: Vector3<f32> =
            top_left + (u as f32 * horizontal) + (v as f32 * vertical) - camera.position;
        direction *= 256.0 as f32;

        let ray = Ray { origin, direction };

        *pixel = ray_color(&ray)
    }

    img.save("docs/testimage.png")
        .expect("Failed to save as png.");

    Ok(())
}
