use crate::engine::{assets::{
    types::texture::Texture,
}, renderer::bind_group::{self, LayoutBuilder}};

pub struct Material {
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        label: &str, 
        texture: &Texture, // extract from server::get_texture
        device: &wgpu::Device,
        layout: &LayoutBuilder,
    ) -> Self {

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = bind_group::ResourceBuilder::new(device, layout)
            .texture_view(&texture.view).unwrap()
            .texture_sampler(&sampler).unwrap()
            .build(label).unwrap();

        return Self {
            bind_group,
        };
    }
}
