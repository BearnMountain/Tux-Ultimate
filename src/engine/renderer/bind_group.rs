pub struct LayoutInfo {
    pub entries: Vec<wgpu::BindGroupLayoutEntry>,
    pub layout: wgpu::BindGroupLayout,
}

pub struct LayoutBuilder {
    device: wgpu::Device,
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

/// 
impl LayoutBuilder {
    pub fn new (device: &wgpu::Device) -> Self {
        return Self {
            device: device.clone(),
            entries: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.entries.clear();
        self.entries = Vec::new();
    }

    pub fn build(&mut self, label: &str) 
        -> LayoutInfo
    {
        return LayoutInfo {
            layout: self.device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(&format!("{label}_layout")),
                    entries: &self.entries,
                }
            ),
            entries: self.entries.clone(),
        };
    }

    pub fn add_buffer(
        &mut self, 
        shader_type: wgpu::ShaderStages, 
        ty: wgpu::BufferBindingType,
    ) -> &mut Self {
        let binding = self.entries.len() as u32;
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: binding,
            visibility: shader_type,
            ty: wgpu::BindingType::Buffer { 
                ty, 
                has_dynamic_offset: false, 
                min_binding_size: None, 
            },
            count: None,
        });
        return self;
    }

    pub fn add_texture_view(
        &mut self,
        shader_type: wgpu::ShaderStages,
        sample_type: wgpu::TextureSampleType,
        view_dimension: wgpu::TextureViewDimension,
    ) -> &mut Self {
        let binding = self.entries.len() as u32;
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: shader_type,
            ty: wgpu::BindingType::Texture {
                sample_type,
                view_dimension,
                multisampled: false,
            },
            count: None,
        });

        return self;
    }

    pub fn add_texture_storage(
        &mut self,
        shader_type: wgpu::ShaderStages,
        access: wgpu::StorageTextureAccess,
        format: wgpu::TextureFormat,
        view_dimension: wgpu::TextureViewDimension,
    ) -> &mut Self {
        let binding = self.entries.len() as u32;
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: shader_type,
            ty: wgpu::BindingType::StorageTexture { 
                access, 
                format, 
                view_dimension, 
            },
            count: None,
        });
        return self;
    }

    pub fn add_texture_sampler(
        &mut self,
        shader_type: wgpu::ShaderStages,
        binding_type: wgpu::SamplerBindingType,
    ) -> &mut Self {
        let binding = self.entries.len() as u32;
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: shader_type,
            ty: wgpu::BindingType::Sampler(binding_type),
            count: None,
        });
        return self;
    }
}

pub struct ResourceBuilder<'a> {
    device: wgpu::Device,
    layout: &'a LayoutInfo,

    entries: Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> ResourceBuilder<'a> {
    pub fn new(
        device: &wgpu::Device, 
        layout: &'a LayoutInfo,
    ) -> Self {
        return Self {
            device: device.clone(),
            layout,
            entries: Vec::new(),
            
        };
    }

    fn reset(&mut self) {
        self.entries.clear();
    }

    pub fn build(&mut self, label: &str) -> anyhow::Result<wgpu::BindGroup> {
        let resource = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("{label}_group")),
            layout: &self.layout.layout.clone(),
            entries: &self.entries,
        });

        self.reset();
        return Ok(resource);
    }

    pub fn buffer(&mut self, buffer: &'a wgpu::Buffer) -> anyhow::Result<&mut Self> {
        let expect = self.layout.entries
            .get(self.entries.len())
            .ok_or(anyhow::anyhow!(
                "buffer binding goes out of bounce, only {} bindings", 
                self.entries.len()
            ))?;

        match expect.ty {
            wgpu::BindingType::Buffer {..} => {},
            _ => {
                return Err(anyhow::anyhow!("incorrect binding resource order. buffer is not next").into());
            },
        }

        self.entries.push(wgpu::BindGroupEntry {
            binding: expect.binding,
            resource: buffer.as_entire_binding(),
        });

        return Ok(self);
    }

    pub fn texture_view(&mut self, buffer: &'a wgpu::TextureView) -> anyhow::Result<&mut Self> {
        let expect = self.layout.entries
            .get(self.entries.len())
            .ok_or(anyhow::anyhow!(
                "texture view binding goes out of bounce, only {} bindings", 
                self.entries.len()
            ))?;

        match expect.ty {
            wgpu::BindingType::Texture {..} => {},
            _ => {
                return Err(anyhow::anyhow!("incorrect binding resource order. texture view is not next").into());
            },
        }

        self.entries.push(wgpu::BindGroupEntry {
            binding: expect.binding,
            resource: wgpu::BindingResource::TextureView(buffer),
        });

        return Ok(self);
    }

    pub fn texture_storage(&mut self, buffer: &'a wgpu::TextureView) -> anyhow::Result<&mut Self> {
        let expect = self.layout.entries
            .get(self.entries.len())
            .ok_or(anyhow::anyhow!(
                "texture storage binding goes out of bounce, only {} bindings", 
                self.entries.len()
            ))?;

        match expect.ty {
            wgpu::BindingType::StorageTexture {..} => {},
            _ => {
                return Err(anyhow::anyhow!("incorrect binding resource order. texture storage is not next").into());
            },
        }

        self.entries.push(wgpu::BindGroupEntry {
            binding: expect.binding,
            resource: wgpu::BindingResource::TextureView(buffer),
        });

        return Ok(self);
    }

    pub fn texture_sampler(&mut self, buffer: &'a wgpu::Sampler) -> anyhow::Result<&mut Self> {
        let expect = self.layout.entries
            .get(self.entries.len())
            .ok_or(anyhow::anyhow!(
                "texture sampler binding goes out of bounce, only {} bindings", 
                self.entries.len()
            ))?;

        match expect.ty {
            wgpu::BindingType::Sampler {..} => {},
            _ => {
                return Err(anyhow::anyhow!("incorrect binding resource order. texture sampler is not next").into());
            },
        }

        self.entries.push(wgpu::BindGroupEntry {
            binding: expect.binding,
            resource: wgpu::BindingResource::Sampler(buffer),
        });

        return Ok(self);
    }
}
