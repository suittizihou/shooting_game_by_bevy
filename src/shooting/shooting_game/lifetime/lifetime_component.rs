use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Lifetime {
    pub lifetime: Timer,
}

impl Lifetime {
    pub fn new(lifetime: f32) -> Self {
        Self::default().with_lifetime(lifetime)
    }

    pub fn with_lifetime(mut self, lifetime_sec: f32) -> Self {
        self.lifetime = Timer::from_seconds(lifetime_sec, TimerMode::Once);
        self
    }
}
