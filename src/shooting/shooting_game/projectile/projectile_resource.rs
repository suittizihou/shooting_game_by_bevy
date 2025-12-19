use bevy::prelude::*;

#[derive(Resource)]
pub struct ProjectileAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}