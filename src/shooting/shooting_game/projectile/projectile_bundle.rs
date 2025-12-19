use bevy::prelude::*;

use crate::shooting::shooting_game::{debri::debri_component::Debri, move_entity::move_entity_bundle::MoveEntityBundle, projectile::{projectile_component::Projectile, projectile_resource::ProjectileResources}};

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    #[bundle()]
    pub move_entity_bundle: MoveEntityBundle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub debri: Debri,
}

impl ProjectileBundle {
    pub fn new(position: Vec3, damage: u32, move_dir: Vec2, assets: &ProjectileResources) -> Self {
        Self {
            move_entity_bundle: MoveEntityBundle::new(
                position,
                0.0,
                10.0,
                500.0,
                Some(move_dir),
            ),
            projectile: Projectile::new(damage),
            mesh: Mesh2d(assets.mesh.clone()),
            material: MeshMaterial2d(assets.material.clone()),
            debri: Debri::new(1.0)
        }
    }
}