use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Debri {
    remove: bool,
}

impl Debri {
    pub fn mark(&mut self) {
        self.remove = true
    }

    pub fn remove(&self) -> bool {
        self.remove
    }
}