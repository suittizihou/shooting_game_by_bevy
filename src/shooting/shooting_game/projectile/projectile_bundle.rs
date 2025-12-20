use bevy::prelude::*;

use crate::shooting::shooting_game::{lifetime::lifetime_component::Lifetime, move_entity::move_entity_bundle::MoveEntityBundle, projectile::{projectile_component::Projectile, projectile_resource::ProjectileResources}, shooter::shooter_component::Shooter};

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    #[bundle()]
    pub move_entity_bundle: MoveEntityBundle,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub lifetime: Lifetime,
}

impl ProjectileBundle {
    pub fn new(shooter: &Shooter, position: Vec3, move_dir: Vec2, assets: &ProjectileResources) -> Self {
        Self {
            move_entity_bundle: MoveEntityBundle::new(
                position,
                0.0,
                10.0,
                500.0,
                Some(move_dir),
            ),
            projectile: Projectile::new(shooter.get_damage(), shooter.get_faction()),
            mesh: Mesh2d(assets.mesh.clone()),
            material: MeshMaterial2d(assets.material.clone()),
            lifetime: Lifetime::new(1.0)
        }
    }
}