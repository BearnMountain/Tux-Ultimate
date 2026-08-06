use glam::{
    Mat4, Vec3,
    camera::rh::{
        view,
        proj::directx,
    }
};
use wgpu::util::DeviceExt;
use crate::engine::renderer::{bind_group, math};

#[allow(non_camel_case_types)]
pub enum CameraAction {
    MOVE_X,
    MOVE_Y,
    MOVE_Z,
    ZOOM,
    ROTATE_X,
    ROTATE_Y,
}

/// Camera Object Used Outside
pub struct Camera {
    pub uploader: CameraUploader,
    pub transform: CameraTransform, // current 
    previous_transform: CameraTransform, // prev
    render_transform: CameraTransform, // gets uploaded - step between prev and current in ticks
    pub controller: CameraController,
}

impl Camera {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        position: Vec3,
        aspect: f32,
    ) -> Self {
        let transform = CameraTransform::new(position, aspect);
        let uploader = CameraUploader::new(device, queue, &transform);

        uploader.upload(&transform);

        return Self {
            uploader,
            previous_transform: transform,
            render_transform: transform,
            transform,
            controller: CameraController::new(),
        }
    }

    pub fn snapshot_previous(&mut self) {
        self.previous_transform = self.transform;
    }

    pub fn update(
        &mut self,
        action: CameraAction,
        change: f32,
    ) {
        self.controller.action(action, change);
        self.uploader.upload(&self.transform);
    }

    pub fn queue_action(&mut self, action: CameraAction, value: f32) {
        self.controller.action(action, value);
    }
    pub fn tick(&mut self, dt: f32) {
        self.controller.update(&mut self.transform, dt);
    }

    pub fn update_render_transform(&mut self, alpha: f32) {
        self.render_transform = self.previous_transform.lerp(&self.transform, alpha);
        self.uploader.upload(&self.render_transform);
    }
}

/// takes keybinds and controls camera
pub struct CameraController {
    pub move_speed: f32,
    pub rotate_speed: f32,
    pub zoom_speed: f32,

    movement: Vec3,
    rotation: (f32, f32),
    zoom: f32,
}

impl CameraController {
    pub fn new() -> Self {
        return Self {
            move_speed: 5.0,
            rotate_speed: 1.5,
            zoom_speed: 3.0,
            movement: Vec3::ZERO,
            rotation: (0.0, 0.0),
            zoom: 0.0,
        };
    }

    pub fn action(&mut self, action: CameraAction, value: f32) {
        match action {
            CameraAction::MOVE_X => self.movement.x += value,
            CameraAction::MOVE_Y => self.movement.y += value,
            CameraAction::MOVE_Z => self.movement.z += value,

            CameraAction::ZOOM => self.zoom += value,

            CameraAction::ROTATE_X => self.rotation.0 += value,
            CameraAction::ROTATE_Y => self.rotation.1 += value,
        }
    }

    pub fn update(&mut self, camera: &mut CameraTransform, dt: f32) -> bool {
        if self.movement == Vec3::ZERO &&
           self.rotation == (0.0, 0.0) &&
           self.zoom == 0.0 {
            return false;
        }

        // rotation
        camera.rotate(
            self.rotation.0 * self.rotate_speed * dt,
            self.rotation.1 * self.rotate_speed * dt,
        );

        // movement
        let forward = camera.forward_direction();
        let right = camera.right();
        let up = Vec3::Y;

        let mut velocity = 
            right * self.movement.x + 
            up * self.movement.y + 
            forward * self.movement.z; 

        if velocity.length_squared() > 0.0 {
            velocity = velocity.normalize();
        }

        camera.position += velocity * self.move_speed * dt;

        // zoom
        camera.fov_y -= self.zoom * self.zoom_speed.to_radians() * dt;
        camera.fov_y = camera.fov_y.clamp(
            15.0_f32.to_radians(),
            120.0_f32.to_radians(),
        );

        // reset
        self.movement = Vec3::ZERO;
        self.rotation = (0.0, 0.0);
        self.zoom = 0.0;

        return true;
    }
}

#[derive(Clone, Copy)]
/// deals with all math: matrix() to get camera for shaders
pub struct CameraTransform {
    pub position: Vec3,
    yaw: f32,
    pitch: f32,
    fov_y: f32,
    pub aspect: f32,
    // clipping
    znear: f32,
    zfar: f32,
}

impl CameraTransform {
    pub fn new(
        position: Vec3,
        aspect: f32
    ) -> Self {
        return Self {
            position,
            yaw: -std::f32::consts::FRAC_2_PI,
            pitch: 0.0,
            fov_y: 60f32.to_radians(),
            aspect,
            znear: 0.1,
            zfar: 1000.0,
        };
    }

    /// xyz norm vector pointing in looking direction
    pub fn forward_direction(&self) -> Vec3 {
        return Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        ).normalize();
    }

    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch = (self.pitch + delta_pitch).clamp(
            -std::f32::consts::FRAC_PI_2 + 0.01,
            std::f32::consts::FRAC_PI_2 - 0.01,
        );
    }

    pub fn right(&self) -> Vec3 {
        self.forward_direction().cross(Vec3::Y).normalize()
    }

    pub fn lerp(&self, other: &CameraTransform, alpha: f32) -> Self {
        let diff = (other.yaw - self.yaw).rem_euclid(std::f32::consts::TAU);
        let shortest = if diff > std::f32::consts::PI { diff - std::f32::consts::TAU } else { diff };

        return CameraTransform {
            position: self.position.lerp(other.position, alpha),
            yaw: self.yaw + shortest * alpha,
            pitch: self.pitch + (other.pitch - self.pitch) * alpha,
            fov_y: self.fov_y + (other.fov_y - self.fov_y) * alpha,
            aspect: other.aspect,
            znear: self.znear,
            zfar: self.zfar,
        };
    }

    // generating data for shaders
    pub fn view_matrix(&self) -> Mat4 {
        return view::look_to_mat4(
            self.position, 
            self.forward_direction(), 
            Vec3::Y, 
        );
    }

    pub fn projection_matrix(&self) -> Mat4 {
        return directx::perspective(
            self.fov_y,
            self.aspect,
            self.znear,
            self.zfar,
        );
    }

    pub fn matrix(&self) -> Mat4 {
        return self.projection_matrix() * self.view_matrix();
    }
}

/// handles all data that is transfered to the gpu
pub struct CameraUploader {
    pub layout: bind_group::LayoutInfo,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl CameraUploader {
    pub fn new(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        camera: &CameraTransform
    ) -> Self {
        let layout = bind_group::LayoutBuilder::new(device)
            .add_buffer(
                wgpu::ShaderStages::VERTEX,
                wgpu::BufferBindingType::Uniform,
            ).build("camera bind group layout");
        let matrix = camera.matrix();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("CameraBuffer"),
            contents: unsafe {
                math::any_as_u8_slice(&matrix)
            },
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group = bind_group::ResourceBuilder::new(device, &layout)
            .buffer(&buffer).unwrap()
            .build("camera bind group").unwrap();

        return Self {
            layout,
            buffer,
            bind_group,
            device: device.clone(),
            queue: queue.clone(),
        };
    }

    pub fn upload(&self, camera: &CameraTransform) {
        let matrix = camera.matrix();
        self.queue.write_buffer(
            &self.buffer, 
            0, 
            unsafe {
                math::any_as_u8_slice(&matrix)
            },
        );
    }
}
