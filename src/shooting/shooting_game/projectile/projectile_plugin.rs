use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_rapier2d::plugin::PhysicsSet;

use crate::shooting::{gameset::StartupGameSet, shooting_game::projectile::{projectile_message::ProjectileMessage, projectile_resource::ProjectileResources, projectile_system::{collision_to_enemy, collision_to_player, spawn_projectile_from_event}}};

pub struct ProjectilePlugin;

fn setup_projectile_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ProjectileResources {
        mesh: meshes.add(Circle::default()),
        material: materials.add(Color::from(YELLOW)),
    });
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ProjectileMessage>();
        
        app.add_systems(Startup, setup_projectile_assets.in_set(StartupGameSet::PostInitialize));
        app.add_systems(
            PostUpdate,
            spawn_projectile_from_event.after(TransformSystems::Propagate),
        );
        app.add_systems(PostUpdate, 
            (
                collision_to_player,
                collision_to_enemy
            ).after(PhysicsSet::StepSimulation));
    }
}