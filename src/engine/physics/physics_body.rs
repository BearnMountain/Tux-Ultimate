use glam::{Vec2, Vec3};

use crate::engine::renderer::transform::{TransformID};



#[derive(Copy, Clone)]
pub struct RigidBody {
    pub transform_id: TransformID,
    
    // Motion
    pub velocity: Vec2,
    pub acceleration: Vec2,

    // Input for this frame
    pub desired_direction: Vec2,

    // Constants
    pub move_acceleration: f32,
    pub damping: f32,
    pub gravity: f32,

    pub grounded: bool,
}

impl RigidBody {
    pub fn new(transform_id: TransformID) -> Self {
        return Self {
            transform_id,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            desired_direction: Vec2::ZERO,
            move_acceleration: 20.0,
            damping: 0.95,
            gravity: 9.81,
            grounded: false,
        };
    }
}
