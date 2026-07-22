pub struct Builder<'a> {
    device: &'a wgpu::Device,
    layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
    group_entries: Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new (device: &'a wgpu::Device) -> Self {
        return Builder {
            device: device,
            layout_entries: Vec::new(),
            group_entries: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.layout_entries.clear();
        self.group_entries.clear();
    }

    pub fn build(&mut self, label: &str) 
        -> (wgpu::BindGroupLayout, wgpu::BindGroup)
    {
        let layout = self.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some(&format!("{label}_layout")),
                entries: &self.layout_entries,
            }
        );

        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("{label}_group")),
            layout: &layout,
            entries: &self.group_entries,
        });

        self.reset();
        return (layout, group);
    }

    pub fn add_buffer(
        &mut self, 
        shader_type: wgpu::ShaderStages, 
        ty: wgpu::BufferBindingType,
        buffer: &'a wgpu::Buffer
    ) -> &mut Self {
        let binding = self.layout_entries.len() as u32;
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: shader_type,
            ty: wgpu::BindingType::Buffer { 
                ty, 
                has_dynamic_offset: false, 
                min_binding_size: None, 
            },
            count: None,
        });
        self.group_entries.push(wgpu::BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        });
        return self;
    }

    pub fn add_texture_view(
        &mut self,
        shader_type: wgpu::ShaderStages,
        sample_type: wgpu::TextureSampleType,
        view_dimension: wgpu::TextureViewDimension,
        texture_view: &'a wgpu::TextureView,
    ) -> &mut Self {
        let binding = self.layout_entries.len() as u32;
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: shader_type,
            ty: wgpu::BindingType::Texture {
                sample_type,
                view_dimension,
                multisampled: false,
            },
            count: None,
        });
        self.group_entries.push(wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(texture_view),
        });
        return self;
    }

    pub fn add_texture_storage(
        &mut self,
        shader_type: wgpu::ShaderStages,
        access: wgpu::StorageTextureAccess,
        format: wgpu::TextureFormat,
        view_dimension: wgpu::TextureViewDimension,
        storage_texture_view: &'a wgpu::TextureView,
    ) -> &mut Self {
        let binding = self.layout_entries.len() as u32;
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: shader_type,
            ty: wgpu::BindingType::StorageTexture { 
                access, 
                format, 
                view_dimension, 
            },
            count: None,
        });
        self.group_entries.push(wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(storage_texture_view),
        });

        return self;
    }

    pub fn add_texture_sampler(
        &mut self,
        shader_type: wgpu::ShaderStages,
        binding_type: wgpu::SamplerBindingType,
        texture_sampler: &'a wgpu::Sampler,
    ) -> &mut Self {
        let binding = self.layout_entries.len() as u32;
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: shader_type,
            ty: wgpu::BindingType::Sampler(binding_type),
            count: None,
        });
        self.group_entries.push(wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Sampler(texture_sampler),
        });
        self
    }
}
