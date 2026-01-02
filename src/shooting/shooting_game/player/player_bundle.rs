use bevy::prelude::*;

use crate::shooting::shooting_game::collider::ball_bundle::BallSensorBundle;
use crate::shooting::shooting_game::{
    debri::debri_component::Debri,
    faction::faction_component::Faction,
    hp::hp_component::Hp,
    move_entity::move_entity_bundle::MoveEntityBundle,
    player::{player_component::Player, player_resource::PlayerResources},
};
use crate::shooting::shooting_game::shooter::pattern_shooter::normal_shooter::normal_shooter_bundle::NormalShooterBundle;
use crate::shooting::shooting_game::shooter::ShooterCore;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle()]
    pub move_entity_bundle: MoveEntityBundle,
    pub hp: Hp,
    pub debri: Debri,
    #[bundle()]
    pub collider: BallSensorBundle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl PlayerBundle {
    fn new(position: Vec3, move_speed: f32, hp: u32, assets: &PlayerResources) -> Self {
        Self {
            player: Player,
            move_entity_bundle: MoveEntityBundle::new(position, 0.0, 30.0, move_speed, None),
            hp: Hp::default().with_hp(hp),
            debri: Debri::default(),
            collider: BallSensorBundle::new(0.5),
            mesh: Mesh2d(assets.mesh.clone()),
            material: MeshMaterial2d(assets.material.clone()),
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        position: Vec3,
        move_speed: f32,
        hp: u32,
        damage: u32,
        assets: &PlayerResources,
    ) -> Entity {
        commands
            .spawn(Self::new(position, move_speed, hp, assets))
            .with_children(|parent| {
                parent.spawn(NormalShooterBundle::new(&ShooterCore::new(
                    Transform::from_xyz(0.0, 0.5, 0.0),
                    damage,
                    0.1,
                    Faction::Player,
                )));
            })
            .id()
    }
}
