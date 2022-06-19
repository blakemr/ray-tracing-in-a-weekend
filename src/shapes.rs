use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vector::Vec3;

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
        let half_b = oc.dot(&ray.direction);
        let c = oc.magnitude_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let dsqrt = discriminant.sqrt();

        // Find the first root in the given range
        let mut root = (-half_b - dsqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + dsqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);

        let mut rec = HitRecord {
            point: p,
            t: root,
            normal: Vec3::zeros(),
            material: self.material.clone(),
            front: false,
        };

        let normal_out = (rec.point - self.position) / self.radius;
        rec.set_normal(ray, normal_out);

        Some(rec)
    }

    fn normal(&self, p: &Vec3) -> Vec3 {
        (p - self.position) / self.radius
    }

    fn clone_dyn(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
