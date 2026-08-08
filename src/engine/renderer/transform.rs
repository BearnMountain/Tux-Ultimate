use glam::{
    Mat4,
    Quat,
    Vec3,
};

use crate::engine::renderer::bind_group::LayoutInfo;

use super::bind_group;
use super::math;

pub type TransformID = usize;

const MAX_TRANSFORMS: u64 = 1024;

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Self {
        return Self {
            position,
            rotation,
            scale,
        };
    }

    pub fn default() -> Self {
        return Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        };
    }

    /// helps to transform between frame, not instantaneously
    pub fn lerp(&self, other: &Transform, alpha: f32) -> Transform {
        Transform {
            position: self.position.lerp(other.position, alpha),
            rotation: self.rotation.slerp(other.rotation, alpha),
            scale: self.scale.lerp(other.scale, alpha),
        }
    }

    /// aint no one rotating a quat by hand
    pub fn rotate(
        &mut self,
        dx: f32, 
        dy: f32,
        dz: f32,
    ) {
        self.rotation = (
            self.rotation *
            Quat::from_euler(glam::EulerRot::YXZ, dy, dx, dz)
        ).normalize();
    }

    pub fn matrix(&self) -> Mat4 {
        return Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.position,
        );
    }
}

pub struct TransformStorage {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    capacity: u64,
    transforms: Vec<Transform>,
    // previous_transforms: Vec<Transform>, // snapstock each tick
    // render_transforms: Vec<Transform>, // interpolated

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub layout: bind_group::LayoutInfo
}

impl TransformStorage {
    /// Requires that pipeline has:
    /// .add_buffer(
    ///     wgpu::ShaderStages::VERTEX, 
    ///     wgpu::BufferBindingType::Storage { read_only: true }, // stores 128mb 
    /// )
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let capacity = MAX_TRANSFORMS;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("transform buffer"),
            size: capacity * std::mem::size_of::<Mat4>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let layout = bind_group::LayoutBuilder::new(device)
            .add_buffer(
                wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                wgpu::BufferBindingType::Storage { read_only: true },
            )
            .build("transform bind group layout");
        let bind_group = bind_group::ResourceBuilder::new(device, &layout)
            .buffer(&buffer).unwrap()
            .build("transform bind group")
            .expect("failed to create transform bind group");
        return Self {
            buffer,
            bind_group,
            capacity,
            transforms: Vec::new(),
            // previous_transforms: Vec::new(),
            // render_transforms: Vec::new(),
            device: device.clone(),
            queue: queue.clone(),
            layout,
        };
    }

    pub fn get(&mut self, id: TransformID) -> Option<&mut Transform> {
        return self.transforms.get_mut(id);
    }

    pub fn add(&mut self, transform: Transform) -> TransformID {
        self.transforms.push(transform);
        return self.transforms.len() - 1;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Transform> {
        self.transforms.iter()
    }

    /// smoothing out animation and transforms
    // pub fn snapshot_previous(&mut self) {
    //     self.previous_transforms.clear();
    //     self.previous_transforms.extend_from_slice(&self.transforms);
    // }
    //
    // pub fn interpolate(&mut self, alpha: f32) {
    //     debug_assert_eq!(
    //         self.previous_transforms.len(), self.transforms.len(),
    //         "transform mismatch, cant add between snapshot and interpolate functions"
    //     );
    //     self.render_transforms.clear();
    //     self.render_transforms.reserve(self.transforms.len());
    //
    //     for (i, current) in self.transforms.iter().enumerate() {
    //         if let Some(previous) = self.previous_transforms.get(i) {
    //             // Both exist: smoothly interpolate
    //             self.render_transforms.push(previous.lerp(current, alpha));
    //         } else {
    //             // Newly spawned entity (no snapshot history yet): render directly at current position
    //             self.render_transforms.push(*current);
    //         }
    //     }
    // }

    pub fn upload(&mut self) {
        let needed = self.transforms.len() as u64;

        // accounts for change in transformer size
        if needed > self.capacity {
            let new_capacity = (needed * 2).max(self.capacity * 2);
            self.buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("transform buffer"),
                size: new_capacity * std::mem::size_of::<Mat4>() as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.bind_group = bind_group::ResourceBuilder::new(&self.device, &self.layout)
                .buffer(&self.buffer).unwrap()
                .build("transform bind group")
                .expect("failed to create transform bind group");
            self.capacity = new_capacity;
        }

        // Convert interpolated render_transforms to matrices
        let matrices: Vec<Mat4> = self.transforms
            .iter()
            .map(|t| t.matrix())
            .collect();

        // uploads to gpu
        let bytes = unsafe { math::any_slice_as_u8_slice(&matrices) };
        self.queue.write_buffer(&self.buffer, 0, bytes);
    }
}
