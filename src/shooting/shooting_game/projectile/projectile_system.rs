use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::shooting::shooting_game::{debri::debri_message::DebriMessage, enemy::enemy_component::Enemy, faction::faction_component::Faction, player::player_component::Player, projectile::{projectile_bundle::ProjectileBundle, projectile_component::Projectile, projectile_message::ProjectileMessage, projectile_resource::ProjectileResources}, shooter::shooter_component::Shooter};

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

pub fn collision_projectile_event(
    mut collisions: MessageReader<CollisionEvent>,
    players: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    projectiles: Query<(Entity, &Projectile)>,
    mut debri_message: MessageWriter<DebriMessage>,
) {
    for col_message in collisions.read() {
        let (entity1, entity2) = match col_message {
            CollisionEvent::Started(a, b, _) => (*a, *b),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let Ok((entity, projectile)) = 
        projectiles.get(entity1).or(projectiles.get(entity2)) else {
            continue;
        };

        if projectile.faction() == Faction::Enemy {
            if players.get(entity1).is_ok() || players.get(entity2).is_ok() {
                debri_message.write(DebriMessage { entity });
                continue;
            }
        }
        else {
            if enemies.get(entity1).is_ok() || enemies.get(entity2).is_ok() {
                debri_message.write(DebriMessage { entity });
                continue;
            }
        }
    }
}