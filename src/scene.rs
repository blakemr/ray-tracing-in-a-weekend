use std::rc::Rc;

use crate::hittable::HitList;
use crate::materials::Scatter;
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

    pub fn add_sphere(&mut self, x: f64, y: f64, z: f64, r: f64, mat: Rc<dyn Scatter>) {
        self.hittables
            .add(Box::new(Sphere::new(Vector3::<f64>::new(x, y, z), r, mat)));
    }

    pub fn ray_color(&self, ray: &Ray, rng: &mut ThreadRng, bounces: u64) -> Vector3<f64> {
        if bounces <= 0 {
            return Vector3::<f64>::zeros();
        }

        if let Some(hit) = self.hittables.hit(ray, 0.001, f64::INFINITY) {
            let target = hit.point + utilities::rand_pouint_in_hemisphere(rng, hit.normal);
            let r = Ray::new(hit.point, target - hit.point);

            0.5 * self.ray_color(&r, rng, bounces - 1)
        } else {
            let unit_dir = ray.direction.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);

            let white = Vector3::<f64>::new(1.0, 1.0, 1.0);
            let blue = Vector3::<f64>::new(0.5, 0.7, 1.0);

            return white.lerp(&blue, t);
        }
    }
}
