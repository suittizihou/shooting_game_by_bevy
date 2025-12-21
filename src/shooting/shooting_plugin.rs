use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shooting::{
    gameset::{PostUpdateGameSet, StartupGameSet, UpdateGameSet},
    shooting_game::{
        debri::debri_plugin::DebriPlugin, enemy::enemy_plugin::EnemyPlugin,
        hit::hit_plugin::HitPlugin, lifetime::lifetime_plugin::LifetimePlugin,
        movement::movement_plugin::MovementPlugin, player::player_plugin::PlayerPlugin,
        projectile::projectile_plugin::ProjectilePlugin,
        take_damage::take_damage_plugin::TakeDamagePlugin,
    },
};

pub struct ShootingPlugin;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (
                StartupGameSet::Initialize,
                StartupGameSet::PostInitialize,
                StartupGameSet::Spawn,
            )
                .chain(),
        );

        app.configure_sets(
            Update,
            (
                UpdateGameSet::PreUpdate,
                UpdateGameSet::Update,
                UpdateGameSet::LateUpdate,
            )
                .chain(),
        );

        app.configure_sets(
            PostUpdate,
            (
                PostUpdateGameSet::PreUpdate,
                PostUpdateGameSet::PhysicsUpdate.after(PhysicsSet::StepSimulation),
                PostUpdateGameSet::Update,
                PostUpdateGameSet::LateUpdate,
            )
                .chain(),
        );

        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ));

        app.add_plugins((
            MovementPlugin,
            PlayerPlugin,
            EnemyPlugin,
            ProjectilePlugin,
            LifetimePlugin,
            DebriPlugin,
            HitPlugin,
            TakeDamagePlugin,
        ));

        app.add_systems(Startup, spawn_camera.in_set(StartupGameSet::Spawn));
    }
}
