use nalgebra::Vector3;
use rand::{thread_rng, Rng};

pub type Vec3 = Vector3<f64>;
pub type Color = Vector3<f64>;

pub fn rand_vec3() -> Vec3 {
    let mut rng = thread_rng();
    Vec3::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

pub fn rand_point_range(min: f64, max: f64) -> Vec3 {
    let mut rng = thread_rng();

    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn rand_point_in_unit_sphere() -> Vec3 {
    loop {
        let point = rand_point_range(-1.0, 1.0);
        if point.magnitude_squared() < 1.0 {
            return point;
        }
    }
}

pub fn rand_pouint_in_hemisphere(normal: Vec3) -> Vec3 {
    let point = rand_point_in_unit_sphere();

    if point.dot(&normal) > 0.0 {
        point
    } else {
        -point
    }
}

pub fn vec_to_color_array(color: &Color) -> [u8; 3] {
    fn scale_color(col: f64) -> u8 {
        (col.clamp(0.0, 1.0) * u8::MAX as f64) as u8
    }

    [
        scale_color(color.x.sqrt()),
        scale_color(color.y.sqrt()),
        scale_color(color.z.sqrt()),
    ]
}

pub fn reflection(in_dir: Vec3, normal: Vec3) -> Vec3 {
    (in_dir - 2.0 * in_dir.dot(&normal) * normal).normalize()
}

pub fn refraction(in_dir: Vec3, normal: Vec3, refraction_ratio: f64) -> Vec3 {
    let cos_theta = (-in_dir).dot(&normal).min(1.0);
    let r_out_perp = refraction_ratio * (in_dir + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * normal;

    r_out_perp + r_out_parallel
}
