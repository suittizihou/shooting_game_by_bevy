use bevy::prelude::*;

use crate::shooting::shooting_game::faction::faction_component::Faction;

#[derive(Component, Default)]
pub struct Shooter {
    damage: u32,
    interval: f32,
    next_fire_time: f32,
    faction: Faction,
}

impl Shooter {
    pub fn new(damage: u32, interval: f32, faction: Faction) -> Self {
        Self::default().with_damage(damage).with_interval(interval).with_faction(faction)
    }

    pub fn with_damage(mut self, damage: u32) -> Self {
        self.damage = damage;
        self
    }

    pub fn with_interval(mut self, interval: f32) -> Self {
        self.interval = interval;
        self
    }

    pub fn with_faction(mut self, faction: Faction) -> Self {
        self.faction = faction;
        self
    }

    pub const fn can_fire(&self, now: f32) -> bool {
        now >= self.next_fire_time
    }

    pub fn mark_fired(&mut self, now: f32) {
        self.next_fire_time = now + self.get_interval();
    }

    pub const fn get_damage(&self) -> u32 {
        self.damage
    }

    pub const fn get_interval(&self) -> f32 {
        self.interval
    }

    pub const fn get_faction(&self) -> Faction {
        self.faction
    }
}

#[derive(Bundle)]
pub struct ShooterBundle {
    pub transform: Transform,
    pub shooter: Shooter,
}

impl ShooterBundle {
    pub fn new(
        transform: Transform,
        damage: u32,
        interval: f32,
        faction: Faction,
    ) -> Self {
        Self {
            transform,
            shooter: Shooter::new(damage, interval, faction),
        }
    }
}