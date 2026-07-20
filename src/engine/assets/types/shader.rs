use wgpu::ShaderModule;
use std::path::{PathBuf};

pub struct Shader {
    pub shader_module: ShaderModule,
    pub vertex_entry: String,
    pub fragment_entry: String,
}

impl Shader {
    // full filepath required
    pub fn new(
        device: &wgpu::Device,
        file_path: &PathBuf,
        vertex_entry: &str,
        fragment_entry: &str,
    ) -> anyhow::Result<Self> {
        let source = std::fs::read_to_string(file_path)?;

        let shader_source = match file_path.extension().and_then(|e| e.to_str()) {
            Some("wgsl") => wgpu::ShaderSource::Wgsl(source.into()),
            Some(ext) => {
                return Err(anyhow::anyhow!("Unsupported shader extension: {ext}").into());
            }
            None => {
                return Err(anyhow::anyhow!("Shader has no extension"));
            }
        };

        let shader_module = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("ShaderModule"),
                source: shader_source,
            }
        );

        return Ok(Self {
            shader_module,
            vertex_entry: vertex_entry.to_string(),
            fragment_entry: fragment_entry.to_string(),
        });
    }
}
