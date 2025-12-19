use bevy::prelude::*;

use crate::shooting::shooting_game::{movement::movement_bundle::Movement2dBundle, player::{player_component::Player, player_resource::PlayerAssets}, shooter::shooter_component::ShooterBundle};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle()]
    pub move_entity_bundle: MoveEntityBundle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl PlayerBundle {
    fn new(
        position: Vec3,
        move_speed: f32,
        assets: &PlayerResources,
    ) -> Self {
        Self {
            player: Player,
            move_entity_bundle: MoveEntityBundle::new(
                position,
                0.0,
                30.0,
                move_speed,
                None,
            ),
            mesh: Mesh2d(assets.mesh.clone()),
            material: MeshMaterial2d(assets.material.clone()),
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        position: Vec3,
        move_speed: f32,
        damage: u32,
        assets: &PlayerResources,
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