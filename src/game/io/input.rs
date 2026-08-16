use std::collections::HashMap;

use glam::Vec2;
use winit::{event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase}, keyboard::KeyCode};

use crate::util::config::{Config};


/// all game binds mapped to actions
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum GameActions {
    // camera
    CAMERA_UP,
    CAMERA_DOWN,
    CAMERA_RIGHT,
    CAMERA_LEFT,
    CAMERA_FORWARD,
    CAMERA_BACKWARD,
    CAMERA_ROTATE_UP,
    CAMERA_ROTATE_DOWN,
    CAMERA_ROTATE_RIGHT,
    CAMERA_ROTATE_LEFT,

    // player
    PLAYER_LEFT,
    PLAYER_RIGHT,
    PLAYER_UP,
    PLAYER_DOWN,
    PLAYER_ROTATE,

    PLACEHOLDER, // just for total number of game actions
}

impl GameActions {
    pub const TOTAL_ACTIONS: usize = GameActions::PLACEHOLDER as usize + 1;
}

pub struct MouseButtons {
    pub left: bool,
    pub middle: bool,
    pub right: bool,
}

pub struct Input {
    // true = pressed, false = released
    pub action_state: [bool; GameActions::TOTAL_ACTIONS],
    bindings: HashMap<KeyCode, GameActions>,

    // mouse input
    pub mouse_buttons: MouseButtons,
    pub mouse_position: Vec2,
    pub mouse_scroll_delta: Vec2, 
    pub mouse_delta: Vec2, 
}

impl Input {
    pub fn new() -> Self {
        let config = &Config::get().read().unwrap().keybinds;
        let mut bindings: HashMap<KeyCode, GameActions> = HashMap::new();

        let mut bind = |key: &str, action: GameActions, name: &str| {
            bindings
                .insert(
                    Input::str_to_key(key)
                        .unwrap_or_else(|| panic!("{name} keybind name is incorrect")),
                    action,
                )
                .is_none()
                .then_some(())
                .expect("duplicate keybind detected");
        };

        // setting up all keybinds
        bind(&config.player_left,  GameActions::PLAYER_LEFT,  "player left");
        bind(&config.player_right, GameActions::PLAYER_RIGHT, "player right");
        bind(&config.player_up,    GameActions::PLAYER_UP,    "player up");
        bind(&config.player_down,  GameActions::PLAYER_DOWN,  "player down");
        { // for debugging purposes
            bindings.insert(KeyCode::ArrowUp, GameActions::CAMERA_FORWARD);
            bindings.insert(KeyCode::ArrowDown, GameActions::CAMERA_BACKWARD);
            bindings.insert(KeyCode::ArrowRight, GameActions::CAMERA_RIGHT);
            bindings.insert(KeyCode::ArrowLeft, GameActions::CAMERA_LEFT);
            bindings.insert(KeyCode::Space, GameActions::CAMERA_UP);
            bindings.insert(KeyCode::ShiftLeft, GameActions::CAMERA_DOWN);
            bindings.insert(KeyCode::KeyR, GameActions::PLAYER_ROTATE);
            // bindings.insert(KeyCode::, GameActions::CAMERA_ROTATE_DOWN);
            // bindings.insert(KeyCode::, GameActions::CAMERA_ROTATE_RIGHT);
            // bindings.insert(KeyCode::, GameActions::CAMERA_ROTATE_LEFT);
        }



        return Self {
            action_state: [false; GameActions::TOTAL_ACTIONS],
            bindings,
            mouse_buttons: MouseButtons{ 
                left: false, 
                middle: false, 
                right: false, 
            },
            mouse_position: Vec2::new(0.0, 0.0),
            mouse_scroll_delta: Vec2::new(0.0, 0.0),
            mouse_delta: Vec2::new(0.0, 0.0),
        }
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
        if let Some(action) = self.bindings.get(key) {
            self.action_state[*action as usize] = matches!(state, ElementState::Pressed);
        }
    }

    fn str_to_key(name: &str) -> Option<KeyCode> {
        match name {
            "Backquote" => Some(KeyCode::Backquote),
            "Backslash" => Some(KeyCode::Backslash),
            "BracketLeft" => Some(KeyCode::BracketLeft),
            "BracketRight" => Some(KeyCode::BracketRight),
            "Comma" => Some(KeyCode::Comma),
            "Digit0" => Some(KeyCode::Digit0),
            "Digit1" => Some(KeyCode::Digit1),
            "Digit2" => Some(KeyCode::Digit2),
            "Digit3" => Some(KeyCode::Digit3),
            "Digit4" => Some(KeyCode::Digit4),
            "Digit5" => Some(KeyCode::Digit5),
            "Digit6" => Some(KeyCode::Digit6),
            "Digit7" => Some(KeyCode::Digit7),
            "Digit8" => Some(KeyCode::Digit8),
            "Digit9" => Some(KeyCode::Digit9),
            "Equal" => Some(KeyCode::Equal),
            "IntlBackslash" => Some(KeyCode::IntlBackslash),
            "IntlRo" => Some(KeyCode::IntlRo),
            "IntlYen" => Some(KeyCode::IntlYen),
            "KeyA" => Some(KeyCode::KeyA),
            "KeyB" => Some(KeyCode::KeyB),
            "KeyC" => Some(KeyCode::KeyC),
            "KeyD" => Some(KeyCode::KeyD),
            "KeyE" => Some(KeyCode::KeyE),
            "KeyF" => Some(KeyCode::KeyF),
            "KeyG" => Some(KeyCode::KeyG),
            "KeyH" => Some(KeyCode::KeyH),
            "KeyI" => Some(KeyCode::KeyI),
            "KeyJ" => Some(KeyCode::KeyJ),
            "KeyK" => Some(KeyCode::KeyK),
            "KeyL" => Some(KeyCode::KeyL),
            "KeyM" => Some(KeyCode::KeyM),
            "KeyN" => Some(KeyCode::KeyN),
            "KeyO" => Some(KeyCode::KeyO),
            "KeyP" => Some(KeyCode::KeyP),
            "KeyQ" => Some(KeyCode::KeyQ),
            "KeyR" => Some(KeyCode::KeyR),
            "KeyS" => Some(KeyCode::KeyS),
            "KeyT" => Some(KeyCode::KeyT),
            "KeyU" => Some(KeyCode::KeyU),
            "KeyV" => Some(KeyCode::KeyV),
            "KeyW" => Some(KeyCode::KeyW),
            "KeyX" => Some(KeyCode::KeyX),
            "KeyY" => Some(KeyCode::KeyY),
            "KeyZ" => Some(KeyCode::KeyZ),
            "Minus" => Some(KeyCode::Minus),
            "Period" => Some(KeyCode::Period),
            "Quote" => Some(KeyCode::Quote),
            "Semicolon" => Some(KeyCode::Semicolon),
            "Slash" => Some(KeyCode::Slash),
            "AltLeft" => Some(KeyCode::AltLeft),
            "AltRight" => Some(KeyCode::AltRight),
            "Backspace" => Some(KeyCode::Backspace),
            "CapsLock" => Some(KeyCode::CapsLock),
            "ContextMenu" => Some(KeyCode::ContextMenu),
            "ControlLeft" => Some(KeyCode::ControlLeft),
            "ControlRight" => Some(KeyCode::ControlRight),
            "Enter" => Some(KeyCode::Enter),
            "SuperLeft" => Some(KeyCode::SuperLeft),
            "SuperRight" => Some(KeyCode::SuperRight),
            "ShiftLeft" => Some(KeyCode::ShiftLeft),
            "ShiftRight" => Some(KeyCode::ShiftRight),
            "Space" => Some(KeyCode::Space),
            "Tab" => Some(KeyCode::Tab),
            "Convert" => Some(KeyCode::Convert),
            "KanaMode" => Some(KeyCode::KanaMode),
            "Lang1" => Some(KeyCode::Lang1),
            "Lang2" => Some(KeyCode::Lang2),
            "Lang3" => Some(KeyCode::Lang3),
            "Lang4" => Some(KeyCode::Lang4),
            "Lang5" => Some(KeyCode::Lang5),
            "NonConvert" => Some(KeyCode::NonConvert),
            "Delete" => Some(KeyCode::Delete),
            "End" => Some(KeyCode::End),
            "Help" => Some(KeyCode::Help),
            "Home" => Some(KeyCode::Home),
            "Insert" => Some(KeyCode::Insert),
            "PageDown" => Some(KeyCode::PageDown),
            "PageUp" => Some(KeyCode::PageUp),
            "ArrowDown" => Some(KeyCode::ArrowDown),
            "ArrowLeft" => Some(KeyCode::ArrowLeft),
            "ArrowRight" => Some(KeyCode::ArrowRight),
            "ArrowUp" => Some(KeyCode::ArrowUp),
            "NumLock" => Some(KeyCode::NumLock),
            "Numpad0" => Some(KeyCode::Numpad0),
            "Numpad1" => Some(KeyCode::Numpad1),
            "Numpad2" => Some(KeyCode::Numpad2),
            "Numpad3" => Some(KeyCode::Numpad3),
            "Numpad4" => Some(KeyCode::Numpad4),
            "Numpad5" => Some(KeyCode::Numpad5),
            "Numpad6" => Some(KeyCode::Numpad6),
            "Numpad7" => Some(KeyCode::Numpad7),
            "Numpad8" => Some(KeyCode::Numpad8),
            "Numpad9" => Some(KeyCode::Numpad9),
            "NumpadAdd" => Some(KeyCode::NumpadAdd),
            "NumpadBackspace" => Some(KeyCode::NumpadBackspace),
            "NumpadClear" => Some(KeyCode::NumpadClear),
            "NumpadClearEntry" => Some(KeyCode::NumpadClearEntry),
            "NumpadComma" => Some(KeyCode::NumpadComma),
            "NumpadDecimal" => Some(KeyCode::NumpadDecimal),
            "NumpadDivide" => Some(KeyCode::NumpadDivide),
            "NumpadEnter" => Some(KeyCode::NumpadEnter),
            "NumpadEqual" => Some(KeyCode::NumpadEqual),
            "NumpadHash" => Some(KeyCode::NumpadHash),
            "NumpadMemoryAdd" => Some(KeyCode::NumpadMemoryAdd),
            "NumpadMemoryClear" => Some(KeyCode::NumpadMemoryClear),
            "NumpadMemoryRecall" => Some(KeyCode::NumpadMemoryRecall),
            "NumpadMemoryStore" => Some(KeyCode::NumpadMemoryStore),
            "NumpadMemorySubtract" => Some(KeyCode::NumpadMemorySubtract),
            "NumpadMultiply" => Some(KeyCode::NumpadMultiply),
            "NumpadParenLeft" => Some(KeyCode::NumpadParenLeft),
            "NumpadParenRight" => Some(KeyCode::NumpadParenRight),
            "NumpadStar" => Some(KeyCode::NumpadStar),
            "NumpadSubtract" => Some(KeyCode::NumpadSubtract),
            "Escape" => Some(KeyCode::Escape),
            "Fn" => Some(KeyCode::Fn),
            "FnLock" => Some(KeyCode::FnLock),
            "PrintScreen" => Some(KeyCode::PrintScreen),
            "ScrollLock" => Some(KeyCode::ScrollLock),
            "Pause" => Some(KeyCode::Pause),
            "BrowserBack" => Some(KeyCode::BrowserBack),
            "BrowserFavorites" => Some(KeyCode::BrowserFavorites),
            "BrowserForward" => Some(KeyCode::BrowserForward),
            "BrowserHome" => Some(KeyCode::BrowserHome),
            "BrowserRefresh" => Some(KeyCode::BrowserRefresh),
            "BrowserSearch" => Some(KeyCode::BrowserSearch),
            "BrowserStop" => Some(KeyCode::BrowserStop),
            "Eject" => Some(KeyCode::Eject),
            "LaunchApp1" => Some(KeyCode::LaunchApp1),
            "LaunchApp2" => Some(KeyCode::LaunchApp2),
            "LaunchMail" => Some(KeyCode::LaunchMail),
            "MediaPlayPause" => Some(KeyCode::MediaPlayPause),
            "MediaSelect" => Some(KeyCode::MediaSelect),
            "MediaStop" => Some(KeyCode::MediaStop),
            "MediaTrackNext" => Some(KeyCode::MediaTrackNext),
            "MediaTrackPrevious" => Some(KeyCode::MediaTrackPrevious),
            "Power" => Some(KeyCode::Power),
            "Sleep" => Some(KeyCode::Sleep),
            "AudioVolumeDown" => Some(KeyCode::AudioVolumeDown),
            "AudioVolumeMute" => Some(KeyCode::AudioVolumeMute),
            "AudioVolumeUp" => Some(KeyCode::AudioVolumeUp),
            "WakeUp" => Some(KeyCode::WakeUp),
            "Meta" => Some(KeyCode::Meta),
            "Hyper" => Some(KeyCode::Hyper),
            "Turbo" => Some(KeyCode::Turbo),
            "Abort" => Some(KeyCode::Abort),
            "Resume" => Some(KeyCode::Resume),
            "Suspend" => Some(KeyCode::Suspend),
            "Again" => Some(KeyCode::Again),
            "Copy" => Some(KeyCode::Copy),
            "Cut" => Some(KeyCode::Cut),
            "Find" => Some(KeyCode::Find),
            "Open" => Some(KeyCode::Open),
            "Paste" => Some(KeyCode::Paste),
            "Props" => Some(KeyCode::Props),
            "Select" => Some(KeyCode::Select),
            "Undo" => Some(KeyCode::Undo),
            _ => None,
        }
    }
}
