use crate::ray::Ray;
use nalgebra::Vector3;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vector3::<f32>::zeros(),
            normal: Vector3::<f32>::zeros(),
            t: 0.0,
            front: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn normal(&self, p: &Vector3<f32>) -> Vector3<f32>;
    fn clone_dyn(&self) -> Box<dyn Hittable>;
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

#[derive(Clone)]
pub struct HitList {
    objects: Vec<Box<dyn Hittable>>, // Probably not optimal, only use for education.
}

impl HitList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut first_hit: Option<HitRecord> = None;

        self.objects
            .clone()
            .into_iter()
            .for_each(|object| match object.hit(ray, t_min, t_max) {
                Some(h) => match first_hit.clone() {
                    Some(hit) => {
                        if hit.t > h.t {
                            first_hit = Some(h);
                        }
                    }
                    None => {
                        first_hit = Some(h);
                    }
                },
                None => (),
            });

        first_hit
    }
}
