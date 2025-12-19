use bevy::{color::palettes::css::PURPLE, prelude::*};

use crate::shooting::{gameset::{StartupGameSet, UpdateGameSet}, shooting_game::player::{player_bundle::PlayerBundle, player_resource::PlayerResources, player_system::*}};

pub struct PlayerPlugin;

fn startup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(
        PlayerResources {
            mesh: meshes.add(Circle::default()),
            material: materials.add(Color::from(PURPLE)),
        }
    );
}

fn spawn_player(
    mut commands: Commands,
    player_res: Res<PlayerResources>,
) {
    PlayerBundle::spawn(
            &mut commands,
            Vec3::ZERO,
            200.0,
            30,
            &player_res,
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