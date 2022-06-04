use crate::utilities::Vec3;

use crate::ray::Ray;
pub struct Camera {
    pub width: f64,
    pub height: f64,
    pub focal_length: f64,

    pub position: Vec3,
}

impl Camera {
    pub fn default(aspect_ratio: f64, zoom: f64) -> Camera {
        Camera {
            width: zoom * aspect_ratio,
            height: zoom,
            focal_length: 1.0,
            position: Vec3::zeros(),
        }
    }

    pub fn top_left(&self) -> Vec3 {
        let horizontal = self.horizontal_vec();
        let vertical = self.vertical_vec();

        self.position - horizontal / 2.0 + vertical / 2.0 - Vec3::new(0.0, 0.0, self.focal_length)
    }

    pub fn horizontal_vec(&self) -> Vec3 {
        Vec3::new(self.width, 0.0, 0.0)
    }

    pub fn vertical_vec(&self) -> Vec3 {
        Vec3::new(0.0, self.height, 0.0)
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
        let origin: Vec3 = self.position;
        let direction: Vec3 = self.top_left() + (u * self.horizontal_vec())
            - (v * self.vertical_vec())
            - self.position;

        Ray { origin, direction }
    }
}
