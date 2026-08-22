use glam::Vec3;
use wgpu::util::DeviceExt;

use crate::engine::renderer::{self, coordinate::Coordinate};
use super::math::any_as_u8_slice;
use super::transform::Transform;

#[repr(C)] // want c layout for memory
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: glam::Vec3,
}

impl Vertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 1] = 
            wgpu::vertex_attr_array![
                0 => Float32x3, 
            ];

        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        };
    }
}

pub struct Mesh {
    // 0 - offset: vertex_buffer
    // offset - end: index_buffer
    pub buffer: wgpu::Buffer,
    pub offset: u64,
    pub index_count: u32,
}

impl Mesh {
    pub fn make_cube(
        device: &wgpu::Device,
        dim: [f32; 3],
    ) -> Self {
        let (w, h, d) = (dim[0], dim[1], dim[2]);

        let (x0, x1) = (-w/2.0, w/2.0);
        let (y0, y1) = (-h/2.0, h/2.0);
        let (z0, z1) = (-d/2.0, d/2.0);

        let vertices: [Vertex; 24] = [
            // Front (+Z)
            Vertex { position: Vec3::new(x0, y0, z1) },
            Vertex { position: Vec3::new(x1, y0, z1) },
            Vertex { position: Vec3::new(x1, y1, z1) },
            Vertex { position: Vec3::new(x0, y1, z1) },

            // Back (-Z)
            Vertex { position: Vec3::new(x1, y0, z0) },
            Vertex { position: Vec3::new(x0, y0, z0) },
            Vertex { position: Vec3::new(x0, y1, z0) },
            Vertex { position: Vec3::new(x1, y1, z0) },

            // Left (-X)
            Vertex { position: Vec3::new(x0, y0, z0) },
            Vertex { position: Vec3::new(x0, y0, z1) },
            Vertex { position: Vec3::new(x0, y1, z1) },
            Vertex { position: Vec3::new(x0, y1, z0) },

            // Right (+X)
            Vertex { position: Vec3::new(x1, y0, z1) },
            Vertex { position: Vec3::new(x1, y0, z0) },
            Vertex { position: Vec3::new(x1, y1, z0) },
            Vertex { position: Vec3::new(x1, y1, z1) },

            // Top (+Y)
            Vertex { position: Vec3::new(x0, y1, z1) },
            Vertex { position: Vec3::new(x1, y1, z1) },
            Vertex { position: Vec3::new(x1, y1, z0) },
            Vertex { position: Vec3::new(x0, y1, z0) },

            // Bottom (-Y)
            Vertex { position: Vec3::new(x0, y0, z0) },
            Vertex { position: Vec3::new(x1, y0, z0) },
            Vertex { position: Vec3::new(x1, y0, z1) },
            Vertex { position: Vec3::new(x0, y0, z1) },
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
        };
    }
}
