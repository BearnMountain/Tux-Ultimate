use winit::event::{ElementState};
use winit::keyboard::{KeyCode};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{OnceLock};
use std::thread::{self, ThreadId};

type Callback = Box<dyn FnMut(ElementState)>;

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
    pub fn init_main_thread() {
        MAIN_THREAD_ID
            .set(thread::current().id())
            .expect("init_main_thread called twice");
    }

    pub fn new() -> Self {
        let mut focus_stack = Vec::new();
        focus_stack.push(KeyboardLayer::Base);
        return Self {
            layers: std::array::from_fn(|_| Layer {
                binds: HashMap::new(),
            }),
            focus_stack,
        };
    }

    pub fn with_mut<R>(f: impl FnOnce(&mut Keyboard) -> R) -> R {
        let owner = MAIN_THREAD_ID
            .get()
            .expect("Keyboard::init_main_thread() was never called");
        assert_eq!(
            thread::current().id(),
            *owner,
            "Keyboard accessed from a non-main thread"
        );
        KEYBOARD.with(|k| f(&mut k.borrow_mut()))
    }

    pub fn current_focus(&self) -> KeyboardLayer {
        return *self
            .focus_stack
            .last()
            .expect("no focus pushed - call push_focus first");
    }

    pub fn handle_key(&mut self, key: KeyCode, state: ElementState) {
        let focus = self.current_focus();
        if let Some(keyboard) = self.layers[focus as usize].binds.get_mut(&key) {
            keyboard(state);
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

static MAIN_THREAD_ID: OnceLock<ThreadId> = OnceLock::new();

thread_local! {
    static KEYBOARD: RefCell<Keyboard> = RefCell::new(Keyboard::new());
}
