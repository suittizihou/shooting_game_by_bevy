use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyResources {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}