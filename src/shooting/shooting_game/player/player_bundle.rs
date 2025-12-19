use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shooting::shooting_game::{movement::movement_bundle::Movement2dBundle, player::{player_component::Player, player_resource::PlayerAssets}, shooter::shooter_component::ShooterBundle};

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