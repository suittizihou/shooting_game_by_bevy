use crate::shooting::shooting_game::shooter::shooter_bundle::{ShooterBundle, ShooterCore};
use bevy::prelude::*;
use crate::shooting::shooting_game::shooter::pattern_shooter::homing_shooter::homing_shooter_component::HomingShooter;

#[derive(Bundle)]
pub struct HomingShooterBundle {
    #[bundle()]
    shooter_bundle: ShooterBundle,
    homing_shooter: HomingShooter,
}

impl HomingShooterBundle {
    pub fn new(core: &ShooterCore) -> Self {
        Self {
            shooter_bundle: ShooterBundle::new(core),
            homing_shooter: HomingShooter,
        }
    }
}
