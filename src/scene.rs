use crate::hittable::HitList;
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

    pub fn add_sphere(&mut self, x: f32, y: f32, z: f32, r: f32) {
        self.hittables
            .add(Box::new(Sphere::new(Vector3::<f32>::new(x, y, z), r)));
    }

    pub fn ray_color(&self, ray: &Ray) -> Vector3<f32> {
        match self.hittables.hit(ray, 0.0, f32::INFINITY) {
            Some(h) => {
                return h.normal;
            }
            None => {
                let t = 0.5 * (ray.direction.normalize().y + 1.0);

                let white = Vector3::<f32>::new(1.0, 1.0, 1.0);
                let blue = Vector3::<f32>::new(0.5, 0.7, 1.0);

                return white.lerp(&blue, t);
            }
        }
    }
}
