use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerResources {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}
