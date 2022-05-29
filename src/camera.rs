use nalgebra::Vector3;
pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub focal_length: f32,

    pub position: Vector3<f32>,
}

impl Camera {
    pub fn default() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        Camera {
            width: 2.0 * aspect_ratio,
            height: 2.0,
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
}
