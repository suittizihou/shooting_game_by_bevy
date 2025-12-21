use bevy::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;

use crate::shooting::shooting_game::hit::{hit_dispatch::hit_dispatcher, hit_message::{ProjectileHitEnemyMessage, ProjectileHitPlayerMessage}};

pub struct HitPlugin;

impl Plugin for HitPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ProjectileHitPlayerMessage>();
        app.add_message::<ProjectileHitEnemyMessage>();

        app.add_systems(PostUpdate, (
            hit_dispatcher,
        ).after(PhysicsSet::StepSimulation));
    }
}