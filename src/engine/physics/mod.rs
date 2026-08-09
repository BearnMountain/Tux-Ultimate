use glam::{Vec2, Vec3};

use crate::engine::renderer::transform::{self, TransformStorage};

pub mod physics_body;
pub mod collisions;

pub struct PhysicsWorld {
    pub bodies: Vec<physics_body::RigidBody>,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        return Self { 
            bodies: Vec::new(),
        };
    }
}

impl PhysicsWorld {
    pub fn update(
        &mut self,
        transforms: &mut TransformStorage,
        dt: f32,
    ) {

        for body in &mut self.bodies {
            let Some(transform) = transforms.get(body.transform_id) else {
                continue;
            };

            //--------------------------------------------------
            // Movement
            //--------------------------------------------------

            body.acceleration.x =
                body.desired_direction.x * body.move_acceleration;

            body.acceleration.y = 
                body.desired_direction.y * 
                body.move_acceleration -
                body.gravity;


            body.velocity += body.acceleration * dt;

            body.velocity.x *= body.damping.powf(dt);

            // update transform
            transform.position += Vec3::new(
                body.velocity.x * dt,
                body.velocity.y * dt,
                0.0
            );

            // Fake ground, remove and replace with collider later
            if transform.position.y < -5.0 {
                transform.position.y = -5.0;

                if body.velocity.y < 0.0 {
                    body.velocity.y = 0.0;
                }

                body.grounded = true;
            } else {
                body.grounded = false;
            }

            // clean for next frame
            body.desired_direction = Vec2::ZERO;    
        }
    }
}
