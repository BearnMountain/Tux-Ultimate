use std::{path::Path};

use crate::engine::assets::{
    handle::Handle, storage::Storage, types::{
        RawSource, TextSource, shader::Shader, texture::Texture,
    },
};

pub struct Server {
    device: wgpu::Device,
    queue: wgpu::Queue,
    shaders: Storage<Shader>,
    textures: Storage<Texture>,
}

impl Server {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        return Self {
            device,
            queue,
            shaders: Storage::new(),
            textures: Storage::new(),
        };
    }

    /// concurrent callable fn to load async resourcs
    pub async fn preload_raw(file_path: &Path) -> anyhow::Result<RawSource> {
        let full_path = Path::new("./assets").join(file_path);
        return RawSource::new(&full_path).await;
    }

    pub async fn preload_text(file_path: &Path) -> anyhow::Result<TextSource> {
        let full_path = Path::new("./assets").join(file_path);
        return TextSource::new(&full_path).await;
    }
    
    /// Preload text source async, then call this func after 'join'
    pub fn load_shader(
        &mut self, 
        source: TextSource,
        vertex_entry: Option<&str>,
        fragment_entry: Option<&str>,
    ) -> Option<Handle<Shader>> {
        let ventry = match vertex_entry {
            Some(entry) => entry,
            None => "vs_main"
        };
        let fentry = match fragment_entry {
            Some(entry) => entry,
            None => "fs_main"
        };

        let shader = match Shader::new(&self.device, source, ventry, fentry) {
            Ok(s) => s,
            Err(err) => {
                log::error!("Server failed loading texture: {err}");
                return None;
            },
        };

        return Some(self.shaders.add(shader));
    }

    pub fn load_texture(
        &mut self, 
        source: RawSource,
    ) -> Option<Handle<Texture>> {
        let texture = match Texture::D2("texture", &self.device, &self.queue, source) {
            Ok(s) => s,
            Err(err) => {
                log::error!("Server failed loading texture: {err}");
                return None;
            },
        };

        return Some(self.textures.add(texture));
    }

    /// returns 'None' if not loaded yet, 'Some(...)' if successfully loaded
    pub fn get_shader(&self, handle: Handle<Shader>) -> Option<&Shader> {
        return self.shaders.get(handle);
    }

    pub fn get_texture(&self, handle: Handle<Texture>) -> Option<&Texture> {
        return self.textures.get(handle);
    }
}

