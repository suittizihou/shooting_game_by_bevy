use crate::shooting::shooting_game::shooter::shooter_message::ShooterMessageCommon;
use bevy::prelude::*;

#[derive(Message)]
pub struct NormalShooterMessage {
    pub shooter_common: ShooterMessageCommon,
}
