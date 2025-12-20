use bevy::prelude::*;

use crate::shooting::{gameset::UpdateGameSet, shooting_game::{debri::debri_message::DebriMessage, lifetime::lifetime_component::Lifetime}};

pub struct LifetimePlugin;

fn lifetime_check(
    time: Res<Time>,
    mut debri_message: MessageWriter<DebriMessage>,
    mut query: Query<(Entity, &mut Lifetime)>
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.lifetime.tick(time.delta());

        if lifetime.lifetime.just_finished() {
            debri_message.write(DebriMessage{
                entity,
            });
        }
    }
}

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lifetime_check.in_set(UpdateGameSet::LateUpdate));
    }
}