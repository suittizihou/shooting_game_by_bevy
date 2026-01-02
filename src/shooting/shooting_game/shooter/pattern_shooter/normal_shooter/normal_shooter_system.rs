use bevy::prelude::*;
use crate::shooting::shooting_game::projectile::projectile_message::ProjectileMessage;
use crate::shooting::shooting_game::shooter::pattern_shooter::normal_shooter::normal_shooter_component::NormalShooter;
use crate::shooting::shooting_game::shooter::pattern_shooter::normal_shooter::normal_shooter_message::NormalShooterMessage;
use crate::shooting::shooting_game::shooter::shooter_component::Shooter;

pub fn normal_shooter_system(
    time: Res<Time>,
    mut shooter_message: MessageReader<NormalShooterMessage>,
    mut projectile_message: MessageWriter<ProjectileMessage>,
    mut query: Query<(Entity, &GlobalTransform, &mut Shooter, &NormalShooter)>,
) {
    let now = time.elapsed_secs();
    for message in shooter_message.read() {
        if let Ok((entity, transform, mut shooter, normal)) =
            query.get_mut(message.shooter_common.entity)
        {
            if shooter.try_shot(now) {
                normal.shot(entity, transform.up().xy(), &mut projectile_message);
            }
        }
    }
}
