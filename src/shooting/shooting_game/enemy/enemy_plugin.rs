use bevy::{color::palettes::css::RED, prelude::*};

use crate::shooting::{gameset::{StartupGameSet, UpdateGameSet}, shooting_game::{enemy::{enemy_resource::EnemyResources, enemy_system::enemy_shot}, enemy_spawner::{enemy_spawner_resource::EnemySpawnerResources, enemy_spawner_system::spawn_enemies}}};

pub struct EnemyPlugin;

fn startup_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(
           EnemyResources {
            mesh: meshes.add(Circle::default()),
            material: materials.add(Color::from(RED)),
        }
    );

    commands.insert_resource(
        EnemySpawnerResources {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        } 
    );
}

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
    }
}