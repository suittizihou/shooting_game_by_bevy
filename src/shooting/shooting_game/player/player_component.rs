use bevy::{ecs::relationship::Relationship, prelude::*};
use bevy_rapier2d::prelude::Collider;

use crate::shooting::shooting_game::{movement::movement_component::{Movement2d, Movement2dBundle}, projectile::projectile_component::{ProjectileAssets, ProjectileBundle}, shooter::shooter_component::{Shooter, ShooterBundle}};

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

#[derive(Resource)]
pub struct PlayerAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub transform: Transform,
    pub collider: Collider,
    #[bundle()]
    pub movement: Movement2dBundle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl PlayerBundle {
    fn new(
        position: Vec3,
        move_speed: f32,
        assets: &PlayerAssets,
    ) -> Self {
        Self {
            player: Player,
            transform: Transform::default().with_translation(position).with_scale(Vec3::splat(30.0)),
            collider: Collider::ball(0.5),
            movement: Movement2dBundle::new(Vec2::ZERO, move_speed),
            mesh: Mesh2d(assets.mesh.clone()),
            material: MeshMaterial2d(assets.material.clone()),
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        position: Vec3,
        move_speed: f32,
        damage: u32,
        assets: &PlayerAssets,
    ) -> Entity {
        commands.spawn(Self::new(
            position,
            move_speed,
            assets,
        ))
        .with_children(|parent| { 
                parent.spawn(
                    ShooterBundle::new(
                        Transform::from_xyz(0.0, 0.5, 0.0),
                        damage,
                        0.1,
                    )); 
            })
        .id()
    }
}