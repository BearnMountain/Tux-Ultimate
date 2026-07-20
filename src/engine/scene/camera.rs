
use glm::*;

pub struct Camera {
    coord: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        return Self {
            coord: Vec3::new(0.0, 0.0, 0.0),
        };
    }
}
