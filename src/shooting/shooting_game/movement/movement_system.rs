use bevy::prelude::*;

use crate::shooting::shooting_game::movement::movement_component::Movement2d;

pub fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Movement2d), With<Movement2d>>) {
    for (mut transform, movement) in &mut query {
        transform.translation += movement.velocity(time.delta_secs());
    }
}