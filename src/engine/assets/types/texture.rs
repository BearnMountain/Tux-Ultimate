use crate::engine::assets::types::{RawSource};

// raw data turned to gpu resource
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl Texture {
    // turns source code to gpu data
    #[allow(non_snake_case)]
    pub fn D2(
        label: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        source: RawSource,
    ) -> anyhow::Result<Self> {
        let size = wgpu::Extent3d {
            width: source.width,
            height: source.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT 
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        // write data to texture 
        queue.write_texture(
            texture.as_image_copy(),
            &source.pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * source.width),
                rows_per_image: Some(source.height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        return Ok(Self {
            texture,
            view,
        });
    }
}
