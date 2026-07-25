use wgpu::ShaderModule;

use crate::engine::assets::types::{TextSource};

// raw data turned to gpu resource
pub struct Shader {
    pub shader_module: ShaderModule,
    pub vertex_entry: String,
    pub fragment_entry: String,
}

impl Shader {
    // turns source code to gpu data
    pub fn new(
        device: &wgpu::Device,
        source: &TextSource,
        vertex_entry: &str,
        fragment_entry: &str,
    ) -> anyhow::Result<Self> {
        let shader_source = match source.path.extension().and_then(|e| e.to_str()) {
            Some("wgsl") => wgpu::ShaderSource::Wgsl(source.source.clone().into()),
            Some(ext) => { return Err(anyhow::anyhow!("Shader extension not supported: {ext}").into()); },
            None => { return Err(anyhow::anyhow!("No shader extension").into()); },
        };

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor { 
            label: Some("ShaderModule"), 
            source: shader_source,
        });
        
        return Ok(Self {
            shader_module,
            vertex_entry: vertex_entry.to_string(),
            fragment_entry: fragment_entry.to_string(),
        });
    }
}
