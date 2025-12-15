use bevy::{color::palettes::css::PURPLE, math::VectorSpace, prelude::*};

use crate::shooting::{gameset::{StartupGameSet, UpdateGameSet}, shooting_game::{movement::movement_component::Movement2d, player::player_component::{Player, PlayerAssets, PlayerBundle, player_move, player_shot}, shooter::shooter_component::Shooter}};

pub struct PlayerPlugin;

fn startup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(
        PlayerAssets {
            mesh: meshes.add(Circle::default()),
            material: materials.add(Color::from(PURPLE)),
        }
    );
}

fn spawn_player(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
) {
    commands.spawn(
        PlayerBundle::new(
            Vec3::ZERO,
            200.0,
            30,
            &player_assets,
        ));
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_player.in_set(StartupGameSet::Initialize));
        app.add_systems(Startup, spawn_player.in_set(StartupGameSet::Spawn));
        app.add_systems(Update, player_move.in_set(UpdateGameSet::PreUpdate));
        app.add_systems(Update, player_shot.in_set(UpdateGameSet::Update));
    }
}