
use glm::*;
use wgpu::util::DeviceExt;

#[repr(C)] // want c layout for memory
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: Vec3,
    color: Vec3,
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
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

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    return ::core::slice::from_raw_parts(
        (p as *const T) as *const u8, 
        ::core::mem::size_of::<T>()
    );
}

pub fn make_triangle(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices: [Vertex; 3] = [
        Vertex { position: Vec3::new(-0.75, -0.75, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
        Vertex { position: Vec3::new(0.75, -0.75, 0.0), color: Vec3::new(0.5, 1.0, 0.0)},
        Vertex { position: Vec3::new(0.0, 0.75, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
    ];
    let bytes: &[u8] = unsafe {
        any_as_u8_slice(&vertices)
    };

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("BufferDescriptor"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX,
    });

    return buffer;
}

pub fn make_quad(device: &wgpu::Device) -> Mesh {
    let vertices: [Vertex; 4] = [
        Vertex { position: Vec3::new(-0.75, -0.75, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
        Vertex { position: Vec3::new( 0.75, -0.75, 0.0), color: Vec3::new(0.5, 1.0, 0.0)},
        Vertex { position: Vec3::new( 0.75,  0.75, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
        Vertex { position: Vec3::new(-0.75,  0.75, 0.0), color: Vec3::new(1.0, 0.0, 0.0)},
    ];
    let mut bytes: &[u8] = unsafe {
        any_as_u8_slice(&vertices)
    };

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("VertexBuffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::VERTEX,
    });

    let indices: [u16; 6] = [
        0, 1, 2,
        2, 3, 0
    ];
    bytes = unsafe {
        any_as_u8_slice(&indices)
    };

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("IndexBuffer"),
        contents: bytes,
        usage: wgpu::BufferUsages::INDEX,
    });

    return Mesh {
        vertex_buffer,
        index_buffer,
    };
}
