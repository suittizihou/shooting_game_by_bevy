use bevy::prelude::*;

use crate::shooting::shooting_game::debri::debri_message::DebriMessage;

pub struct DebriPlugin;

fn garbage_collection(
    mut commands: Commands,
    mut debri_message: MessageReader<DebriMessage>,
) {
    for message in debri_message.read() {
        let entity = message.entity;
        commands.entity(entity).despawn_related::<Children>();
        commands.entity(entity).despawn();
    }
}

impl Plugin for DebriPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DebriMessage>();
        app.add_systems(Last, garbage_collection);
    }
}