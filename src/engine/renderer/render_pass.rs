use serde::de;

#[derive(Eq, PartialEq)]
pub enum RenderPassStage {
    UI,
    GAME,
}

/// RenderPass storage stores all default renderpasses
/// for the game
///     
/// Notes:
/// - couple RenderPassStorage::resize with window resize to not have rendering issues
/// - always start with begin_game(...) as it clears previous frame
pub struct RenderPassStorage {
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
}

impl RenderPassStorage {
    /// creates storage for renderpasses that work for:
    /// - game
    /// - ui
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let (depth_texture, depth_view) = Self::create_depth_texture(&device, &config);

        return Self {
            depth_texture,
            depth_view,
        };
    }

    /// required to update textures for each renderpass
    pub fn resize(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        let (depth_texture, depth_view) = Self::create_depth_texture(&device, &config);

        self.depth_texture = depth_texture;
        self.depth_view = depth_view;
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float, // must match your pipeline's depth_stencil format
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        return (texture, view);
    }

    /// First in order, everything after
    pub fn begin_game<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        color_view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("game render pass"),

            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color_view,
                resolve_target: None,

                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },

                depth_slice: None,
            })],

            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_view,

                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),

                stencil_ops: None,
            }),

            ..Default::default()
        })
    }

    pub fn begin_ui<'a>(
        &'a self,
        encoder: &'a mut wgpu::CommandEncoder,
        color_view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("ui render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color_view,
                resolve_target: None,

                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },

                depth_slice: None,
            })],
            depth_stencil_attachment: None,

            ..Default::default()
        })
    }
}
