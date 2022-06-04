use nalgebra::Vector3;
use rand::{thread_rng, Rng};

type Vec3 = Vector3<f64>;

trait Reflection {
    fn reflect(&self, normal: &Vec3) -> Vec3;
}

trait Refraction {
    fn refract(&self, normal: &Vec3, refraction_ratio: f64) -> Vec3;
}

impl Reflection for Vec3 {
    fn reflect(&self, normal: &Vec3) -> Vec3 {
        (self - 2.0 * self.dot(&normal) * normal).normalize()
    }
}

impl Refraction for Vec3 {
    fn refract(&self, normal: &Vec3, refraction_ratio: f64) -> Vec3 {
        let cos_theta = (-self).dot(&normal).min(1.0);
        let r_out_perp = refraction_ratio * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * normal;

        r_out_perp + r_out_parallel
    }
}

pub fn rand_vec3() -> Vec3 {
    let mut rng = thread_rng();
    Vec3::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
    )
}

pub fn rand_vec3_from_range(min: f64, max: f64) -> Vec3 {
    let mut rng = thread_rng();

    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}

pub fn rand_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let point = rand_vec3_from_range(-1.0, 1.0);
        if point.magnitude_squared() < 1.0 {
            return point;
        }
    }
}

pub fn rand_vec3_in_hemisphere(normal: Vec3) -> Vec3 {
    let point = rand_vec3_in_unit_sphere();

    if point.dot(&normal) > 0.0 {
        point
    } else {
        -point
    }
}
