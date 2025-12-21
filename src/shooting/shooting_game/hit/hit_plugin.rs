use bevy::prelude::*;
use bevy_rapier2d::plugin::PhysicsSet;

use crate::shooting::shooting_game::hit::{hit_dispatch::hit_dispatcher, hit_message::{ProjectileHitEnemy, ProjectileHitPlayer}};

pub struct HitPlugin;

impl Plugin for HitPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ProjectileHitPlayer>();
        app.add_message::<ProjectileHitEnemy>();

        app.add_systems(PostUpdate, (
            hit_dispatcher,
        ).after(PhysicsSet::StepSimulation));
    }
}