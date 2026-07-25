use crate::engine::assets::types::{RawSource};

// raw data turned to gpu resource
pub struct Texture {
    pub texture: wgpu::Texture,
}

impl Texture {
    // turns source code to gpu data
    #[allow(non_snake_case)]
    pub fn D2(
        label: &str,
        device: &wgpu::Device,
        source: &RawSource,
    ) -> anyhow::Result<Self> {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: source.width,
                height: source.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[
                wgpu::TextureFormat::Rgba8Unorm,
            ],
        });
        
        return Ok(Self {
            texture,
        });
    }
}
