use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::shooting::shooting_game::{movement::movement_component::Movement2d, player::player_component::Player, projectile::{projectile_bundle::ProjectileBundle, projectile_resource::ProjectileAssets}, shooter::shooter_component::Shooter};

pub fn player_move(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Movement2d, With<Player>>) {
    for mut player in &mut query {
        let mut velocity = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            velocity.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            velocity.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            velocity.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            velocity.x += 1.0;
        }
        player.set_direction(velocity);
    }
}

pub fn player_shot(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    projectile_assets: Res<ProjectileAssets>,
    mut shooters: Query<(&GlobalTransform, &mut Shooter, &ChildOf)>,
    players: Query<(), With<Player>>,
) {
    if !input.pressed(KeyCode::Space) {
        return;
    }

    let now = time.elapsed_secs();

    for (g_transform, mut shooter, child_of) in &mut shooters {
        if players.get(child_of.get()).is_err() {
            continue;
        }

        if shooter.can_fire(now) == false {
            continue;
        }
        
        commands.spawn(
            ProjectileBundle::new(
                g_transform.translation(),
                shooter.get_damage(),
                &projectile_assets,
            ));

        shooter.mark_fired(now);
    }
}