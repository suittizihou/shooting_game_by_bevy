use bevy::prelude::*;

use crate::shooting::shooting_game::collider::ball_bundle::BallSensorBundle;
use crate::shooting::shooting_game::{
    debri::debri_component::Debri,
    enemy::{enemy_component::Enemy, enemy_resource::EnemyResources},
    faction::faction_component::Faction,
    hp::hp_component::Hp,
    move_entity::move_entity_bundle::MoveEntityBundle,
    shooter::shooter_component::ShooterBundle,
};

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub move_entity_bundle: MoveEntityBundle,
    pub hp: Hp,
    pub debri: Debri,
    #[bundle()]
    pub collider: BallSensorBundle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl EnemyBundle {
    fn new(position: Vec3, move_speed: f32, hp: u32, assets: &EnemyResources) -> Self {
        Self {
            enemy: Enemy,
            move_entity_bundle: MoveEntityBundle::new(
                position,
                180.0,
                30.0,
                move_speed,
                Some(Vec2::new(0.0, -1.0)),
            ),
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
        assets: &EnemyResources,
    ) -> Entity {
        commands
            .spawn(Self::new(position, move_speed, hp, assets))
            .with_children(|parent| {
                parent.spawn(ShooterBundle::new(
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    damage,
                    1.0,
                    Faction::Enemy,
                ));
            })
            .id()
    }
}
