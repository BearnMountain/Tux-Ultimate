use winit::event::KeyEvent;
use winit::keyboard::{KeyCode, PhysicalKey};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{LazyLock, Mutex};

type Callback = Box<dyn FnMut() + Send>;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum KeyboardLayer {
    Base,
    Settings,
    Invalid, // ensures that coder always uses 'push_focus' instead of leaving ambigious
}

struct Layer {
    binds: HashMap<KeyCode, Callback>,
}

pub struct Keyboard {
    layers: [Layer; KeyboardLayer::Invalid as usize],
    focus_stack: Vec<KeyboardLayer>,
}

impl Keyboard {
    pub fn new() -> Self {
        return Self {
            layers: std::array::from_fn(|_| Layer {
                binds: HashMap::new(),
            }),
            focus_stack: Vec::new(),
        };
    }

    pub fn current_focus(&self) -> KeyboardLayer {
        return *self
            .focus_stack
            .last()
            .expect("no focus pushed - call push_focus first");
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        let focus = self.current_focus();
        if let Some(keyboard) = self.layers[focus as usize].binds.get_mut(&key) {
            keyboard();
        }
    }

    pub fn subscribe(&mut self, code: KeyCode, callback: Callback) -> bool {
        let focus = self.current_focus();
        let binds = &mut self.layers[focus as usize].binds;

        if binds.contains_key(&code) {
            log::debug!("already have keycode bound");
            return false;
        }

        binds.insert(code, callback);
        return true;
    }

    pub fn unsubscribe(&mut self, code: KeyCode) -> bool {
        let focus = self.current_focus();
        let binds = &mut self.layers[focus as usize].binds;

        if binds.remove(&code).is_none() {
            log::debug!("key bind doesn't exist");
            return false;
        }
        return true;
    }

    pub fn push_focus(&mut self, focus: KeyboardLayer) {
        debug_assert!(focus != KeyboardLayer::Invalid, "cannot push_focus to Invalid");
        self.focus_stack.push(focus);
    }

    pub fn pop_focus(&mut self) {
        debug_assert!(
            self.focus_stack.pop().is_some(),
            "pop_focus called with empty focus_stack"
        );
    }

}

pub static KEYBOARD: LazyLock<Mutex<Keyboard>> = 
    LazyLock::new(|| Mutex::new(Keyboard::new()));
