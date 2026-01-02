use crate::shooting::shooting_game::shooter::shooter_component::Shooter;
use crate::shooting::shooting_game::{
    debri::debri_message::DebriMessage,
    hp::hp_component::Hp,
    movement::movement_component::Movement2d,
    player::{
        player_bundle::PlayerBundle, player_component::Player, player_resource::PlayerResources,
    },
    take_damage::take_damage_message::TakeDamageMessage,
};
use bevy::ecs::relationship::Relationship;
use bevy::{color::palettes::css::PURPLE, prelude::*};
use crate::shooting::shooting_game::shooter::pattern_shooter::normal_shooter::normal_shooter_message::NormalShooterMessage;
use crate::shooting::shooting_game::shooter::shooter_message::ShooterMessageCommon;

pub fn startup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(PlayerResources {
        mesh: meshes.add(Circle::default()),
        material: materials.add(Color::from(PURPLE)),
    });
}

pub fn spawn_player(mut commands: Commands, player_res: Res<PlayerResources>) {
    PlayerBundle::spawn(&mut commands, Vec3::ZERO, 10000.0, 100, 30, &player_res);
}

pub fn player_move(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Movement2d, With<Player>>,
) {
    for mut player in &mut query {
        let mut velocity = Vec2::ZERO;
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            velocity.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            velocity.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            velocity.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            velocity.x += 1.0;
        }
        player.set_direction(velocity);
    }
}

pub fn player_shot(
    input: Res<ButtonInput<KeyCode>>,
    mut shooter_message: MessageWriter<NormalShooterMessage>,
    shooters: Query<(Entity, &ChildOf), With<Shooter>>,
    players: Query<(), With<Player>>,
) {
    if !input.pressed(KeyCode::Space) {
        return;
    }

    for (entity, child_of) in &shooters {
        if players.get(child_of.get()).is_err() {
            continue;
        }
        shooter_message.write(NormalShooterMessage {
            shooter_common: ShooterMessageCommon { entity },
        });
    }
}

pub fn apply_damage_player(
    mut take_damage_messages: MessageReader<TakeDamageMessage>,
    mut players: Query<&mut Hp, With<Player>>,
    mut debri: MessageWriter<DebriMessage>,
) {
    for message in take_damage_messages.read() {
        if let Ok(mut hp) = players.get_mut(message.entity) {
            hp.take_damage(message.damage);

            if hp.is_dead() {
                debri.write(DebriMessage {
                    entity: message.entity,
                });
            }
        };
    }
}
