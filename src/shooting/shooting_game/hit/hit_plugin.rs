use bevy::prelude::*;

use crate::shooting::{gameset::PostUpdateGameSet, shooting_game::hit::{hit_dispatch::hit_dispatcher, hit_message::{ProjectileHitEnemyMessage, ProjectileHitPlayerMessage}}};

pub struct HitPlugin;

impl Plugin for HitPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ProjectileHitPlayerMessage>();
        app.add_message::<ProjectileHitEnemyMessage>();

        app.add_systems(PostUpdate, (
            hit_dispatcher,
        ).in_set(PostUpdateGameSet::PhysicsUpdate));
    }
}