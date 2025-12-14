use bevy::prelude::*;

use crate::shooting::shooting_game::{debri::debri_component::Debri, movement::movement_component::Movement2d};

#[derive(Component)]
pub struct Projectile {
    damage: u32,
}

impl Default for Projectile {
    fn default() -> Self {
        Self { damage: u32::default() }
    }
}

impl Projectile {    
    pub fn new(damage: u32) -> Self {
        Self::default().with_damage(damage)
    }

    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage = damage;
        self
    }
}

#[derive(Resource)]
pub struct ProjectileAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub transform: Transform,
    pub movement: Movement2d,
    pub projectile: Projectile,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub debri: Debri,
}

impl ProjectileBundle {
    pub fn new(position: Vec3, damage: u32, assets: &ProjectileAssets) -> Self {
        Self {
            transform: Transform::default().with_translation(position).with_scale(Vec3::splat(10.0)),
            movement: Movement2d::default().with_direction(Vec2::new(0.0, 1.0)).with_speed(500.0),
            projectile: Projectile::new(damage),
            mesh: Mesh2d(assets.mesh.clone()),
            material: MeshMaterial2d(assets.material.clone()),
            debri: Debri::new(1.0)
        }
    }
}