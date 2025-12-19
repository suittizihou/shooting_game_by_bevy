use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::shooting::shooting_game::movement::movement_component::Movement2d;

#[derive(Bundle)]
pub struct Movement2dBundle {
    pub movement: Movement2d,
    pub velocity: Velocity,
}

impl Movement2dBundle {
    pub fn new(direction: Vec2, speed: f32) -> Self {
        Self {
            movement: Movement2d::new(direction, speed),
            velocity: Velocity::zero(),
        }
    }
}