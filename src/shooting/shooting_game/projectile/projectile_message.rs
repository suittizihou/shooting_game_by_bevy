use bevy::prelude::*;

#[derive(Message, Copy, Clone)]
pub struct ProjectileMessage {
    pub entity: Entity,
}