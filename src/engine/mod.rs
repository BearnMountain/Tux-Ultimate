// pub mod io;
// pub mod graphics;
pub mod renderer;
// pub mod scene;
pub mod assets;
// pub mod input;
// pub mod physics;
// pub mod math;
// pub mod animation;
// pub mod audio;
// pub mod net;
// pub mod ui;

use std::sync::Arc;
use tokio::runtime::Handle;
use winit::window::Window;

pub struct Engine {
    pub renderer: renderer::Renderer,
    pub asset_server: assets::server::Server,

    // io handling
    // keyboard: io::keyboard::Keyboard,
}

impl Engine {
    pub fn new(window: Arc<Window>) -> Self {
        let (graphics,) = pollster::block_on(async {
            let context = renderer::context::RenderContext::new(window).await;

            return (context,);
        });

        let renderer = renderer::Renderer::new(graphics);
        let asset_server = assets::server::Server::new(
            renderer.get_render_context().device.clone(),
            renderer.get_render_context().queue.clone(),
        );

        return Self {
            renderer,
            asset_server,
        };
    }
}
