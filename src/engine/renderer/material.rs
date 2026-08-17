use crate::engine::{
    assets::types::texture::Texture, 
    renderer::bind_group::{self, LayoutBuilder, LayoutInfo}};

pub type MaterialID = usize;

pub struct Material {
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(
        label: &str, 
        texture: &Texture, // extract from server::get_texture
        device: &wgpu::Device,
        layout: &LayoutInfo,
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
            .build(label).expect("failed to create bind group for new material");

        return Self {
            bind_group,
        };
    }
}

// pub struct MaterialStorage {
//     materials: Vec<Material>,
// }
//
// impl MaterialStorage {
//     pub fn new() -> Self {
//         return Self {
//             materials: Vec::new(),
//         };
//     }
//
//     pub fn get(&self, id: MaterialID) -> Option<&Material> {
//         return self.materials.get(id);
//     }
//
//     pub fn add(&mut self, material: Material) -> MaterialID {
//         self.materials.push(material);
//         return self.materials.len() - 1;
//     }
// }
