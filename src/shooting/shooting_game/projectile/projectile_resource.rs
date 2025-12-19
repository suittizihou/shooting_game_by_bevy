use bevy::prelude::*;

#[derive(Resource)]
pub struct ProjectileResources {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}