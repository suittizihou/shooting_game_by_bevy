use bevy::prelude::*;

use crate::shooting::shooting_game::{movement::movement_component::Movement2d, projectile::projectile_component::{ProjectileAssets, ProjectileBundle}, shooter::shooter_component::Shooter};

#[derive(Component)]
pub struct Player;

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
    mut query: Query<(&Transform, &mut Shooter), With<Player>>
) {
    if !input.pressed(KeyCode::Space) {
        return;
    }

    let now = time.elapsed_secs();

    for (transform, mut shooter) in &mut query {
        if shooter.can_fire(now) == false {
            continue;
        }
        commands.spawn(
            ProjectileBundle::new(
                transform.translation,
                shooter.get_damage(),
                &projectile_assets,
            ));

        shooter.mark_fired(now);
    }
}

#[derive(Resource)]
pub struct PlayerAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub transform: Transform,
    pub movement: Movement2d,
    pub shooter: Shooter,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl PlayerBundle {
    pub fn new(
        position: Vec3,
        move_speed: f32,
        damage: u32,
        assets: &PlayerAssets,
    ) -> Self {
        Self {
            player: Player,
            transform: Transform::default().with_translation(position).with_scale(Vec3::splat(30.0)),
            movement: Movement2d::new(Vec2::ZERO, move_speed),
            shooter: Shooter::new(damage, 0.1),
            mesh: Mesh2d(assets.mesh.clone()),
            material: MeshMaterial2d(assets.material.clone()),
        }
    }
}