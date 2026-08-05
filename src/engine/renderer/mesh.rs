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
            Vertex { position: glam::Vec3::new(x + w, y,     0.0), color: glam::Vec3::new(1.0, 1.0, 1.0)},
            Vertex { position: glam::Vec3::new(x,     y,     0.0), color: glam::Vec3::new(1.0, 1.0, 1.0)},
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

    pub fn make_cube(
        device: &wgpu::Device,
        material_id: renderer::material::MaterialID,
        pipeline_id: renderer::pipeline::PipelineID,
        transform_id: renderer::transform::TransformID,
        pos: [f32; 3],
        dim: [f32; 3],
    ) -> Self {
        let (x, y, z) = (pos[0], pos[1], pos[2]);
        let (w, h, d) = (dim[0], dim[1], dim[2]);

        let (x0, x1) = (x, x + w);
        let (y0, y1) = (y, y + h);
        let (z0, z1) = (z, z + d);

        let white = glam::Vec3::new(1.0, 0.5, 1.0);

        let vertices: [Vertex; 24] = [
            // Front (+Z)
            Vertex { position: glam::Vec3::new(x0, y0, z1), color: white },
            Vertex { position: glam::Vec3::new(x1, y0, z1), color: white },
            Vertex { position: glam::Vec3::new(x1, y1, z1), color: white },
            Vertex { position: glam::Vec3::new(x0, y1, z1), color: white },

            // Back (-Z)
            Vertex { position: glam::Vec3::new(x1, y0, z0), color: white },
            Vertex { position: glam::Vec3::new(x0, y0, z0), color: white },
            Vertex { position: glam::Vec3::new(x0, y1, z0), color: white },
            Vertex { position: glam::Vec3::new(x1, y1, z0), color: white },

            // Left (-X)
            Vertex { position: glam::Vec3::new(x0, y0, z0), color: white },
            Vertex { position: glam::Vec3::new(x0, y0, z1), color: white },
            Vertex { position: glam::Vec3::new(x0, y1, z1), color: white },
            Vertex { position: glam::Vec3::new(x0, y1, z0), color: white },

            // Right (+X)
            Vertex { position: glam::Vec3::new(x1, y0, z1), color: white },
            Vertex { position: glam::Vec3::new(x1, y0, z0), color: white },
            Vertex { position: glam::Vec3::new(x1, y1, z0), color: white },
            Vertex { position: glam::Vec3::new(x1, y1, z1), color: white },

            // Top (+Y)
            Vertex { position: glam::Vec3::new(x0, y1, z1), color: white },
            Vertex { position: glam::Vec3::new(x1, y1, z1), color: white },
            Vertex { position: glam::Vec3::new(x1, y1, z0), color: white },
            Vertex { position: glam::Vec3::new(x0, y1, z0), color: white },

            // Bottom (-Y)
            Vertex { position: glam::Vec3::new(x0, y0, z0), color: white },
            Vertex { position: glam::Vec3::new(x1, y0, z0), color: white },
            Vertex { position: glam::Vec3::new(x1, y0, z1), color: white },
            Vertex { position: glam::Vec3::new(x0, y0, z1), color: white },
        ];

        let indices: [u16; 36] = [
            // Front
            0, 1, 2,
            2, 3, 0,

            // Back
            4, 5, 6,
            6, 7, 4,

            // Left
            8, 9, 10,
            10, 11, 8,

            // Right
            12, 13, 14,
            14, 15, 12,

            // Top
            16, 17, 18,
            18, 19, 16,

            // Bottom
            20, 21, 22,
            22, 23, 20,
        ];

        let byte_vertex: &[u8] = unsafe { any_as_u8_slice(&vertices) };
        let byte_index: &[u8] = unsafe { any_as_u8_slice(&indices) };
        let merge: &[u8] = &[byte_vertex, byte_index].concat();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("CubeBuffer"),
            contents: merge,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX,
        });

        return Self {
            buffer,
            offset: byte_vertex.len() as u64,
            index_count: 36,
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
