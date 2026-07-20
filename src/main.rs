mod engine;
use engine::renderer::pipeline;
use engine::renderer::mesh_builder;

mod game;
mod util;
use util::config::Config;

use engine::io::keyboard::{KEYBOARD, KeyboardLayer};
use std::sync::Arc;

use anyhow::Result;

use env_logger::Env;
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::{self, ElementState, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{self, KeyCode, PhysicalKey::{self, Code}}, window::{Window, WindowAttributes, WindowId},
};

use wgpu;

use crate::engine::renderer::{self, bind_group_layout};

struct State {
    window: Arc<Window>,
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,

    // loading graphics
    render_pipeline: wgpu::RenderPipeline,
    quad_mesh: mesh_builder::Mesh,
}

impl State {
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

    pub fn render(&mut self) -> Result<()>{
        self.window.request_redraw();
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout |
            wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
            wgpu::CurrentSurfaceTexture::Outdated |
            wgpu::CurrentSurfaceTexture::Lost => {
                self.resize(self.size.width, self.size.height);
                return Ok(());
            },
            wgpu::CurrentSurfaceTexture::Validation => {
                panic!("Surface validation failed");
            },
        };

        let image_view = surface_texture.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );

        // queue for all draw calls
        let mut command_encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        // clear color
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.25,
                    g: 0.0,
                    b: 0.5,
                    a: 0.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("Renderpass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        };

        // submit draw commands and present to window surface texture
        {
            let mut pass = command_encoder
                .begin_render_pass(&render_pass_descriptor);

            // draw each pipeline
            pass.set_pipeline(&self.render_pipeline);
            pass.set_vertex_buffer(0, self.quad_mesh.vertex_buffer.slice(..));
            pass.set_index_buffer(self.quad_mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..6, 0, 0..1);
        }
        self.queue.submit(std::iter::once(command_encoder.finish()));
        self.queue.present(surface_texture);

        return Ok(());
    }
    
    pub fn update_surface(&mut self) {
        self.surface = self.instance.create_surface(self.window.clone()).unwrap();
    }
}

struct App {
    state: Option<State>,
    window: Option<Arc<Window>>,
}

impl App {
    pub fn new() -> Self {
        return Self {
            state: None,
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
        self.state = Some(pollster::block_on(State::new(window.clone())));
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
    }

    // fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    //     if let Some(window) = &self.window {
    //         window.request_redraw();
    //     }
    // }

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
                if let Some(state) = self.state.as_mut() {
                    state.size = physical_size;
                    state.update_surface();
                    state.resize(state.size.width, state.size.height);
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

                    KEYBOARD.lock().unwrap().handle_key(key, event.state);
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
                if let Some(state) = self.state.as_mut() {
                    // if state.size.width != state.config.width || state.size.height != state.config.height {
                    //     state.resize(state.size.width, state.size.height);
                    // }
                    if let Err(e) = state.render() {
                        eprintln!("render error: {e:?}");
                        state.update_surface();
                        state.resize(state.size.width, state.size.height);
                    }
                }
                // self.window.as_ref().unwrap().request_redraw();
            },
            _ => {},
        }
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // initializes system peripherals
    env_logger::Builder::from_env(
        Env::default().default_filter_or("warn")
    ).init();
    Config::init("assets/config.toml");

    let event_loop = EventLoop::new()?;

    // setting default keybinds
    {
        let mut keyboard = KEYBOARD.lock().unwrap();

        keyboard.push_focus(KeyboardLayer::Base);
        keyboard.subscribe(KeyCode::KeyW, Box::new(|state| {
            if state == ElementState::Pressed {
                println!("pressed w");
            } else {
                println!("released w");
            }
        }));
    }

    event_loop.set_control_flow(ControlFlow::Poll); // preferable for games

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    return Ok(());
}

// texture class
// engine has textures
// shader uses textures
fn main() {
    let _ = pollster::block_on(run());
}


