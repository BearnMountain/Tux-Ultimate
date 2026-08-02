use std::collections::HashMap;

use winit::{event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase}, keyboard::KeyCode};

use crate::util::config::{Config};


/// all game binds mapped to actions
#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Copy, Clone)]
pub enum GameActions {
    // camera
    CAMERA_UP,
    CAMERA_DOWN,
    CAMERA_RIGHT,
    CAMERA_LEFT,
    CAMERA_ZOOM_IN,
    CAMERA_ZOOM_OUT,
    CAMERA_ROTATE_UP,
    CAMERA_ROTATE_DOWN,
    CAMERA_ROTATE_RIGHT,
    CAMERA_ROTATE_LEFT,

    // player
    PLAYER_LEFT,
    PLAYER_RIGHT,
    PLAYER_UP,
    PLAYER_DOWN,



    PLACEHOLDER, // just for total number of game actions
}

impl GameActions {
    pub const TOTAL_ACTIONS: usize = GameActions::PLACEHOLDER as usize + 1;
}

pub struct Input {
    // true = pressed, false = released
    pub keys_down: [bool; GameActions::TOTAL_ACTIONS],
    bindings: HashMap<KeyCode, GameActions>,

    // mouse input
    pub mouse_button: (bool, bool, bool), // left, middle, right
    // pub mouse_position: (f64, f64), // xy
    pub mouse_scroll: (f64, f64), 

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


        return Self {
            keys_down: [false; GameActions::TOTAL_ACTIONS],
            bindings,
            // mouse_position: (0.0, 0.0),
            mouse_scroll: (0.0, 0.0),
            mouse_button: (false, false, false),
        }
    }

    pub fn mouse_wheel(&mut self, delta: &MouseScrollDelta, phase: &TouchPhase) {
        let _ = phase;
        let (x, y) = match delta {
            MouseScrollDelta::LineDelta(x, y) => (*x as f64, *y as f64),
            MouseScrollDelta::PixelDelta(pos) => (pos.x, pos.y),
        };

        println!("{x} {y}");

        self.mouse_scroll = (x, y);
    }

    pub fn mouse_button(&mut self, state: &ElementState, button: &MouseButton) {
        match button {
            MouseButton::Left => {
                if matches!(state, ElementState::Pressed) {
                    self.mouse_button.0 = true;
                } else {
                    self.mouse_button.0 = false;
                }
            },
            MouseButton::Right => {
                if matches!(state, ElementState::Pressed) {
                    self.mouse_button.2 = true;
                } else {
                    self.mouse_button.2 = false;
                }
            },
            MouseButton::Middle => {
                if matches!(state, ElementState::Pressed) {
                    self.mouse_button.1 = true;
                } else {
                    self.mouse_button.1 = false;
                }
            },
            MouseButton::Back => {
                log::debug!("mouse input back not implemented");
            },
            MouseButton::Forward => {
                log::debug!("mouse input forward not implemented");
            },
            MouseButton::Other(_) => {
                log::debug!("mouse input other not implemented");
            },
        };
    }

    pub fn keyboard(&mut self, key: &KeyCode, state: &ElementState) {
        if let Some(action) = self.bindings.get(key) {
            if matches!(state, ElementState::Pressed) {
                self.keys_down[*action as usize] = true;
            } else {
                self.keys_down[*action as usize] = false;
            }
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
