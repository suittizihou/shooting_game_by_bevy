use bevy::prelude::*;

use crate::shooting::shooting_plugin::ShootingPlugin;

mod shooting;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShootingPlugin)
        .run();
}
