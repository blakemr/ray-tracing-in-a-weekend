use crate::utilities::Color;
use crate::{hittable::HitRecord, ray::Ray, utilities};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

// Lambertian
pub struct Diffuse {
    albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + utilities::rand_point_in_unit_sphere();
        if scatter_direction.magnitude() < 1.0e-8 {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let ray_reflect = utilities::reflection(ray.direction, hit.normal);
        let scattered = Ray::new(
            hit.point,
            ray_reflect + self.fuzz * utilities::rand_point_in_unit_sphere(),
        );

        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit.front {
            1.0 / self.refraction_index
        } else {
            self.refraction_index / 1.0
        };

        let unit_dir = ray.direction.normalize();

        let cos_theta = (-unit_dir).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0 {
            utilities::reflection(unit_dir, hit.normal)
        } else {
            utilities::refraction(unit_dir, hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.point, direction);

        Some((Color::new(0.9, 0.9, 1.0), scattered))
    }
}
