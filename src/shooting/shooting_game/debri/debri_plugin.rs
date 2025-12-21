use bevy::prelude::*;

use crate::shooting::shooting_game::debri::{debri_message::DebriMessage, debri_system::{garbage_collection, mark_debri}};

pub struct DebriPlugin;

impl Plugin for DebriPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DebriMessage>();
        app.add_systems(Last, (
            mark_debri,
            garbage_collection,
        ).chain());
    }
}