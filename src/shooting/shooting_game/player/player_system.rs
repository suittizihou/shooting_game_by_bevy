use bevy::{color::palettes::css::PURPLE, ecs::relationship::Relationship, prelude::*};

use crate::shooting::shooting_game::{debri::debri_message::DebriMessage, hp::hp_component::Hp, movement::movement_component::Movement2d, player::{player_bundle::PlayerBundle, player_component::Player, player_resource::PlayerResources}, projectile::projectile_message::ProjectileMessage, shooter::shooter_component::Shooter, take_damage::take_damage_message::TakeDamageMessage};

pub fn startup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(
        PlayerResources {
            mesh: meshes.add(Circle::default()),
            material: materials.add(Color::from(PURPLE)),
        }
    );
}

pub fn spawn_player(
    mut commands: Commands,
    player_res: Res<PlayerResources>,
) {
    PlayerBundle::spawn(
            &mut commands,
            Vec3::ZERO,
            10000.0,
            100,
            30,
            &player_res,
        );
}

pub fn player_move(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Movement2d, With<Player>>) {
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
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut projectile_message: MessageWriter<ProjectileMessage>,
    mut shooters: Query<(Entity, &mut Shooter, &ChildOf)>,
    players: Query<(), With<Player>>,
) {
    if !input.pressed(KeyCode::Space) {
        return;
    }

    let now = time.elapsed_secs();

    for (entity, mut shooter, child_of) in &mut shooters {
        if players.get(child_of.get()).is_err() {
            continue;
        };

        if shooter.can_fire(now) == false {
            continue;
        }
        
        projectile_message.write(
            ProjectileMessage { 
                entity,
            },
        );

        shooter.mark_fired(now);
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
                debri.write(DebriMessage { entity: message.entity });
            }
        };
    }
}