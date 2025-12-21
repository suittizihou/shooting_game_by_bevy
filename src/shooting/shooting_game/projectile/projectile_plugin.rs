use bevy::prelude::*;

use crate::shooting::{
    gameset::{PostUpdateGameSet, StartupGameSet},
    shooting_game::projectile::{
        projectile_message::ProjectileMessage,
        projectile_system::{
            collision_to_enemy, collision_to_player, setup_projectile_assets,
            spawn_projectile_from_event,
        },
    },
};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ProjectileMessage>();

        app.add_systems(
            Startup,
            setup_projectile_assets.in_set(StartupGameSet::PostInitialize),
        );
        app.add_systems(
            PostUpdate,
            spawn_projectile_from_event.after(TransformSystems::Propagate),
        );
        app.add_systems(
            PostUpdate,
            (collision_to_player, collision_to_enemy).in_set(PostUpdateGameSet::Update),
        );
    }
}
