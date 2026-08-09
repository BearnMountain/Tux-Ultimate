use std::io::{BufReader, Cursor};

use glam::{Vec2, Vec3};
use gltf::Gltf;

use crate::engine::assets::types::TextSource;

pub struct ModelMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

/// shaders/*_model.wgsl input
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

impl ModelVertex {
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 3] = 
            wgpu::vertex_attr_array![
                0 => Float32x3, 
                1 => Float32x2,
                2 => Float32x3, 
            ];

        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        };
    }
}

pub struct Model {
    index: u32,
}

impl Model {
    pub fn create_from_gltf(
        source: &TextSource,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        // let gltf = Gltf::open("assets/characters/test/tux/scene.gltf")
        //     .expect("failed to load gltf file");
        // let gltf_cursor = Cursor::new(&source.source);
        // let gltf_reader = BufReader::new(gltf_cursor);
        // let gltf = Gltf::from_reader(gltf_reader)
        //     .expect("failed to create gltf object");
        //
        // for scene in gltf.scenes() {
        //     for node in scene.nodes() {
        //         println!("Node {}", node.index());
        //
        //         let children = node.children().map(|child| {
        //             dbg!(child);
        //         });
        //
        //         let mesh = node.mesh().expect("got mesh");
        //         let primitives = mesh.primitives();
        //         primitives.for_each(|primitive| {
        //             let material = primitive.material().index();
        //             let indices = primitive.indices();
        //         })
        //     }
        // }

        return Self {
            index: 0,
        };
    }


}

