use std::{cmp::Ordering, marker::PhantomData};

#[derive(Clone, Copy)]
pub struct Handle<T> {
    pub id: usize,
    _marker: PhantomData<T>,
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Handle<T> {}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> Handle<T> {
    pub fn new(id: usize) -> Self {
        return Self {
            id,
            _marker: PhantomData,
        };
    }
}
