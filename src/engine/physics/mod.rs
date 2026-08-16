use glam::{Vec2, Vec3, Vec3Swizzles};

use crate::engine::{
    math::EngineMath, physics::{body::RigidBody, bounding_box::MTV, collider::Collider}, renderer::transform::{self, Transform, TransformStorage}
};

pub mod body;
pub mod collider;
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
            bound_q1: Vec2::new(-1000.0, -1000.0),
            bound_q3: Vec2::new(1000.0, 1000.0),
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
            let body = &mut self.bodies[i];
            let collider = &mut self.colliders[i];
    
            let transform_ref = transform_storage
                .get(body.transform_id)
                .expect("rigid body created with incorrect transform id");

            // sync collider transforms with bodies
            collider.set_position(transform_ref.position.xy());
            collider.set_angle(EngineMath::quat_to_xy(transform_ref.rotation));

            // updates objects based on internal states
            if body.is_static {
                continue;
            }

            let pos_xy = transform_ref.position.xy();
            if !(pos_xy.x >= self.bound_q1.x && 
                 pos_xy.x <= self.bound_q3.x && 
                 pos_xy.y >= self.bound_q1.y && 
                 pos_xy.y <= self.bound_q3.y
            ) {
                continue;
            }

            self.bodies[i].update(dt, self.gravity, transform_ref);
        }

        // collision
        for i in 0..n {
            let (coll_left, coll_right) = self.colliders.split_at_mut(i + 1);
            let (bodies_left, bodies_right) = self.bodies.split_at_mut(i + 1);
            let coll_i = &mut coll_left[i];
            let body_i = &mut bodies_left[i];

            for j in 0..bodies_right.len() {
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

                let (tran_i, tran_j) = transform_storage
                    .get2(body_i.transform_id, body_j.transform_id)
                    .expect("rigid body created with incorrect transform id: {i}");

                Self::resolve_collision_position(
                    mtv,
                    body_i,
                    body_j,
                    tran_i,
                    tran_j,
                );

                coll_i.set_position(tran_i.position.xy());
                coll_i.set_angle(EngineMath::quat_to_xy(tran_i.rotation));
                
                coll_j.set_position(tran_j.position.xy());
                coll_j.set_angle(EngineMath::quat_to_xy(tran_j.rotation));

                body_i.resolve_collision_velocity(mtv);
                body_j.resolve_collision_velocity(MTV {
                    magnitude: mtv.magnitude,
                    direction: -mtv.direction,
                });
            }
        }
    }

    // TODO: quad tree for great efficiency
    pub fn add(&mut self, body: RigidBody, collider: Collider) -> usize {
        self.bodies.push(body);
        self.colliders.push(collider);
        return self.bodies.len() - 1;
    }

    fn resolve_collision_position(
        mtv: MTV,
        body_a: &RigidBody,
        body_b: &RigidBody,
        transform_a: &mut Transform,
        transform_b: &mut Transform,
    ) {
        let total_inv_mass = body_a.inv_mass + body_b.inv_mass;

        if total_inv_mass <= 0.0 {
            return;
        }

        let correction = mtv.direction * mtv.magnitude;

        let a_percent = body_a.inv_mass / total_inv_mass;
        let b_percent = body_b.inv_mass / total_inv_mass;

        transform_a.position -= (correction * a_percent).extend(0.0);
        transform_b.position += (correction * b_percent).extend(0.0);
    }
}
