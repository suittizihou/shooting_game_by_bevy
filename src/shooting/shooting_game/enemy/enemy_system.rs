use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::shooting::shooting_game::{debri::debri_message::DebriMessage, enemy::enemy_component::Enemy, hp::hp_component::Hp, projectile::projectile_message::ProjectileMessage, shooter::shooter_component::Shooter, take_damage::take_damage_message::TakeDamageMessage};

pub fn enemy_shot(
    time: Res<Time>,
    mut projectile_message: MessageWriter<ProjectileMessage>,
    mut shooters: Query<(Entity, &mut Shooter, &ChildOf)>,
    enemies: Query<(), With<Enemy>>,
) {
    let now = time.elapsed_secs();

    for (entity, mut shooter, child_of) in &mut shooters {
        if enemies.get(child_of.get()).is_err() {
            continue;
        };

        if shooter.can_fire(now) == false {
            continue;
        }

        projectile_message.write(
            ProjectileMessage {
                entity
            }
        );

        shooter.mark_fired(now);
    }
}

pub fn apply_damage_enemy(
    mut take_damage_messages: MessageReader<TakeDamageMessage>,
    mut enemies: Query<&mut Hp, With<Enemy>>,
    mut debri: MessageWriter<DebriMessage>,
) {
    for message in take_damage_messages.read() {
        if let Ok(mut hp) = enemies.get_mut(message.entity) {
            hp.take_damage(message.damage);

            if hp.is_dead() {
                debri.write(DebriMessage { entity: message.entity });
            }
        };
    }
}