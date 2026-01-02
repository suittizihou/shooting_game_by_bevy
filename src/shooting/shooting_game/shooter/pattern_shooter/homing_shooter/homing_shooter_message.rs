use crate::shooting::shooting_game::shooter::shooter_message::ShooterMessageCommon;
use bevy::prelude::{Entity, Message};

#[derive(Message)]
pub struct HomingShooterMessage {
    pub shooter_common: ShooterMessageCommon,
    pub target_entity: Entity,
}
