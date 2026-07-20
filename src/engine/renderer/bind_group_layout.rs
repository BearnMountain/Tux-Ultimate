pub struct Builder<'a> {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
    device: &'a wgpu::Device,
}

impl<'a> Builder<'a> {
    pub fn new (device: &'a wgpu::Device) -> Self {


        return Builder {
            entries: Vec::new(),
            device: device,
        }
    }

    fn reset(&mut self) {
        self.entries.clear();
    }

    pub fn add_material(&mut self) {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as u32,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture { 
                sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                view_dimension: wgpu::TextureViewDimension::D2, 
                multisampled: false 
            },
            count: None,
        });

        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as u32,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(
                wgpu::SamplerBindingType::Filtering
            ),
            count: None,
        });
    }

    pub fn build(&mut self, label: &str) -> wgpu::BindGroupLayout {
        let layout = self.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some(label),
                entries: &self.entries,
            }
        );
        self.reset();

        return layout;
    }
}
