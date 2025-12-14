use bevy::prelude::*;

#[derive(Component)]
pub struct Shooter {
    damage: u32,
    interval: f32,
    pub next_fire_time: f32,
}

impl Default for Shooter {
    fn default() -> Self {
        Self{ 
            damage: Default::default(),
            interval: Default::default(),
            next_fire_time: Default::default(), 
        }
    }
}

impl Shooter {
    pub fn new(damage: u32, interval: f32) -> Self {
        Self::default().with_damage(damage).with_interval(interval)
    }

    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage = damage;
        self
    }

    pub fn with_interval(mut self, interval: f32) -> Self {
        self.interval = interval;
        self
    }

    pub const fn get_damage(&self) -> u32 {
        self.damage
    }

    pub const fn get_interval(&self) -> f32 {
        self.interval
    }
}