use std::sync::atomic::Ordering;
use bevy::prelude::Component;
use crate::object::{COUNTER, ObjectId};

#[derive(Component, Copy, Clone)]
pub struct Object {
    pub id: ObjectId
}

impl Object {
    pub fn new() -> Self {
        Object {
            id: COUNTER.fetch_add(1, Ordering::Relaxed)
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::new()
    }
}
