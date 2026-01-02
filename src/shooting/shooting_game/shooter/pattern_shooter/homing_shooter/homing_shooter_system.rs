use bevy::prelude::*;
use crate::shooting::shooting_game::projectile::projectile_message::ProjectileMessage;
use crate::shooting::shooting_game::shooter::pattern_shooter::homing_shooter::homing_shooter_component::HomingShooter;
use crate::shooting::shooting_game::shooter::pattern_shooter::homing_shooter::homing_shooter_message::HomingShooterMessage;
use crate::shooting::shooting_game::shooter::shooter_component::Shooter;

pub fn homing_shooter_system(
    time: Res<Time>,
    mut shooter_message: MessageReader<HomingShooterMessage>,
    mut projectile_message: MessageWriter<ProjectileMessage>,
    mut shooters: Query<(Entity, &GlobalTransform, &mut Shooter, &HomingShooter)>,
    entities: Query<&Transform>,
) {
    let now = time.elapsed_secs();
    for message in shooter_message.read() {
        if let Ok((entity, transform, mut shooter_core, shooter)) =
            shooters.get_mut(message.shooter_common.entity)
        {
            if shooter_core.try_shot(now) {
                if let Ok(target) = entities.get(message.target_entity) {
                    let dir = target.translation - transform.translation();
                    shooter.shot(entity, dir.xy(), &mut projectile_message);
                }
            }
        }
    }
}
