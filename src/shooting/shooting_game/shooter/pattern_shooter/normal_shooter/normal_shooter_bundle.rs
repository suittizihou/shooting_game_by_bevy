use crate::shooting::shooting_game::shooter::shooter_bundle::{ShooterBundle, ShooterCore};
use bevy::prelude::*;
use crate::shooting::shooting_game::shooter::pattern_shooter::normal_shooter::normal_shooter_component::NormalShooter;

#[derive(Bundle)]
pub struct NormalShooterBundle {
    #[bundle()]
    shooter_bundle: ShooterBundle,
    normal_shooter: NormalShooter,
}

impl NormalShooterBundle {
    pub fn new(core: &ShooterCore) -> Self {
        Self {
            shooter_bundle: ShooterBundle::new(core),
            normal_shooter: NormalShooter,
        }
    }
}
