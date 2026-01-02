use crate::shooting::shooting_game::projectile::projectile_message::ProjectileMessage;
use bevy::prelude::*;

#[derive(Component)]
pub struct NormalShooter;

impl NormalShooter {
    pub fn shot(
        &self,
        entity: Entity,
        direction: Vec2,
        message: &mut MessageWriter<ProjectileMessage>,
    ) {
        message.write(ProjectileMessage { entity, direction });
    }
}
