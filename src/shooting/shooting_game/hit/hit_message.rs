use bevy::prelude::*;

#[derive(Message, Copy, Clone)]
pub struct ProjectileHitPlayer {
    pub projectile: Entity,
    pub player: Entity,
}

#[derive(Message, Copy, Clone)]
pub struct ProjectileHitEnemy {
    pub projectile: Entity,
    pub enemy: Entity,
}