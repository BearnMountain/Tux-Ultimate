mod engine;

mod game;
mod util;
use util::config::Config;

use std::{path::Path, sync::Arc};

use env_logger::Env;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey::{self}}, window::{Window, WindowAttributes, WindowId},
};

use crate::engine::{
    Engine, assets::{server}, renderer::{self, bind_group, mesh, pipeline}};

struct App {
    engine: Option<Engine>,
    window: Option<Arc<Window>>,
}

impl App {
    pub fn new() -> Self {
        return Self {
            engine: None,
            window: None,
        }
    }
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let global_config = Config::get().read().unwrap();

        // creating window
        let attrs = WindowAttributes::default()
            .with_title("Tux Ultimate")
            .with_inner_size(LogicalSize::new(
                global_config.window.width, 
                global_config.window.height
            ));

        let window = Arc::new(
            event_loop.create_window(attrs).unwrap()
        );
        self.engine = Some(Engine::new(window.clone()));
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();

        // testing engine
        let engine = self.engine.as_mut().unwrap();

        // gather test data
        let (shader_text, texture_raw) = pollster::block_on(async {
            tokio::try_join!(
                server::Server::preload_text(Path::new("shaders/shader.wgsl")),
                server::Server::preload_raw(Path::new("textures/brick-texture-54.png")),
            )
        }).expect("rip");

        let shader_handle = match engine.asset_server.load_shader(shader_text, None, None) {
            Some(handle) => handle,
            None => { 
                log::error!("failed to load shader source text");
                return;
            },
        };
        let texture_handle = match engine.asset_server.load_texture(texture_raw) {
            Some(handle) => handle,
            None => { 
                log::error!("failed to load texture source raw");
                return;
            },
        };
        
        let mut binding = bind_group::LayoutBuilder::new(
            &engine.renderer.get_render_context().device
        );
        let bind_group_layout_builder = binding
            .add_texture_view(
                wgpu::ShaderStages::FRAGMENT, 
                wgpu::TextureSampleType::Float { filterable: true }, 
                wgpu::TextureViewDimension::D2
            )
            .add_texture_sampler(
                wgpu::ShaderStages::FRAGMENT, 
                wgpu::SamplerBindingType::Filtering
            )
            .build("material bind group test");
        
        let pipeline = {
            let contex = engine.renderer.get_render_context();
            pipeline::Builder::new(&contex.device)
                .set_shader(engine.asset_server.get_shader(shader_handle).unwrap())
                .set_pixel_format(contex.config.format)
                .add_buffer_layout(Some(mesh::Vertex::get_layout()))
                .add_bind_group_layout(&bind_group_layout_builder.layout.clone().unwrap())
                .build_pipeline("pipeline test")
        };
        
        let material = {
            let contex = engine.renderer.get_render_context();
            renderer::material::Material::new(
                "test material", 
                engine.asset_server.get_texture(texture_handle).unwrap(), 
                &contex.device, 
                &bind_group_layout_builder
            )
        };
        
        // get stuff renderable each loop
        let material_id = engine.renderer.add_material(material);
        let pipeline_id = engine.renderer.add_pipeline(pipeline);
        {
            let mesh0 = renderer::mesh::Mesh::make_quad(
                &engine.renderer.get_render_context().device, 
                material_id,
                pipeline_id,
                [100.0, 100.0], // pos
                [100.0, 100.0], // area
            );
            let _mesh_id = engine.renderer.add_mesh(mesh0);

            let mesh1 = renderer::mesh::Mesh::make_quad(
                &engine.renderer.get_render_context().device, 
                material_id,
                pipeline_id,
                [100.0, 300.0], 
                [100.0, 100.0],
            );
            let _mesh_id = engine.renderer.add_mesh(mesh1);
        }
        

        // set default keybinds
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let _ = window_id;

        match event {
            // WindowEvent::ActivationTokenDone { serial, token } => todo!(),
            WindowEvent::Resized(physical_size) => {
                if let Some(engine) = &mut self.engine {
                    engine.renderer.resize(Some(physical_size));
                    engine.renderer.update_surface();
                }
            },
            // WindowEvent::Moved(physical_position) => todo!(),
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            // WindowEvent::Destroyed => todo!(),
            // WindowEvent::DroppedFile(path_buf) => todo!(),
            // WindowEvent::HoveredFile(path_buf) => todo!(),
            // WindowEvent::HoveredFileCancelled => todo!(),
            // WindowEvent::Focused(_) => todo!(),
            WindowEvent::KeyboardInput { 
                device_id: _, 
                event, 
                is_synthetic: _,
            } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    if key == KeyCode::Escape {
                        event_loop.exit();
                    }

                    // Keyboard::with_mut(|keyboard| {
                    //     keyboard.handle_key(key, event.state);
                    // });
                }
            },
            // WindowEvent::ModifiersChanged(modifiers) => todo!(),
            // WindowEvent::Ime(ime) => todo!(),
            // WindowEvent::CursorMoved { device_id, position } => todo!(),
            // WindowEvent::CursorEntered { device_id } => todo!(),
            // WindowEvent::CursorLeft { device_id } => todo!(),
            // WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
            // WindowEvent::MouseInput { device_id, state, button } => todo!(),
            // WindowEvent::PinchGesture { device_id, delta, phase } => todo!(),
            // WindowEvent::PanGesture { device_id, delta, phase } => todo!(),
            // WindowEvent::DoubleTapGesture { device_id } => todo!(),
            // WindowEvent::RotationGesture { device_id, delta, phase } => todo!(),
            // WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
            // WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
            // WindowEvent::Touch(touch) => todo!(),
            // WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            // WindowEvent::ThemeChanged(theme) => todo!(),
            // WindowEvent::Occluded(_) => todo!(),
            WindowEvent::RedrawRequested => {
                // draw
                if let Some(engine) = &mut self.engine {
                    if let Err(e) = engine.renderer.render() {
                        eprintln!("render error: {e:?}");
                        engine.renderer.update_surface();
                        engine.renderer.resize(None);
                    }
                }
                // self.window.as_ref().unwrap().request_redraw();
            },
            _ => {},
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init system info
    env_logger::Builder::from_env(
        Env::default()
            .default_filter_or("warn,app=debug")
    ).init();    
    Config::init("assets/config.toml");

    let rt = tokio::runtime::Runtime::new().expect("failed to start app");
    let _guard = rt.enter();

    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Poll); // preferable for games

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    return Ok(());
}


