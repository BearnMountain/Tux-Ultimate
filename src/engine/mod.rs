pub mod io;
// pub mod graphics;
pub mod renderer;
pub mod scene;
pub mod assets;
// pub mod input;
// pub mod physics;
// pub mod math;
// pub mod animation;
// pub mod audio;
// pub mod net;
// pub mod ui;

use std::sync::Arc;
use winit::window::Window;


pub struct Engine {
    pub renderer: renderer::Renderer,

    // io handling
    // keyboard: io::keyboard::Keyboard,
}

impl Engine {
    pub fn new(window: Arc<Window>) -> Self {
        let graphics = pollster::block_on(async {
            let graphics = renderer::context::RenderContext::new(window).await;

            return graphics;
        });

        let renderer = renderer::Renderer::new(graphics);

        return Self {
            renderer,
        };
    }
}
