#![allow(unused)]
mod engine;
mod game;

use engine::io::keyboard::{KEYBOARD, KeyboardLayer};

use std::error::Error;

use env_logger::Env;
use winit::{
    application::ApplicationHandler, event::{self, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{self, KeyCode, PhysicalKey::{self, Code}}, window::{Window, WindowAttributes, WindowId},
};
use env_logger;

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // generate ui

        // creating window
        self.window = Some(
            event_loop.create_window(WindowAttributes::default()).unwrap()
        );

    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {

        match event {
            // WindowEvent::ActivationTokenDone { serial, token } => todo!(),
            // WindowEvent::Resized(physical_size) => todo!(),
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
                device_id, 
                event, 
                is_synthetic 
            } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    KEYBOARD.lock().unwrap().handle_key(key);

                    if key == KeyCode::Escape {
                        event_loop.exit();
                    }
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

                self.window.as_ref().unwrap().request_redraw();
            },
            _ => {},
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initializes system peripherals
    env_logger::Builder::from_env(
        Env::default().default_filter_or("debug")
    ).init();
    let event_loop = EventLoop::new()?;

    {
        let mut keyboard = KEYBOARD.lock().unwrap();

        keyboard.push_focus(KeyboardLayer::Base);
        keyboard.subscribe(KeyCode::KeyW, Box::new(|| {
            println!("pressed w");
        }));
    }

    event_loop.set_control_flow(ControlFlow::Poll); // preferable for games
    event_loop.run_app(&mut App::default())?;

    return Ok(());
}


