use std::rc::Rc;

use crate::hittable::HitList;
use crate::materials::Material;
use crate::ray::Ray;
use crate::shapes::Sphere;

use nalgebra::Vector3;

pub struct Scene {
    hittables: HitList,
}

impl Scene {
    pub fn new() -> Scene {
        let hittables = HitList::new();
        Scene { hittables }
    }

    pub fn add_sphere(&mut self, x: f64, y: f64, z: f64, r: f64, mat: Rc<dyn Material>) {
        self.hittables
            .add(Box::new(Sphere::new(Vector3::<f64>::new(x, y, z), r, mat)));
    }

    pub fn ray_color(&self, ray: &Ray, bounces: u64) -> Vector3<f64> {
        if bounces <= 0 {
            return Vector3::<f64>::zeros();
        }

        if let Some(hit) = self.hittables.hit(ray, 0.001, f64::INFINITY) {
            if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
                self.ray_color(&scattered, bounces - 1)
                    .component_mul(&attenuation)
            } else {
                Vector3::<f64>::zeros()
            }
        } else {
            let unit_dir = ray.direction.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);

            let white = Vector3::<f64>::new(1.0, 1.0, 1.0);
            let blue = Vector3::<f64>::new(0.5, 0.7, 1.0);

            return white.lerp(&blue, t);
        }
    }
}
