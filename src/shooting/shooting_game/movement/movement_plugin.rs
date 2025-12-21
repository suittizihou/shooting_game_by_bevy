use bevy::prelude::*;

use crate::shooting::{gameset::UpdateGameSet, shooting_game::movement::movement_system::*};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system.in_set(UpdateGameSet::Update));
    }
}
