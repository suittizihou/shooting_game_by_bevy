use bevy::{color::palettes::css::PURPLE, prelude::*};

use crate::shooting::{gameset::{StartupGameSet, UpdateGameSet}, shooting_game::{movement::{movement_component::Movement2d}, player::player_component::{Player, player_move, player_shot}, shooter::shooter_component::Shooter}};

pub struct PlayerPlugin;

fn player_spawn(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Player,
        Shooter::new(30, 0.1),
        Transform::default().with_scale(Vec3::splat(30.0)),
        Movement2d::new(Vec2::ZERO, 200.0),
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
    ));
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_spawn.in_set(StartupGameSet::Spawn));
        app.add_systems(Update, player_move.in_set(UpdateGameSet::PreUpdate));
        app.add_systems(Update, player_shot.in_set(UpdateGameSet::Update));
    }
}