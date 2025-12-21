use bevy::prelude::*;

use crate::shooting::shooting_game::debri::{debri_component::Debri, debri_message::DebriMessage};

pub fn mark_debri(
    mut debri_messages: MessageReader<DebriMessage>,
    mut debris: Query<&mut Debri>,
) {
    for message in debri_messages.read() {
        if let Ok(mut debri) = debris.get_mut(message.entity) {
            debri.mark();
        }
    }
}

pub fn garbage_collection(
    mut commands: Commands,
    debris: Query<(Entity, &Debri)>,
) {
    for (entity, debri) in &debris {
        if debri.remove() {
            commands.entity(entity).despawn_related::<Children>();
            commands.entity(entity).despawn();
        }
    }
}