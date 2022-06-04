use crate::utilities::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn default() -> Self {
        Ray {
            origin: Vec3::zeros(),
            direction: Vec3::zeros(),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (t * self.direction)
    }
}
