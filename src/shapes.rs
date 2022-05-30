use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: Vector3<f32>, radius: f32) -> Self {
        Self { position, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.position;
        let a = ray.direction.magnitude_squared();
        let b = oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let dsqrt = discriminant.sqrt();

        // Find the first root in the given range
        let root = (-b - dsqrt) / a;
        if t_min > root || root > t_max {
            let root = (-b + dsqrt) / a;
            if t_min > root || root > t_max {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = self.normal(&point);
        let t = root;
        let front = ray.direction.dot(&normal) > 0.0;

        Some(HitRecord {
            point,
            normal,
            t,
            front,
        })
    }

    fn normal(&self, p: &Vector3<f32>) -> Vector3<f32> {
        (p - self.position) / self.radius
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
