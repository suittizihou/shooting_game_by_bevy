use crate::shooting::gameset::UpdateGameSet;
use crate::shooting::shooting_game::shooter::pattern_shooter::homing_shooter::homing_shooter_message::HomingShooterMessage;
use crate::shooting::shooting_game::shooter::pattern_shooter::normal_shooter::normal_shooter_message::NormalShooterMessage;
use crate::shooting::shooting_game::shooter::pattern_shooter::normal_shooter::normal_shooter_system::normal_shooter_system;
use bevy::prelude::*;
use crate::shooting::shooting_game::shooter::pattern_shooter::homing_shooter::homing_shooter_system::homing_shooter_system;

pub struct ShooterPlugin;

impl Plugin for ShooterPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<NormalShooterMessage>();
        app.add_message::<HomingShooterMessage>();

        app.add_systems(
            Update,
            (normal_shooter_system, homing_shooter_system).in_set(UpdateGameSet::LateUpdate),
        );
    }
}
