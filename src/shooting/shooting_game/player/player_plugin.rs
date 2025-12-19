use bevy::{color::palettes::css::PURPLE, prelude::*};

use crate::shooting::{gameset::{StartupGameSet, UpdateGameSet}, shooting_game::player::player_component::{PlayerAssets, PlayerBundle, player_move, player_shot}};

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
    PlayerBundle::spawn(
            &mut commands,
            Vec3::ZERO,
            200.0,
            30,
            &player_assets,
        );
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_player.in_set(StartupGameSet::Initialize));
        app.add_systems(Startup, spawn_player.in_set(StartupGameSet::Spawn));
        app.add_systems(Update, player_move.in_set(UpdateGameSet::PreUpdate));
        app.add_systems(Update, player_shot.in_set(UpdateGameSet::Update));
    }
}