use std::path::Path;
use crate::engine::assets::{
    server::Server,
    types::texture::Texture,
};

pub struct Material {
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        texture: &Texture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: &str, 
        layout: &wgpu::BindGroupLayout
    ) -> Self {
        let file: Server = Server::new();
    }
}
