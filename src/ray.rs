use nalgebra::Vector3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn default() -> Self {
        Ray {
            origin: Vector3::<f64>::zeros(),
            direction: Vector3::<f64>::zeros(),
        }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + (t * self.direction)
    }
}
