use nalgebra::Vector3;

use crate::ray::Ray;
pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub focal_length: f32,

    pub position: Vector3<f32>,
}

impl Camera {
    pub fn default(aspect_ratio: f32, zoom: f32) -> Camera {
        Camera {
            width: zoom * aspect_ratio,
            height: zoom,
            focal_length: 1.0,
            position: Vector3::<f32>::zeros(),
        }
    }

    pub fn top_left(&self) -> Vector3<f32> {
        let horizontal = Vector3::<f32>::new(self.width, 0.0, 0.0);
        let vertical = Vector3::<f32>::new(0.0, self.height, 0.0);

        self.position - horizontal / 2.0 + vertical / 2.0
            - Vector3::<f32>::new(0.0, 0.0, self.focal_length)
    }

    pub fn horizontal_vec(&self) -> Vector3<f32> {
        Vector3::<f32>::new(self.width, 0.0, 0.0)
    }

    pub fn vertical_vec(&self) -> Vector3<f32> {
        Vector3::<f32>::new(0.0, self.height, 0.0)
    }

    pub fn cast_ray(&self, u: f32, v: f32) -> Ray {
        let origin: Vector3<f32> = self.position;
        let direction: Vector3<f32> = self.top_left() + (u * self.horizontal_vec())
            - (v * self.vertical_vec())
            - self.position;

        Ray { origin, direction }
    }
}
