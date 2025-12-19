use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Projectile {
    damage: u32,
}

impl Projectile {    
    pub fn new(damage: u32) -> Self {
        Self::default().with_damage(damage)
    }

    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage = damage;
        self
    }
}