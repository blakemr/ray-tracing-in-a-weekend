use nalgebra::Vector3;
use rand::{prelude::ThreadRng, Rng};

pub fn rand_point(mut rng: ThreadRng) -> Vector3<f32> {
    Vector3::<f32>::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

pub fn rand_point_range(mut rng: ThreadRng, min: f32, max: f32) -> Vector3<f32> {
    Vector3::<f32>::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn vec_to_color_array(color: &Vector3<f32>) -> [u8; 3] {
    fn scale_color(col: f32) -> u8 {
        (col.clamp(0.0, 1.0) * u8::MAX as f32) as u8
    }

    [
        scale_color(color.x),
        scale_color(color.y),
        scale_color(color.z),
    ]
}
