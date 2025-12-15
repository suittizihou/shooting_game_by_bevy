use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Debri {
    pub destroy_time: Timer,
}

impl Debri {
    pub fn new(destroy_time: f32) -> Self {
        Self::default().with_destroy_time(destroy_time)
    }

    pub fn with_destroy_time(mut self, destroy_time_sec: f32) -> Self {
        self.destroy_time = Timer::from_seconds(destroy_time_sec, TimerMode::Once);
        self
    }
}