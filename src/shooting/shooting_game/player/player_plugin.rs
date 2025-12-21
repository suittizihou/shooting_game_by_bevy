use bevy::prelude::*;

use crate::shooting::{
    gameset::{PostUpdateGameSet, StartupGameSet, UpdateGameSet},
    shooting_game::player::player_system::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_player.in_set(StartupGameSet::Initialize));
        app.add_systems(Startup, spawn_player.in_set(StartupGameSet::Spawn));
        app.add_systems(Update, player_move.in_set(UpdateGameSet::PreUpdate));
        app.add_systems(Update, player_shot.in_set(UpdateGameSet::Update));
        app.add_systems(
            PostUpdate,
            apply_damage_player.in_set(PostUpdateGameSet::LateUpdate),
        );
    }
}
