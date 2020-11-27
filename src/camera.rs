use glam::{Vec3};

pub struct Camera {
    pub zoom:f32,
    pub pos:Vec3
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            zoom:16.0,
            pos:Vec3::new(0.0, 0.0, 0.0)
        }
    }
}