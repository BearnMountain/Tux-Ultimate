pub struct RenderPassStorage {
    pass_descriptors: Vec<wgpu::RenderPassDescriptor<'static>>,
}

impl RenderPassStorage {
    pub fn init(
        command_encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let storage = RenderPassStorage {
            pass_descriptors: Vec::new(),
        };

        let (depth_texture, depth_view) = Self::create_depth_texture(&device, &config);

        let game_pass = wgpu::RenderPassDescriptor {
            label: Some("game render pass"),
            color_attachments: &[None],
            depth_stencil_attachment: None,
            ..Default::default()
        };

        let ui_pass = wgpu::RenderPassDescriptor {
            label: Some("ui render pass"),
            color_attachments: &[None],
            depth_stencil_attachment: None,
            ..Default::default()
        };


        return storage;
    }

    pub fn get_ui(
        &self,
        surface: &wgpu::Surface,
    ) -> &wgpu::RenderPassDescriptor<'static> {
        let pass = &mut self.pass_descriptors[0];
        let output = surface.get_current_texture()?;
        let color_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        pass.color_attachments = &[Some(wgpu::RenderPassColorAttachment {
            view: &color_view,
            resolve_target: None,
            ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            depth_slice: None,
        })];


        return pass;
    }

    pub fn get_game(
        &mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        surface: &wgpu::Surface,
    ) -> &wgpu::RenderPassDescriptor<'static> {
        let output = surface.get_current_texture()
            .expect("failed to grab surface texture");
        let color_view = output.texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let (depth_texture, depth_view) = create_depth_texture(&device, &config);

        let pass = &mut self.pass_descriptors[1];
        pass.color_attachments = &[Some(wgpu::RenderPassColorAttachment {
            view: &color_view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.5,
                }),
                store: wgpu::StoreOp::Store,
            },
        })];
        pass.depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_view,
            depth_ops: Some(wgpu::Operations { 
                load: wgpu::LoadOp::Clear(1.0), 
                store: wgpu::StoreOp::Store 
            }),
            stencil_ops: None,
        });
        return &self.pass_descriptors[1];
    }

    fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> (wgpu::Texture, wgpu::TextureView) {
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
}
