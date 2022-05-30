use nalgebra::Vector3;

use crate::{hittable::HitRecord, ray::Ray, utilities};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vector3<f64>, Ray)>;
}

// Lambertian
pub struct Diffuse {
    albedo: Vector3<f64>,
}

impl Diffuse {
    pub fn new(albedo: Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let mut scatter_direction = hit.normal + utilities::rand_point_in_unit_sphere();
        if scatter_direction.magnitude() < 1.0e-8 {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vector3<f64>,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let ray_reflect =
            (ray.direction - 2.0 * ray.direction.dot(&hit.normal) * hit.normal).normalize();
        let scattered = Ray::new(hit.point, ray_reflect);

        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
