use bevy::prelude::*;

use crate::shooting::{gameset::UpdateGameSet, shooting_game::lifetime::lifetime_system::lifetime_check};

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lifetime_check.in_set(UpdateGameSet::LateUpdate));
    }
}