use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::shooting::shooting_game::movement::movement_component::Movement2d;

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Movement2d), With<Movement2d>>,
) {
    for (mut rigidbody, movement) in &mut query {
        rigidbody.linvel = movement.velocity(time.delta_secs()).xy();
    }
}
