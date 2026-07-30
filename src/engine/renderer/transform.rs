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

pub struct Transform {
    pub position: Vec3,
    pub angle: f32,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        return Self {
            position: Vec3::ZERO,
            angle: 0.0,
            scale: Vec3::ONE,
        };
    }

    pub fn matrix(&self) -> Mat4 {
        return Mat4::from_scale_rotation_translation(
            self.scale,
            Quat::from_rotation_z(self.angle),
            self.position,
        );
    }
}

pub struct TransformStorage {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    capacity: u64,
    transforms: Vec<Transform>,

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
                wgpu::ShaderStages::VERTEX,
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

    pub fn upload(&mut self) {
        let needed = self.transforms.len() as u64;
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

        let matrices: Vec<Mat4> = self.transforms.iter().map(|t| t.matrix()).collect();
        self.queue.write_buffer(
            &self.buffer, 
            0, 
            unsafe { math::any_as_u8_slice(&matrices) }
        );
    }
}
