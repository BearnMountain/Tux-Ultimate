use glam::{Quat, Vec2};

use crate::engine::{physics::bounding_box::MTV, renderer::transform::{Transform, TransformID}};

// good idea, maybe
// #[derive(Clone, Copy, PartialEq, Eq)]
// pub enum BodyType {
//     Dynamic, // updated by collision + game
//     Static, // platform
//     Kinematic, // moved by game, not collision
// }

#[derive(Copy, Clone)]
pub struct RigidBody {
    // stores all information about obj state
    pub transform_id: TransformID,

    // linear motion
    pub velocity: Vec2,
    pub acceleration: Vec2,

    // Angular Motion
    pub angle: f32,
    pub angular_velocity: f32,

    // Frame Input
    pub desired_direction: Vec2,

    // Constants
    pub mass: f32,
    pub inv_mass: f32,
    pub moment_of_inertia: f32,
    pub inv_moment_of_inertia: f32,
    pub move_acceleration: f32,
    pub damping: f32,
    pub angular_damping: f32,
    pub gravity_scale: f32,
    pub resitution: f32,
    pub is_static: bool,

    pub grounded: bool,

    force_accumulator: Vec2,
    impulse_accumulator: Vec2,
    torque_accumulator: f32,
}

impl RigidBody {
    pub fn new(transform_id: TransformID, mass: f32) -> Self {
        let moment_of_inertia = mass; // TODO: for now, update for characters
        let inv_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        let inv_moment_of_inertia = if moment_of_inertia > 0.0 { 1.0 / moment_of_inertia } else { 0.0 };

        return Self {
            transform_id,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            angle: 0.0,
            angular_velocity: 0.0,
            desired_direction: Vec2::ZERO,
            mass,
            inv_mass,
            moment_of_inertia,
            inv_moment_of_inertia,
            move_acceleration: 20.0,
            damping: 0.95,
            angular_damping: 0.98,
            gravity_scale: 1.0,
            resitution: 0.3,
            is_static: false,
            grounded: false,
            force_accumulator: Vec2::ZERO,
            impulse_accumulator: Vec2::ZERO,
            torque_accumulator: 0.0,
        };
    }

    pub fn new_static(transform_id: TransformID) -> Self {
        return Self { 
            is_static: true, 
            mass: 0.0, 
            moment_of_inertia: 0.0, 
            ..Self::new(transform_id, 0.0) 
        };
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.force_accumulator += force;
    }
    pub fn apply_torque(&mut self, torque: f32) {
        self.torque_accumulator += torque;
    }
    pub fn apply_knockback(&mut self, direction: Vec2, force: f32) {
        self.apply_impulse(direction.normalize_or_zero() * force);
    }
    /// player updates itself
    pub fn apply_impulse(&mut self, impulse: Vec2) {
        self.impulse_accumulator += impulse;
    }
    /// other updates player
    pub fn apply_impulse_at_point(
        &mut self, 
        impulse: Vec2, 
        contact_point: Vec2, 
        center_of_mass: Vec2
    ) {
        let r = contact_point - center_of_mass;
        let torque = r.x * impulse.y - r.y * impulse.x;
        self.impulse_accumulator += impulse;
        self.torque_accumulator += torque;
    }

    // update object
    pub fn update(
        &mut self, 
        dt: f32,
        world_gravity: Vec2,
        transform: &mut Transform 
    ) {
        if dt <= 0.0 {
            return;
        }

        // updating from inputs
        let steering = self.desired_direction.normalize_or_zero() 
            * self.move_acceleration;
        let gravity_force = if self.grounded {Vec2::ZERO} 
            else {world_gravity * self.gravity_scale};

        // update self
        self.acceleration = steering + gravity_force + self.force_accumulator * self.inv_mass;
        self.velocity += self.acceleration * dt;
        self.velocity += self.impulse_accumulator * self.inv_mass;
        self.velocity *= self.damping;

        let angular_acceleration = self.torque_accumulator * self.inv_moment_of_inertia;
        self.angular_velocity += angular_acceleration * dt;
        self.angular_velocity *= self.angular_damping;
        self.angle += self.angular_velocity * dt;

        // update world objects
        transform.position += (self.velocity * dt).extend(0.0);
        transform.rotation = Quat::from_rotation_z(self.angle);

        // reset
        self.force_accumulator = Vec2::ZERO;
        self.impulse_accumulator = Vec2::ZERO;
        self.torque_accumulator = 0.0;
        self.grounded = false; // just incase mtv updates obj
    }

    pub fn resolve_collision(
        &mut self, 
        mtv: MTV, 
        other_inv_mass: f32, 
        transform: &mut Transform
    ) {
        let total_inv_mass = self.inv_mass + other_inv_mass;
        let percent_self_mass = if total_inv_mass > 0.0 { self.inv_mass / total_inv_mass} 
            else { 1.0 };
        transform.position += (mtv.direction * mtv.magnitude * percent_self_mass)
            .extend(0.0);

        let v_along_norm = self.velocity.dot(mtv.direction);
        if v_along_norm < 0.0 {
            self.velocity -= mtv.direction * v_along_norm * (1.0 + self.resitution);
        }

        if mtv.direction.y > 0.7 {
            self.grounded = true;
        }
    }

}
