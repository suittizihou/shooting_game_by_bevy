use bevy::prelude::*;

use crate::shooting::shooting_game::{projectile::{projectile_bundle::ProjectileBundle, projectile_message::ProjectileMessage, projectile_resource::ProjectileResources}, shooter::shooter_component::Shooter};

pub fn spawn_projectile_from_event(
    mut commands: Commands,
    mut event: MessageReader<ProjectileMessage>,
    projectile_resources: Res<ProjectileResources>,
    query: Query<(&GlobalTransform, &Shooter)>,
) {
    for req in event.read() {
        if let Ok((transform, shooter)) = query.get(req.entity) {
            commands.spawn(
                ProjectileBundle::new(
                    shooter,
                    transform.translation(),
                    transform.up().xy(),
                    &projectile_resources,
                )
            );
        }
    }
}