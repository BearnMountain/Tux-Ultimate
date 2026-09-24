use std::collections::HashMap;

use glam::Vec2;
use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::KeyCode,
};

use crate::util::config::Config;

pub struct MouseButtons {
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}

pub struct GameInput {
    // true = pressed, false = released
    // pub action_state: [bool; GameActions::TOTAL_ACTIONS],
    // bindings: HashMap<KeyCode, GameActions>,
    //
    // // mouse input
    pub mouse_buttons: MouseButtons,
    pub mouse_position: Vec2,
    pub mouse_scroll_delta: Vec2,
    pub mouse_delta: Vec2,
}

impl GameInput {
    pub fn new() -> Self {


        return Self {
            mouse_buttons: MouseButtons {
                left: false,
                middle: false,
                right: false,
            },
            mouse_position: Vec2::new(0.0, 0.0),
            mouse_scroll_delta: Vec2::new(0.0, 0.0),
            mouse_delta: Vec2::new(0.0, 0.0),
        };
    }

    pub fn mouse_wheel(&mut self, delta: &MouseScrollDelta, _phase: &TouchPhase) {
        // match phase {
        //     TouchPhase::Started => {
        //         if self.mouse_scroll_delta < 0.0 {
        //             self.action_state[GameActions::CAMERA_ZOOM_IN as usize] = true
        //         } else {
        //             self.action_state[GameActions::CAMERA_ZOOM_OUT as usize] = true
        //         }
        //     },
        //     TouchPhase::Moved => {
        //         if self.mouse_scroll.1 < 0.0 {
        //             self.action_state[GameActions::CAMERA_ZOOM_IN as usize] = true
        //         } else {
        //             self.action_state[GameActions::CAMERA_ZOOM_OUT as usize] = true
        //         }
        //     },
        //     TouchPhase::Ended => {
        //         self.mouse_scroll = (0.0, 0.0);
        //         self.action_state[GameActions::CAMERA_ZOOM_IN as usize] = false;
        //         self.action_state[GameActions::CAMERA_ZOOM_OUT as usize] = false;
        //     },
        //     TouchPhase::Cancelled => self.action_state[GameActions::CAMERA_ZOOM_IN as usize] = false,
        // }

        let delta = match delta {
            MouseScrollDelta::LineDelta(x, y) => Vec2::new(*x, *y),
            MouseScrollDelta::PixelDelta(pos) => Vec2::new(pos.x as f32, pos.y as f32),
        };

        self.mouse_scroll_delta += delta;
    }

    pub fn mouse_button(&mut self, state: &ElementState, button: &MouseButton) {
        let pressed = matches!(state, ElementState::Pressed);
        match button {
            MouseButton::Left => self.mouse_buttons.left = pressed,
            MouseButton::Right => self.mouse_buttons.right = pressed,
            MouseButton::Middle => self.mouse_buttons.middle = pressed,
            MouseButton::Back => log::debug!("mouse input back not implemented"),
            MouseButton::Forward => log::debug!("mouse input forward not implemented"),
            MouseButton::Other(_) => log::debug!("mouse input other not implemented"),
        };
    }

    pub fn mouse_movement(&mut self, x: f32, y: f32) {
        self.mouse_delta += Vec2::new(x, y);
    }

    pub fn keyboard(&mut self, key: &KeyCode, state: &ElementState) {

    }
}
