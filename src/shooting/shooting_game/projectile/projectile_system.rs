use bevy::{color::palettes::css::YELLOW, prelude::*};

use crate::shooting::shooting_game::{debri::debri_message::DebriMessage, faction::faction_component::Faction, hit::hit_message::{ProjectileHitEnemyMessage, ProjectileHitPlayerMessage}, projectile::{projectile_bundle::ProjectileBundle, projectile_component::Projectile, projectile_message::ProjectileMessage, projectile_resource::ProjectileResources}, shooter::shooter_component::Shooter, take_damage::take_damage_message::TakeDamageMessage};

pub fn setup_projectile_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(ProjectileResources {
        mesh: meshes.add(Circle::default()),
        material: materials.add(Color::from(YELLOW)),
    });
}

pub fn spawn_projectile_from_event(
    mut commands: Commands,
    mut event: MessageReader<ProjectileMessage>,
    projectile_resources: Res<ProjectileResources>,
    query: Query<(&GlobalTransform, &Shooter)>,
) {
    for req in event.read() {
        if let Ok((transform, shooter)) = query.get(req.entity) {
            commands.spawn(
                ProjectileBundle::new(
                    shooter,
                    transform.translation(),
                    transform.up().xy(),
                    &projectile_resources,
                )
            );
        }
    }
}

pub fn collision_to_player(
    mut messages: MessageReader<ProjectileHitPlayerMessage>,
    projectiles: Query<&Projectile>,
    mut debri_message: MessageWriter<DebriMessage>,
    mut take_damage_message: MessageWriter<TakeDamageMessage>,
) {
    for message in messages.read() {
        let Ok(projectile) = projectiles.get(message.projectile) else {
            continue;
        };
        match projectile.faction() {
            Faction::Enemy => {
                take_damage_message.write(TakeDamageMessage { 
                    entity: message.player,
                    damage: projectile.damage() 
                });
                debri_message.write(DebriMessage { entity: message.projectile });
            },
            Faction::Player => {},
        }
    }
}

pub fn collision_to_enemy(
    mut messages: MessageReader<ProjectileHitEnemyMessage>,
    projectiles: Query<&Projectile>,
    mut debri_message: MessageWriter<DebriMessage>,
    mut take_damage_message: MessageWriter<TakeDamageMessage>,
) {
    for message in messages.read() {
        let Ok(projectile) = projectiles.get(message.projectile) else {
            continue;
        };
        match projectile.faction() {
            Faction::Player => {
                take_damage_message.write(TakeDamageMessage {
                    entity: message.enemy,
                    damage: projectile.damage() 
                });
                debri_message.write(DebriMessage { entity: message.projectile });
            },
            Faction::Enemy => {},
        }
    }
}