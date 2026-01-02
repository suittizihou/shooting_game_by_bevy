use crate::shooting::shooting_game::faction::faction_component::Faction;
use crate::shooting::shooting_game::shooter::shooter_component::Shooter;
use bevy::prelude::{Bundle, Transform};

#[derive(Clone)]
pub struct ShooterCore {
    transform: Transform,
    damage: u32,
    interval: f32,
    faction: Faction,
}

impl ShooterCore {
    pub fn new(transform: Transform, damage: u32, interval: f32, faction: Faction) -> Self {
        Self {
            transform,
            damage,
            interval,
            faction,
        }
    }
}

#[derive(Bundle)]
pub struct ShooterBundle {
    pub transform: Transform,
    pub shooter: Shooter,
}

impl ShooterBundle {
    pub fn new(core: &ShooterCore) -> Self {
        Self {
            transform: core.transform,
            shooter: Shooter::new(core.damage, core.interval, core.faction),
        }
    }
}
