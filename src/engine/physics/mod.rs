use glam::{Vec2, Vec3};

use crate::engine::{
    physics::{body::RigidBody, collider::Collider}, renderer::transform::TransformStorage
};

pub mod body;
pub mod collider;
pub mod collisions;
mod bounding_box;
mod test;

pub struct PhysicsWorld {
    // cortessian based quadrant bounds
    bound_q1: Vec2,
    bound_q3: Vec2,

    pub bodies: Vec<RigidBody>,
    pub colliders: Vec<Collider>,
    pub gravity: Vec3,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        return Self { 
            bound_q1: Vec2::new(1000.0, 1000.0),
            bound_q3: Vec2::new(-1000.0, -1000.0),
            bodies: Vec::new(),
            colliders: Vec::new(),
            gravity: Vec3::new(0.0, -9.81, 0.0),
        };
    }
}

impl PhysicsWorld {
    pub fn update(
        &mut self, 
        tick: f32,
    ) {
        
        for body in &mut self.bodies {
            
        }
    }


    // TODO: quad tree for great efficiency
    pub fn add(&mut self, body: RigidBody, collider: Collider) -> usize {
        self.bodies.push(body);
        self.colliders.push(collider);
        return self.bodies.len() - 1;
    }



    // pub fn update(
    //     &mut self,
    //     transforms: &mut TransformStorage,
    //     dt: f32,
    // ) {
    //
    //     for body in &mut self.bodies {
    //         let Some(transform) = transforms.get(body.transform_id) else {
    //             continue;
    //         };
    //
    //         //--------------------------------------------------
    //         // Movement
    //         //--------------------------------------------------
    //
    //         body.acceleration.x =
    //             body.desired_direction.x * body.move_acceleration;
    //
    //         body.acceleration.y = 
    //             body.desired_direction.y * 
    //             body.move_acceleration -
    //             body.gravity;
    //
    //
    //         body.velocity += body.acceleration * dt;
    //
    //         body.velocity.x *= body.damping.powf(dt);
    //
    //         // update transform
    //         transform.position += Vec3::new(
    //             body.velocity.x * dt,
    //             body.velocity.y * dt,
    //             0.0
    //         );
    //
    //         // Fake ground, remove and replace with collider later
    //         if transform.position.y < -5.0 {
    //             transform.position.y = -5.0;
    //
    //             if body.velocity.y < 0.0 {
    //                 body.velocity.y = 0.0;
    //             }
    //
    //             body.grounded = true;
    //         } else {
    //             body.grounded = false;
    //         }
    //
    //         // clean for next frame
    //         body.desired_direction = Vec2::ZERO;    
    //     }
    // }
}
