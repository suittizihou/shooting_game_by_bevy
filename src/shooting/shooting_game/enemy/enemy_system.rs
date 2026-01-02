use crate::shooting::shooting_game::enemy::enemy_spawner::enemy_spawner_resource::EnemySpawnerResources;
use crate::shooting::shooting_game::player::player_component::Player;
use crate::shooting::shooting_game::shooter::pattern_shooter::homing_shooter::homing_shooter_message::HomingShooterMessage;
use crate::shooting::shooting_game::shooter::shooter_component::Shooter;
use crate::shooting::shooting_game::shooter::shooter_message::ShooterMessageCommon;
use crate::shooting::shooting_game::{
    debri::debri_message::DebriMessage,
    enemy::{enemy_component::Enemy, enemy_resource::EnemyResources},
    hp::hp_component::Hp,
    take_damage::take_damage_message::TakeDamageMessage,
};
use bevy::ecs::relationship::Relationship;
use bevy::{color::palettes::css::RED, prelude::*};

pub fn startup_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(EnemyResources {
        mesh: meshes.add(Circle::default()),
        material: materials.add(Color::from(RED)),
    });

    commands.insert_resource(EnemySpawnerResources {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });
}

pub fn enemy_shot(
    mut shooter_message: MessageWriter<HomingShooterMessage>,
    shooters: Query<(Entity, &ChildOf), With<Shooter>>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    players: Query<(Entity, &GlobalTransform), With<Player>>,
) {
    for (entity, child_of) in &shooters {
        let Ok(enemy_transform) = enemies.get(child_of.get()) else {
            continue;
        };
        let mut near_entity: Option<Entity> = None;
        let mut min_distance = f32::MAX;
        for (player_entity, player_transform) in &players {
            let distance = player_transform
                .translation()
                .distance(enemy_transform.translation());
            if min_distance >= distance {
                min_distance = distance;
                near_entity = Some(player_entity);
            }
        }

        match near_entity {
            Some(player) => {
                shooter_message.write(HomingShooterMessage {
                    shooter_common: ShooterMessageCommon { entity },
                    target_entity: player,
                });
            }
            None => {}
        };
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
                debri.write(DebriMessage {
                    entity: message.entity,
                });
            }
        };
    }
}
