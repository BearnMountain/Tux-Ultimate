use std::{path::Path, sync::Arc};

use winit::window::Window;

use crate::engine::renderer::context::RenderContext;

/// imgui
pub struct UI {
    window: Arc<Window>,
    imgui_context: imgui::Context,

    font_size: f32,
}

impl UI {
    pub fn init(window: &Arc<Window>, graphics: &RenderContext) -> Self {
        let mut ui: UI = UI {
            window: window.clone(),
            imgui_context: imgui::Context::create(),
            font_size: 13.0,
        };

        // create instance
        let mut platform = imgui_winit_support::WinitPlatform::new(
            &mut ui.imgui_context
        );
        platform.attach_window(
            ui.imgui_context.io_mut(), 
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        ui.imgui_context.set_ini_filename(None);

        // setting default font
        ui.set_font(&Path::new("assets/fonts/JetBrainsMono-Regular.ttf"));



        return ui;
    }

    pub fn set_font_size(&mut self, font_size: f64) {
        let hidpi_factor = self.window.scale_factor();
        self.font_size = (hidpi_factor * font_size) as f32;
    }

    pub fn set_font(&mut self, path: &Path) {
        self.imgui_context.io_mut().font_global_scale = (
            1.0 / self.window.scale_factor()
        ) as f32;
        self.imgui_context.fonts().add_font(&[imgui::FontSource::TtfData { 
            data: &std::fs::read(path).expect("font path is incorrect"), 
            size_pixels: self.font_size, 
            config: None,
        }]);
    }
}
