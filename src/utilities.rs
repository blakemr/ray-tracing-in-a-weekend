use nalgebra::Vector3;
use rand::{prelude::ThreadRng, Rng};

pub fn rand_point(rng: &mut ThreadRng) -> Vector3<f64> {
    Vector3::<f64>::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

pub fn rand_point_range(rng: &mut ThreadRng, min: f64, max: f64) -> Vector3<f64> {
    Vector3::<f64>::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn rand_point_in_unit_sphere(rng: &mut ThreadRng) -> Vector3<f64> {
    loop {
        let point = rand_point_range(rng, -1.0, 1.0);
        if point.magnitude_squared() < 1.0 {
            return point;
        }
    }
}

pub fn rand_pouint_in_hemisphere(rng: &mut ThreadRng, normal: Vector3<f64>) -> Vector3<f64> {
    let point = rand_point_in_unit_sphere(rng);

    if point.dot(&normal) > 0.0 {
        point
    } else {
        -point
    }
}

pub fn vec_to_color_array(color: &Vector3<f64>) -> [u8; 3] {
    fn scale_color(col: f64) -> u8 {
        (col.clamp(0.0, 1.0) * u8::MAX as f64) as u8
    }

    [
        scale_color(color.x.sqrt()),
        scale_color(color.y.sqrt()),
        scale_color(color.z.sqrt()),
    ]
}
