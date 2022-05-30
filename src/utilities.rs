use nalgebra::Vector3;
use rand::{prelude::ThreadRng, thread_rng, Rng};

pub fn rand_point(rng: &mut ThreadRng) -> Vector3<f64> {
    Vector3::<f64>::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

pub fn rand_point_range(min: f64, max: f64) -> Vector3<f64> {
    let mut rng = thread_rng();

    Vector3::<f64>::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn rand_point_in_unit_sphere() -> Vector3<f64> {
    loop {
        let point = rand_point_range(-1.0, 1.0);
        if point.magnitude_squared() < 1.0 {
            return point;
        }
    }
}

pub fn rand_pouint_in_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let point = rand_point_in_unit_sphere();

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

pub fn refraction(
    in_dir: Vector3<f64>,
    normal: Vector3<f64>,
    refraction_ratio: f64,
) -> Vector3<f64> {
    let cos_theta = (-1.0 * in_dir).dot(&normal).min(1.0);
    let r_out_perp = refraction_ratio * (in_dir + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude().powi(2)).abs().sqrt() * normal;

    r_out_perp + r_out_parallel
}
