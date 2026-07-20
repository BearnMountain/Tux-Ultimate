use std::{collections::HashMap};
use crate::engine::assets::handle::Handle;

pub struct Storage<T> {
    assets: HashMap<usize, T>,
    next_id: usize,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        return Self {
            assets: HashMap::new(),
            next_id: 0,
        };
    }

    pub fn add(&mut self, assets: T) -> Handle<T> {
        let id = self.next_id;
        self.next_id += 1;

        self.assets.insert(id, assets);

        return Handle::new(id);
    }

    pub fn remove(&mut self, handle: &Handle<T>) -> bool {
        match self.assets.remove(&handle.id) {
            Some(thing) => {let _ = thing; return true;},
            None => false,
        };
        return false;
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        return self.assets.get(&handle.id);
    }
}
