#![allow(unused)]
mod engine;
use engine::renderer::pipeline_builder::PipelineBuilder;
use engine::renderer::mesh_builder;

mod game;

use engine::io::keyboard::{KEYBOARD, KeyboardLayer};
use std::sync::Arc;

use std::error::Error;
use anyhow::Result;

use env_logger::Env;
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::{self, ElementState, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{self, KeyCode, PhysicalKey::{self, Code}}, window::{Window, WindowAttributes, WindowId},
};

use wgpu::{Instance, InstanceDescriptor, RequestAdapterError, RequestAdapterOptionsBase};

use crate::engine::renderer::mesh_builder::make_triangle;



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
    triangle_mesh: wgpu::Buffer,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // descriptor to handle gpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });
        
        // creates winit surface for graphics
        let surface = instance.create_surface(window.clone()).unwrap();

        // select physical gpu
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await.unwrap();

        // logical device and command queue for graphics
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            }
        ).await.unwrap();

        // format for the surface
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats.iter()
            .copied().filter(|f| f.is_srgb())
            .next().unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            color_space: wgpu::SurfaceColorSpace::default(),
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // vertex buffer for triangles
        let triangle_mesh = mesh_builder::make_triangle(&device);

        // creates pipeline
        let mut pipeline_builder = PipelineBuilder::new();
        pipeline_builder.add_buffer_layout(Some(mesh_builder::Vertex::get_layout()));
        pipeline_builder.set_shader_module("shaders/shader.wgsl", "vs_main", "fs_main");
        pipeline_builder.set_pixel_format(config.format);
        let render_pipeline = pipeline_builder.build_pipeline(&device);

        return Self {
            window: window,
            instance: instance,
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            size: size,
            render_pipeline: render_pipeline,
            triangle_mesh: triangle_mesh,
        };
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            let max = 2048;
            self.config.width = width.min(max);
            self.config.height = height.min(max);
            self.surface.configure(&self.device, &self.config);
        }
    }

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
            pass.set_vertex_buffer(0, self.triangle_mesh.slice(..));
            pass.draw(0..3, 0..1);
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

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {

        match event {
            // WindowEvent::ActivationTokenDone { serial, token } => todo!(),
            WindowEvent::Resized(physical_size) => {
                if let Some(state) = self.state.as_mut() {
                    state.size = physical_size;
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

fn main() {
    pollster::block_on(run());
}


