use egui::{ColorImage, TextureHandle, TextureOptions};

use crate::engine::assets::types::RawSource;


// raw data turned to gpu resource
pub struct EGuiTexture {
    pub texture: egui::TextureHandle,
}

impl EGuiTexture {
    // turns source code to gpu data
    #[allow(non_snake_case)]
    pub fn D2(
        context: &egui::Context,
        source: RawSource,
    ) -> anyhow::Result<TextureHandle> {
        let image = image::load_from_memory(&source.pixels)?
            .to_rgb8();

        let size = [image.width() as usize, image.height() as usize];

        let color_image = ColorImage::from_rgba_unmultiplied(
            size, 
            image.as_raw()
        );

        return Ok(context.load_texture(
            "my_image",
            color_image,
            TextureOptions::LINEAR,
        ));
    }
}
