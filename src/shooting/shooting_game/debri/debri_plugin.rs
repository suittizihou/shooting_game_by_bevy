use bevy::prelude::*;

use crate::shooting::{gameset::UpdateGameSet, shooting_game::debri::debri_component::Debri};

pub struct DebriPlugin;

fn garbage_collection(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Debri)>
) {
    for (entity, mut debri) in &mut query {
        debri.destroy_time.tick(time.delta());

        if debri.destroy_time.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

impl Plugin for DebriPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, garbage_collection.in_set(UpdateGameSet::LateUpdate));
    }
}