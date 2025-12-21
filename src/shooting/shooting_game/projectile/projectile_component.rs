use bevy::prelude::*;

use crate::shooting::shooting_game::faction::faction_component::Faction;

#[derive(Component, Default)]
pub struct Projectile {
    damage: u32,
    faction: Faction,
}

impl Projectile {
    pub fn new(damage: u32, faction: Faction) -> Self {
        Self::default().with_damage(damage).with_faction(faction)
    }

    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage = damage;
        self
    }

    pub fn with_faction(mut self, faction: Faction) -> Self {
        self.faction = faction;
        self
    }

    pub fn faction(&self) -> Faction {
        self.faction
    }

    pub fn damage(&self) -> u32 {
        self.damage
    }
}
