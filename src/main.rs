mod engine;

mod game;
mod util;
use util::config::Config;

use std::sync::Arc;

use env_logger::Env;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::{WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey::{self}}, window::{Window, WindowAttributes, WindowId},
};

use crate::game::Game;

struct App {
    window: Option<Arc<Window>>,
    game: Option<Game>,
}

impl App {
    pub fn new() -> Self {
        return Self {
            window: None,
            game: None,
        };
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
        self.game = Some(Game::init(window.clone()));
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
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
                if let Some(game) = &mut self.game {
                    game.engine.resize(Some(physical_size));
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
                    self.game.as_mut().unwrap().input_handler.keyboard(&key, &event.state);
                }
            },
            // WindowEvent::ModifiersChanged(modifiers) => todo!(),
            // WindowEvent::Ime(ime) => todo!(),
            // WindowEvent::CursorMoved { device_id, position } => todo!(),
            // WindowEvent::CursorEntered { device_id } => todo!(),
            // WindowEvent::CursorLeft { device_id } => todo!(),
            WindowEvent::MouseWheel { 
                device_id: _, 
                delta, 
                phase 
            } => self.game.as_mut().unwrap().input_handler.mouse_wheel(&delta, &phase), 
            WindowEvent::MouseInput { 
                device_id: _, 
                state, 
                button 
            } => self.game.as_mut().unwrap().input_handler.mouse_button(&state, &button),
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
                if let Some(game) = &mut self.game {
                    if let Err(e) = game.engine.renderer.render() {
                        eprintln!("render error: {e:?}");
                        game.engine.renderer.update_surface();
                        game.engine.resize(None);
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


