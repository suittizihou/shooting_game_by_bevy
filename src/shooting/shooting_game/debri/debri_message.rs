use bevy::prelude::*;

#[derive(Message, Copy, Clone)]
pub struct DebriMessage {
    pub entity: Entity,
}
