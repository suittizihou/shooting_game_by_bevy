use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::shooting::shooting_game::{enemy::enemy_component::Enemy, projectile::{projectile_bundle::ProjectileBundle, projectile_resource::ProjectileResources}, shooter::shooter_component::Shooter};

pub fn enemy_shot(
    mut commands: Commands,
    time: Res<Time>,
    projectile_resources: Res<ProjectileResources>,
    mut shooters: Query<(&GlobalTransform, &mut Shooter, &ChildOf)>,
    enemies: Query<(), With<Enemy>>,
) {
    let now = time.elapsed_secs();

    for (transform, mut shooter, child_of) in &mut shooters {
        if enemies.get(child_of.get()).is_err() {
            continue;
        }

        if shooter.can_fire(now) == false {
            continue;
        }

        commands.spawn(
            ProjectileBundle::new(
                &shooter,
                transform.translation(),
                transform.up().xy(),
                &projectile_resources,
            )
        );

        shooter.mark_fired(now);
    }
}