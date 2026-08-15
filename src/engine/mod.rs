// pub mod io;
// pub mod graphics;
pub mod renderer;
pub mod scene;
pub mod assets;
// pub mod input;
pub mod physics;
pub mod math;
// pub mod animation;
// pub mod audio;
// pub mod net;
// pub mod ui;

use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

use crate::engine::assets::server;

pub struct Engine {
    pub renderer: renderer::Renderer,
    pub asset_server: assets::server::Server,
    pub physics_world: physics::PhysicsWorld,
}

impl Engine {
    pub fn new(window: Arc<Window>) -> Self {
        let (graphics,) = pollster::block_on(async {
            let context = renderer::context::RenderContext::new(window.clone()).await;

            return (context,);
        });

        let renderer = renderer::Renderer::new(
            graphics,
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );
        let asset_server = server::Server::new(
            &renderer.get_render_context().device,
            &renderer.get_render_context().queue,
        );

        return Self {
            renderer,
            asset_server,
            physics_world: physics::PhysicsWorld::default(),
        };
    }
    
    pub fn resize(&mut self, physical_size: Option<PhysicalSize<u32>>) {
        let (width, height) = match physical_size {
            Some(size) => (size.width, size.height),
            None => (
                self.renderer.get_render_context().size.width, 
                self.renderer.get_render_context().size.height
            ),
        };

        self.renderer.resize(PhysicalSize{width, height});
        self.renderer.camera.transform.aspect = width as f32 / height as f32;
        self.renderer.update_surface();
    }
}
