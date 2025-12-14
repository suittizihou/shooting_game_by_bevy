use bevy::{color::palettes::css::YELLOW, prelude::*};

use crate::shooting::{gameset::StartupGameSet, shooting_game::projectile::projectile_component::ProjectileAssets};

pub struct ProjectilePlugin;

fn setup_projectile_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ProjectileAssets {
        mesh: meshes.add(Circle::default()),
        material: materials.add(Color::from(YELLOW)),
    });
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_projectile_assets.in_set(StartupGameSet::PostInitialize));
    }
}