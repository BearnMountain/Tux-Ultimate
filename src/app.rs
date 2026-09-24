use std::{sync::Arc, time::{Duration, Instant}};

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, ControlFlow}, window::Window};

use crate::{game::Game, util::config::Config};

const MAXIMUM_ACCUMULATOR: Duration = Duration::from_millis(100);
const MAX_TICKS_PER_FRAME: u32 = 5;

/// treated as an adapter for winit, takes window events to 
/// run the "game"
///
/// everything init here and output to game/mod.rs
pub struct App {
    window: Option<Arc<Window>>,
    game: Option<Game>,

    last_time: Instant,
    accumulator: Duration,
    dt: Duration,
}

impl App {
    pub fn new() -> Self {
        let tick_rate = Config::get().read().unwrap().app.tick_rate as u64;

        return Self {
            window: None,
            game: None,
            last_time: Instant::now(),
            accumulator: Duration::ZERO,
            dt: Duration::from_nanos(
                1_000_000_000 / tick_rate.max(1)
            ),
        };
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let global_config = Config::get().read().unwrap();

        // creating window
        let attrs = winit::window::WindowAttributes::default()
            .with_title("Tux Ultimate")
            .with_inner_size(winit::dpi::LogicalSize::new(
                global_config.window.width,
                global_config.window.height,
            ));

        let window = Arc::new(event_loop.create_window(attrs).unwrap());

        // cursor
        window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .expect("cant take control of cursor");
        window.set_cursor_visible(false);

        let mut game = Game::init(window.clone());

        // game.setup_game();

        self.game = Some(game);
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();

        // set frame timer
        self.last_time = Instant::now();
        self.accumulator = Duration::ZERO;

        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(game) = &mut self.game else { return };

        match event {
            // WindowEvent::ActivationTokenDone { serial, token } => todo!(),
            WindowEvent::Resized(physical_size) => {
                game.engine.resize(Some(physical_size));
            }
            // WindowEvent::Moved(physical_position) => todo!(),
            WindowEvent::CloseRequested => {
                // save files etc in the future
                event_loop.exit();
            }
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
                if let winit::keyboard::PhysicalKey::Code(key) = event.physical_key {
                    if key == winit::keyboard::KeyCode::KeyQ {
                        event_loop.exit();
                    }
                    game.input_handler.keyboard(&key, &event.state);
                }
            },
            // WindowEvent::ModifiersChanged(modifiers) => todo!(),
            // WindowEvent::Ime(ime) => todo!(),
            // WindowEvent::CursorMoved {
            //     device_id: _,
            //     position
            // } => game.input_handler.mouse_movement(position),
            // WindowEvent::CursorEntered { device_id } => todo!(),
            // WindowEvent::CursorLeft { device_id } => todo!(),
            // WindowEvent::MouseWheel {
            //     device_id: _,
            //     delta,
            //     phase
            // } => game.input_handler.mouse_wheel(&delta, &phase),
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
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
                // let alpha = self.accumulator.as_secs_f32() / self.dt.as_secs_f32();
                // game.engine.renderer.interpolation_alpha = alpha.clamp(0.0, 1.0);

                // draw at monitor refresh rate
                if let Err(e) = game.frame() {
                    eprintln!("render error: {e:?}");
                    game.engine.renderer.update_surface();
                    game.engine.resize(None);
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let Some(game) = &mut self.game else { return };

        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                game.input_handler
                    .mouse_movement(delta.0 as f32, delta.1 as f32);
            }
            winit::event::DeviceEvent::MouseWheel { delta } => {
                game.input_handler.mouse_wheel(
                    &delta, 
                    &winit::event::TouchPhase::Started
                );
            }
            
            // usually ignore this because WindowEvent::KeyboardInput is easier
            // DeviceEvent::Key(raw_key_event) => {},

            // joystick/raw axis events
            // DeviceEvent::Motion { axis, value } => {},

            _ => {}
        }
    }

    // sets tick intervals
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(game) = &mut self.game else { return };

        let now = Instant::now();
        let mut frame_time = now - self.last_time;
        self.last_time = now;

        frame_time = frame_time.min(MAXIMUM_ACCUMULATOR);
        self.accumulator += frame_time;

        // renders frame 1/tick freqency
        let mut ticks = 0;
        while self.accumulator >= self.dt && ticks < MAX_TICKS_PER_FRAME {
            game.update();

            self.accumulator -= self.dt;
            ticks += 1;
        }

        if ticks == MAX_TICKS_PER_FRAME {
            self.accumulator = Duration::ZERO;
        }

        self.window.as_ref().unwrap().request_redraw();
    }
}
