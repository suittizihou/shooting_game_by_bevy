use bevy::prelude::*;

use crate::shooting::{gameset::{StartupGameSet, UpdateGameSet}, shooting_game::{debri::debri_plugin::DebriPlugin, movement::movement_plugin::MovementPlugin, player::player_plugin::PlayerPlugin, projectile::projectile_plugin::ProjectilePlugin}};

pub struct ShootingPlugin;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, 
            (
                StartupGameSet::Initialize,
                StartupGameSet::PostInitialize,
                StartupGameSet::Spawn
            ).chain());
            
        app.configure_sets(Update, 
            (
                UpdateGameSet::PreUpdate,
                UpdateGameSet::Update,
                UpdateGameSet::LateUpdate
            ).chain());

        app.add_plugins(MovementPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(ProjectilePlugin);
        app.add_plugins(DebriPlugin);

        app.add_systems(Startup, spawn_camera.in_set(StartupGameSet::Spawn));
    }
}