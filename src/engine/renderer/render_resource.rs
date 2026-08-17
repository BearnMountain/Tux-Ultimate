use std::collections::VecDeque;

use super::{
    material::Material,
    transform::Transform,
    mesh::Mesh,
};

use crate::util::handle::Handle;

pub struct RenderResources {
    // pub render_pass: Handle<...>,
    pub pipeline: Handle<wgpu::RenderPipeline>,
    pub material: Handle<Material>,
    pub transform: Handle<Transform>,
    pub mesh: Handle<Mesh>,
}

pub struct RenderStorage<T> {
    assets: Vec<Option<T>>,
    free: VecDeque<usize>,
}

impl<T> RenderStorage<T> {
    pub fn new() -> Self {
        return Self {
            assets: Vec::new(),
            free: VecDeque::new(),
        };
    }

    pub fn add(&mut self, assets: T) -> Handle<T> {
        if self.free.len() > 0 {
            self.assets[self.free.pop_front().unwrap()] = Some(assets);
        } else {
            self.assets.push(Some(assets));
        }
        return Handle::new(self.assets.len() - 1);
    }

    pub fn remove(&mut self, handle: &Handle<T>) -> Option<T> {
        self.free.push_back(handle.id);
        return self.assets[handle.id].take();
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        return self.assets[handle.id].as_ref();
    }
}
