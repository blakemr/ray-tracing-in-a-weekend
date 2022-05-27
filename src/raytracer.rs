use image::{Rgb, RgbImage};
use nalgebra::{Point3, Vector3};

struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3<f32> {
        self.origin + (t * self.direction)
    }
}

pub fn make_image() -> std::io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 0..(IMAGE_WIDTH * IMAGE_HEIGHT) {
        let x = i % IMAGE_WIDTH;
        let y = i / IMAGE_WIDTH;

        let r: f32 = x as f32 / IMAGE_WIDTH as f32;
        let g: f32 = y as f32 / IMAGE_HEIGHT as f32;
        let b: f32 = 0.25;

        let ir: u8 = (r * 256.0) as u8;
        let ig: u8 = (g * 256.0) as u8;
        let ib: u8 = (b * 256.0) as u8;

        img.put_pixel(x, y, Rgb([ir, ig, ib]));
    }

    img.save("docs/testimage.png")
        .expect("Failed to save as png.");

    Ok(())
}
