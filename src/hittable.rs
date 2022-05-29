use crate::Ray;
use nalgebra::Vector3;

pub struct HitRecord {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vector3::<f32>::zeros(),
            normal: Vector3::<f32>::zeros(),
            t: 0.0,
            front: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        Some(HitRecord::new())
    }

    fn normal(&self, p: &Vector3<f32>) -> Vector3<f32> {
        *p
    }
}
