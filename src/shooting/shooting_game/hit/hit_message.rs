use bevy::prelude::*;

#[derive(Message, Copy, Clone)]
pub struct ProjectileHitPlayerMessage {
    pub projectile: Entity,
    pub player: Entity,
}

#[derive(Message, Copy, Clone)]
pub struct ProjectileHitEnemyMessage {
    pub projectile: Entity,
    pub enemy: Entity,
}
