use bevy::prelude::*;

use crate::shooting::shooting_game::{enemy::{enemy_bundle::EnemyBundle, enemy_resource::EnemyResources}, enemy_spawner::enemy_spawner_resource::EnemySpawnerResources};

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_spawner_res: ResMut<EnemySpawnerResources>,
    enemy_res: Res<EnemyResources>,
) {
    if enemy_spawner_res.timer.tick(time.delta()).just_finished() {
        EnemyBundle::spawn(
            &mut commands,
            Vec3::new(0.0, 400.0, 0.0),
            10000.0,
            5,
            &enemy_res,
        );
    }
}