use bevy::prelude::*;

#[derive(Message, Copy, Clone)]
pub struct TakeDamageMessage {
    pub entity: Entity,
    pub damage: u32,
}