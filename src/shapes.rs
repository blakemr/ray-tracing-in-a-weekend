use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::utilities::Vec3;

#[derive(Clone)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            position,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

        let normal_out = self.normal(&ray.at(root));
        Some(HitRecord::new(ray, root, normal_out, self.material.clone()))
    }

    fn normal(&self, p: &Vec3) -> Vec3 {
        (p - self.position) / self.radius
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
