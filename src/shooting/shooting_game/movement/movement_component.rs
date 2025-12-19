use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

#[derive(Component, Default)]
#[require(Velocity)]
pub struct Movement2d {
    direction: Vec2,
    speed: f32,
}

impl Movement2d {
    pub fn new(direction: Vec2, speed: f32) -> Self {
        Self::default().with_direction(direction).with_speed(speed)
    }

    pub fn with_direction(mut self, direction: Vec2) -> Self {
        self.direction = direction.normalize_or_zero();
        self
    }

    pub const fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn set_direction(&mut self, direction: Vec2) {
        self.direction = direction.normalize_or_zero()
    }

    pub fn velocity(&self, delta_time: f32) -> Vec3 {
        let velocity = self.direction * self.speed * delta_time;
        Vec3::new(velocity.x, velocity.y, 0.0)
    }
}