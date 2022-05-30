use crate::hittable::HitList;
use crate::ray::Ray;
use crate::shapes::Sphere;
use crate::utilities;

use nalgebra::Vector3;
use rand::prelude::ThreadRng;

pub struct Scene {
    hittables: HitList,
}

impl Scene {
    pub fn new() -> Scene {
        let hittables = HitList::new();
        Scene { hittables }
    }

    pub fn add_sphere(&mut self, x: f64, y: f64, z: f64, r: f64) {
        self.hittables
            .add(Box::new(Sphere::new(Vector3::<f64>::new(x, y, z), r)));
    }

    pub fn ray_color(&self, ray: &Ray, rng: &mut ThreadRng, bounces: u64) -> Vector3<f64> {
        let mut r: Ray = ray.clone();

        for i in 0..bounces {
            let color_scale = (0.5 as f64).powf(i as f64);

            match self.hittables.hit(&r, 0.0, f64::INFINITY) {
                Some(hit) => {
                    let target = hit.point + hit.normal + utilities::rand_point_in_unit_sphere(rng);

                    let origin = hit.point;
                    let direction = target - hit.point;
                    r = Ray { origin, direction };
                }
                None => {
                    let color_final = r.direction;
                    let t = 0.5 * (color_final.normalize().y + 1.0);

                    let white = Vector3::<f64>::new(1.0, 1.0, 1.0);
                    let blue = Vector3::<f64>::new(0.5, 0.7, 1.0);

                    return color_scale * white.lerp(&blue, t);
                }
            }
        }

        return Vector3::<f64>::zeros();
    }
}
