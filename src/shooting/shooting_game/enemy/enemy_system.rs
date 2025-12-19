use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::shooting::shooting_game::{enemy::enemy_component::Enemy, projectile::projectile_message::ProjectileMessage, shooter::shooter_component::Shooter};

pub fn enemy_shot(
    time: Res<Time>,
    mut projectile_message: MessageWriter<ProjectileMessage>,
    mut shooters: Query<(Entity, &mut Shooter, &ChildOf)>,
    enemies: Query<(), With<Enemy>>,
) {
    let now = time.elapsed_secs();

    for (entity, mut shooter, child_of) in &mut shooters {
        if enemies.get(child_of.get()).is_err() {
            continue;
        };

        if shooter.can_fire(now) == false {
            continue;
        }

        projectile_message.write(
            ProjectileMessage {
                entity
            }
        );

        shooter.mark_fired(now);
    }
}