use bevy::prelude::*;

use crate::shooting::shooting_game::take_damage::take_damage_message::TakeDamageMessage;

pub struct TakeDamagePlugin;

impl Plugin for TakeDamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TakeDamageMessage>();
    }
}
