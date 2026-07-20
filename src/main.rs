mod engine;

mod game;
mod util;
use util::config::Config;

use engine::io::keyboard::KeyboardLayer;
use std::sync::Arc;

use anyhow::Result;

use env_logger::Env;
use winit::{
    application::ApplicationHandler, 
    event::{ElementState, WindowEvent}, 
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey::{self}}, 
    window::{Window, WindowAttributes, WindowId},
};

use crate::engine::Engine;
use crate::engine::io::keyboard::Keyboard;

    // loading graphics
    // render_pipeline: wgpu::RenderPipeline,
    // quad_mesh: mesh_builder::Mesh,

    // pub async fn new(window: Arc<Window>) -> Self {
    //     // vertex buffer for triangles
    //     let quad_mesh = mesh_builder::make_quad(&device);
    //     let material_bind_group_layout: wgpu::BindGroupLayout;
    //     {
    //         let mut builder = bind_group_layout::Builder::new(&device);
    //         builder.add_material();
    //         material_bind_group_layout = builder.build("MaterialBindGroupLayout");
    //
    //     }
    //
    //     let render_pipeline: wgpu::RenderPipeline;
    //     {
    //         // creates pipeline
    //         let mut builder = pipeline::Builder::new(&device);
    //         builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
    //         builder.set_pixel_format(config.format);
    //         builder.add_buffer_layout(Some(mesh_builder::Vertex::get_layout()));
    //         builder.add_bind_group_layout(&material_bind_group_layout);
    //         render_pipeline = builder.build_pipeline("RenderPipeline");
    //     }
    // }


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
        // generate ui

        // creating window
        let window = Arc::new(
            event_loop.create_window(WindowAttributes::default()).unwrap()
        );
        self.engine = Some(Engine::new(window.clone()));
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();

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

                    Keyboard::with_mut(|keyboard| {
                        keyboard.handle_key(key, event.state);
                    });
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

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    // setting default keybinds
    Keyboard::with_mut(|keyboard| {
        keyboard.push_focus(KeyboardLayer::Base);
        keyboard.subscribe(KeyCode::KeyW, Box::new(|state| {
            if state == ElementState::Pressed {
                println!("pressed w");
            } else {
                println!("released w");
            }
        }));
        keyboard.pop_focus();
    });

    event_loop.set_control_flow(ControlFlow::Poll); // preferable for games

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    return Ok(());
}

// texture class
// engine has textures
// shader uses textures
fn main() {
    // init system info
    env_logger::Builder::from_env(
        Env::default().default_filter_or("warn")
    ).init();
    Config::init("assets/config.toml");
    Keyboard::init_main_thread();

    let _ = pollster::block_on(run());
}


