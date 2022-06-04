use std::rc::Rc;

use crate::materials::Material;
use crate::ray::Ray;
use crate::utilities::Vec3;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, normal_out: Vec3, mat: Rc<dyn Material>) -> HitRecord {
        let mut record = HitRecord {
            point: ray.at(t),
            normal: Vec3::zeros(),
            material: mat,
            t: t,
            front: false,
        };

        record.set_normal(ray, normal_out);

        record
    }

    pub fn set_normal(&mut self, ray: &Ray, normal_out: Vec3) {
        self.front = ray.direction.dot(&normal_out) < 0.0;
        self.normal = if self.front { normal_out } else { -normal_out }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn normal(&self, p: &Vec3) -> Vec3;
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

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
