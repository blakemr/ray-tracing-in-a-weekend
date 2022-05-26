use image::{Rgb, RgbImage};
use std::fs::write;

#[test]
fn test() -> std::io::Result<()> {
    make_image()
}

fn make_image() -> std::io::Result<()> {
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
