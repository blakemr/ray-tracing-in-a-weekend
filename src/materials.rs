use nalgebra::Vector3;

use crate::{hittable::HitRecord, ray::Ray};

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Vector3<f64>, Ray)>;
}
