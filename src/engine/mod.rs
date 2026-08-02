// pub mod io;
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
use winit::{dpi::PhysicalSize, window::Window};

pub struct Engine {
    pub renderer: renderer::Renderer,
    pub asset_server: assets::server::Server,

    // io handling
    pub camera: scene::camera::Camera,
    
    // keyboard: io::keyboard::Keyboard,
}

impl Engine {
    pub fn new(window: Arc<Window>) -> Self {
        let (graphics,) = pollster::block_on(async {
            let context = renderer::context::RenderContext::new(window.clone()).await;

            return (context,);
        });

        let camera = scene::camera::Camera::new(
            &graphics.device, 
            &graphics.queue, 
            glam::Vec3::new(0.0, 0.0, 0.0), 
            window.inner_size().width as f32 / window.inner_size().height as f32,
        );

        let renderer = renderer::Renderer::new(graphics);
        let asset_server = assets::server::Server::new(
            renderer.get_render_context().device.clone(),
            renderer.get_render_context().queue.clone(),
        );

        return Self {
            renderer,
            asset_server,
            camera,
        };
    }
    
    pub fn resize(&mut self, physical_size: Option<PhysicalSize<u32>>) {
        self.renderer.resize(physical_size);
        self.camera.transform.aspect = 
            physical_size.unwrap().width as f32 /
            physical_size.unwrap().height as f32;
    }


}
