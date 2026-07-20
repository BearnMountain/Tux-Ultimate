use std::path::Path;

use crate::engine::assets::{
    handle::Handle, storage::Storage, types::{shader::Shader}
};

pub struct Server {
    textures: Storage<Shader>,
}

impl Server {
    pub fn new() -> Self {
        return Self {
            textures: Storage::new(),
        };
    }
    
    /// # Example: file_path = "shaders/shader.wgsl"
    pub fn load_shader(
        &mut self, 
        device: &wgpu::Device,
        file_path: &Path, // relative path
        vertex_entry: Option<&str>,
        fragment_entry: Option<&str>,
    ) -> Option<Handle<Shader>> {
        let texture_full_path = Path::new("./assets").join(file_path);
        let ventry = match vertex_entry {
            Some(entry) => entry,
            None => "vs_main"
        };
        let fentry = match fragment_entry {
            Some(entry) => entry,
            None => "fs_main"
        };

        let texture = match Shader::new(device, &texture_full_path, ventry, fentry) {
            Ok(texture) => texture,
            Err(err) => {
                log::error!("Server failed loading texture: {err}");
                return None;
            },
        };

        return Some(self.textures.add(texture));
    }

    pub fn get_shader(&self, handle: Handle<Shader>) -> Option<&Shader> {
        return self.textures.get(handle);
    }
}

