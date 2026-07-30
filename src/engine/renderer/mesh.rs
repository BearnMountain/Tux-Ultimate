use wgpu::util::DeviceExt;

use crate::engine::renderer::{self, coordinate::Coordinate, transform::TransformID};
use super::math::any_as_u8_slice;

#[repr(C)] // want c layout for memory
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: glam::Vec3,
    color: glam::Vec3,
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = 
            wgpu::vertex_attr_array![
                0 => Float32x3, 
                1 => Float32x3,
            ];

        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        };
    }
}

pub type MeshID = usize;

pub struct Mesh {
    // 0 - offset: vertex_buffer
    // offset - end: index_buffer
    pub buffer: wgpu::Buffer,
    pub offset: u64,
    pub index_count: u32,

    pub material_id: renderer::material::MaterialID,
    pub pipeline_id: renderer::pipeline::PipelineID,
    pub transform_id: renderer::transform::TransformID,
}

impl Mesh {
    pub fn make_quad(
        device: &wgpu::Device, 
        material_id: renderer::material::MaterialID,
        pipeline_id: renderer::pipeline::PipelineID,
        transform_id: renderer::transform::TransformID,
        pos: [f32; 2],
        dim: [f32; 2],
    ) -> Mesh {
        let (x, y) = (pos[0], pos[1]);
        let (w, h) = (dim[0], dim[1]);

        let vertices: [Vertex; 4] = [
            Vertex { position: glam::Vec3::new(x,     y - h, 0.0), color: glam::Vec3::new(1.0, 1.0, 1.0)},
            Vertex { position: glam::Vec3::new(x + w, y - h, 0.0), color: glam::Vec3::new(1.0, 1.0, 1.0)},
            Vertex { position: glam::Vec3::new(x + w, y, 0.0), color: glam::Vec3::new(1.0, 1.0, 1.0)},
            Vertex { position: glam::Vec3::new(x,     y, 0.0), color: glam::Vec3::new(1.0, 1.0, 1.0)},
        ];
        let indices: [u16; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        let bytes_vertex: &[u8] = unsafe {
            any_as_u8_slice(&vertices)
        };
        let bytes_index: &[u8] = unsafe {
            any_as_u8_slice(&indices)
        };
        let bytes_merged: &[u8] = &[
            bytes_vertex,
            bytes_index,
        ].concat();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("VertexBuffer"),
            contents: bytes_merged,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX,
        });
        let offset: u64 = bytes_vertex.len().try_into().unwrap();

        return Mesh {
            buffer,
            offset,
            index_count: 6,
            material_id,
            pipeline_id,
            transform_id,
        };
    }
}

pub struct MeshStorage {
    meshes: Vec<Mesh>,
}

impl MeshStorage {
    pub fn new() -> Self {
        return Self {
            meshes: Vec::new(),
        };
    }

    // pipeline -> material
    pub fn get_all_sorted(&mut self) -> &[Mesh] {
        self.meshes.sort_by_key(|mesh| (
            mesh.pipeline_id,
            mesh.material_id,
        ));
        return &self.meshes;
    }

    pub fn get(&self, id: MeshID) -> Option<&Mesh> {
        return self.meshes.get(id);
    }

    pub fn add(&mut self, mesh: Mesh) -> MeshID {
        self.meshes.push(mesh);
        return self.meshes.len() - 1;
    }
}
