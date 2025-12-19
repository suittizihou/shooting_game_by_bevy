use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}