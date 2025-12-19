use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::shooting::shooting_game::{debri::debri_component::Debri, movement::movement_bundle::Movement2dBundle, projectile::{projectile_component::Projectile, projectile_resource::ProjectileAssets}};

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub transform: Transform,
    pub collider: Collider,
    #[bundle()]
    pub movement: Movement2dBundle,
    pub projectile: Projectile,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub debri: Debri,
}

impl ProjectileBundle {
    pub fn new(position: Vec3, damage: u32, assets: &ProjectileAssets) -> Self {
        Self {
            transform: Transform::default().with_translation(position).with_scale(Vec3::splat(10.0)),
            collider: Collider::ball(0.5),
            movement: Movement2dBundle::new(Vec2::new(0.0, 1.0), 500.0),
            projectile: Projectile::new(damage),
            mesh: Mesh2d(assets.mesh.clone()),
            material: MeshMaterial2d(assets.material.clone()),
            debri: Debri::new(1.0)
        }
    }
}