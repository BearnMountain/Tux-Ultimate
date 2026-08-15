use glam::{Vec2, Vec3, Vec3Swizzles};

use crate::engine::{
    math::EngineMath, physics::{body::RigidBody, bounding_box::MTV, collider::Collider}, renderer::transform::{self, TransformStorage}
};

pub mod body;
pub mod collider;
pub mod collisions;
mod bounding_box;

pub struct PhysicsWorld {
    // cortessian based quadrant bounds
    bound_q1: Vec2,
    bound_q3: Vec2,

    pub bodies: Vec<RigidBody>,
    pub colliders: Vec<Collider>,
    pub gravity: Vec2,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        return Self { 
            bound_q1: Vec2::new(1000.0, 1000.0),
            bound_q3: Vec2::new(-1000.0, -1000.0),
            bodies: Vec::new(),
            colliders: Vec::new(),
            gravity: Vec2::new(0.0, -9.81),
        };
    }
}

impl PhysicsWorld {

    pub fn update(
        &mut self, 
        dt: f32,
        transform_storage: &mut TransformStorage,
    ) {
        // update bodies + colliders
        let n = self.bodies.len();
        for i in 0..n {
            if self.bodies[i].is_static {
                continue;
            }

            let transform_ref = transform_storage
                .get(self.bodies[i].transform_id)
                .expect("rigid body created with incorrect transform id");

            let pos_xy = transform_ref.position.xy();
            if !(pos_xy.x >= self.bound_q1.x && 
                 pos_xy.x <= self.bound_q3.x && 
                 pos_xy.y >= self.bound_q1.y && 
                 pos_xy.y <= self.bound_q3.y
            ) {
                continue;
            }

            let body = &mut self.bodies[i];
            let collider = &mut self.colliders[i];

            body.update(dt, self.gravity, transform_ref);

            // sync collider transforms with bodies
            collider.set_position(transform_ref.position.xy());
            collider.set_angle(EngineMath::quat_to_xy(transform_ref.rotation));
        }

        // collision
        for i in 0..n {
            let (coll_left, coll_right) = self.colliders.split_at_mut(i);
            let (bodies_left, bodies_right) = self.bodies.split_at_mut(i);
            let coll_i = &mut coll_left[i];
            let body_i = &mut bodies_left[i];

            for j in 0..(n-i) {
                let body_j = &mut bodies_right[j];
                if body_i.is_static && body_j.is_static {
                    continue;
                }

                // store collision
                let coll_j = &mut coll_right[j];
                let mtv = coll_i.collision(coll_j);
                if mtv.magnitude <= 0.0 {
                    continue;
                }

                let (tran_i, tran_j) = transform_storage.get2(
                    body_i.transform_id,
                    body_j.transform_id,
                ).expect("rigid body created with incorrect transform id: {i}");
                body_i.resolve_collision(
                    mtv, 
                    body_j.inv_mass, 
                    tran_i,
                );
                body_j.resolve_collision(
                    MTV { magnitude: mtv.magnitude, direction: -mtv.direction}, 
                    body_i.inv_mass, 
                    tran_j,
                );
            }
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
