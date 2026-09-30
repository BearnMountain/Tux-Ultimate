use std::path::Path;

use egui::TextureHandle;
use crate::{engine::assets::types::{RawSource, egui_texture::EGuiTexture, wgpu_texture::WGPUTexture}, util::handle::Handle};

use super::storage::Storage;


pub struct UiServer {
    context: egui::Context,
    textures: Storage<TextureHandle>,
}

impl UiServer {
    pub fn new(context: &egui::Context) -> Self {
        return Self {
            context: context.clone(),
            textures: Storage::new(),
        };
    }

    /// concurrent callable fn to load async resourcs
    pub async fn preload_raw(file_path: &Path) -> anyhow::Result<RawSource> {
        let full_path = Path::new("./assets").join(file_path);
        return RawSource::new(&full_path).await;
    }

    /// Preload text source async, then call this func after 'join'
    pub fn load_texture(
        &mut self, source: RawSource
    ) -> Option<Handle<TextureHandle>> {
        let texture = match EGuiTexture::D2(&self.context, source) {
            Ok(s) => s,
            Err(err) => {
                log::error!("Server failed loading texture: {err}");
                return None;
            }
        };

        return Some(self.textures.add(texture));
    }

    /// returns 'None' if not loaded yet, 'Some(...)' if successfully loaded
    pub fn get_texture(
        &self, handle: 
        Handle<TextureHandle>
    ) -> Option<&TextureHandle> {
        return self.textures.get(handle);
    }
}
