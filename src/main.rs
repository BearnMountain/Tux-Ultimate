mod engine;

mod game;
mod util;
use util::config::Config;

use std::{sync::Arc, time::{Duration, Instant}};

use env_logger::Env;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::{WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey::{self}}, window::{Window, WindowAttributes, WindowId},
};

use crate::game::Game;

// max time that a frame isnt updated
const MAXIMUM_ACCUMULATOR: Duration = Duration::from_millis(100);

struct App {
    window: Option<Arc<Window>>,
    game: Option<Game>,

    last_time: Instant,
    accumulator: Duration,
    tick: u64,
    dt: Duration,
}

impl App {
    pub fn new() -> Self {
        let tick_rate = Config::get().read().unwrap().app.tick_rate as u64;
        let dt = Duration::from_nanos(1_000_000_000 / tick_rate.max(1));
        return Self {
            window: None,
            game: None,
            last_time: Instant::now(),
            accumulator: Duration::ZERO,
            tick: 0,
            dt,
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

        // reset frame timer
        self.last_time = Instant::now();
        self.accumulator = Duration::ZERO;

        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(game) = &mut self.game else { return };

        match event {
            // WindowEvent::ActivationTokenDone { serial, token } => todo!(),
            WindowEvent::Resized(physical_size) => {
                game.engine.resize(Some(physical_size));
            },
            // WindowEvent::Moved(physical_position) => todo!(),
            WindowEvent::CloseRequested => {
                // save files etc in the future
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
                    game.input_handler.keyboard(&key, &event.state);
                }
            },
            // WindowEvent::ModifiersChanged(modifiers) => todo!(),
            // WindowEvent::Ime(ime) => todo!(),
            WindowEvent::CursorMoved { 
                device_id: _, 
                position 
            } => game.input_handler.mouse_movement(position),
            // WindowEvent::CursorEntered { device_id } => todo!(),
            // WindowEvent::CursorLeft { device_id } => todo!(),
            WindowEvent::MouseWheel { 
                device_id: _, 
                delta, 
                phase 
            } => game.input_handler.mouse_wheel(&delta, &phase), 
            WindowEvent::MouseInput { 
                device_id: _, 
                state, 
                button 
            } => game.input_handler.mouse_button(&state, &button),
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
                // compute render interpolation
                let alpha = self.accumulator.as_secs_f32() / self.dt.as_secs_f32();
                game.engine.renderer.interpolation_alpha = alpha.clamp(0.0, 1.0);

                // draw
                if let Err(e) = game.engine.renderer.render() {
                    eprintln!("render error: {e:?}");
                    game.engine.renderer.update_surface();
                    game.engine.resize(None);
                }
                // self.window.as_ref().unwrap().request_redraw();
            },
            _ => {},
        }
    }

    // sets tick intervals
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(game) = &mut self.game else { return };

        let now = Instant::now();
        let mut frame_time = now - self.last_time;
        self.last_time = now;

        if frame_time > MAXIMUM_ACCUMULATOR {
            frame_time = MAXIMUM_ACCUMULATOR;
        }
        self.accumulator += frame_time;

        // general polling
        game.update(frame_time);
        
        // renders frame 1/tick freqency
        let max_ticks_per_frame = 5;
        let mut ticks = 0;
        while self.accumulator >= self.dt && ticks < max_ticks_per_frame {
            game.engine.renderer.update_transform_snapshots();
            game.frame(self.dt, self.tick);

            self.accumulator -= self.dt;
            self.tick += 1;
            ticks += 1;
        }

        if ticks >= max_ticks_per_frame {
            self.accumulator = Duration::ZERO;
        }

        if let Some(window) = &self.window {
            window.request_redraw();
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


