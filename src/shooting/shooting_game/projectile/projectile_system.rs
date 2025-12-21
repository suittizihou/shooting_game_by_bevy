use bevy::prelude::*;

use crate::shooting::shooting_game::{debri::debri_message::DebriMessage, hit::hit_message::ProjectileHitEnemy, faction::faction_component::Faction, hit::hit_message::ProjectileHitPlayer, projectile::{projectile_bundle::ProjectileBundle, projectile_component::Projectile, projectile_message::ProjectileMessage, projectile_resource::ProjectileResources}, shooter::shooter_component::Shooter};

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

pub fn collision_to_player(
    mut messages: MessageReader<ProjectileHitPlayer>,
    projectiles: Query<&Projectile>,
    mut debri_message: MessageWriter<DebriMessage>,
) {
    for message in messages.read() {
        let Ok(projectile) = projectiles.get(message.projectile) else {
            continue;
        };
        if projectile.faction() == Faction::Enemy {
            debri_message.write(DebriMessage { entity: message.projectile });
        }
    }
}

pub fn collision_to_enemy(
    mut messages: MessageReader<ProjectileHitEnemy>,
    projectiles: Query<&Projectile>,
    mut debri_message: MessageWriter<DebriMessage>,
) {
    for message in messages.read() {
        let Ok(projectile) = projectiles.get(message.projectile) else {
            continue;
        };
        if projectile.faction() == Faction::Player {
            debri_message.write(DebriMessage { entity: message.projectile });
        }
    }
}