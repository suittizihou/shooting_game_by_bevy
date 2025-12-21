use bevy::prelude::*;

use crate::shooting::{gameset::{PostUpdateGameSet, StartupGameSet, UpdateGameSet}, shooting_game::{enemy::enemy_system::{apply_damage_enemy, enemy_shot, startup_enemy}, enemy_spawner::enemy_spawner_system::spawn_enemies}};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            startup_enemy.in_set(StartupGameSet::Initialize),
        );
        app.add_systems(
            Update,
            spawn_enemies.in_set(UpdateGameSet::PreUpdate),
        );
        app.add_systems(
            Update,
            enemy_shot.after(TransformSystems::Propagate),
        );
        app.add_systems(
            PostUpdate,
            apply_damage_enemy.in_set(PostUpdateGameSet::LateUpdate),
        );
    }
}