// generic is never used, phantom removes error
use std::marker::PhantomData;

pub struct Handle<T> {
    pub id: usize,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: usize) -> Self {
        return Self {
            id,
            _marker: PhantomData,
        };
    }
}

