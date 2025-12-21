use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::shooting::shooting_game::{enemy::enemy_component::Enemy, hit::hit_message::{ProjectileHitEnemy, ProjectileHitPlayer}, player::player_component::Player, projectile::projectile_component::Projectile};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HitKind {
    Projectile,
    Player,
    Enemy,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct OrderedPair<A, B> {
    left: (A, B),
    right: (A, B),
}

impl<A: Copy, B: Ord + Copy> OrderedPair<A, B> {
    fn new(left: (A, B), right: (A, B)) -> Self {
        if left.1 <= right.1 {
            Self {
                left,
                right,
            }
        }
        else {
            Self{
                left: right,
                right: left,
            }
        }
    }
}

fn classify(
    entity: Entity,
    players: &Query<(), With<Player>>,
    enemies: &Query<(), With<Enemy>>,
    projectiles: &Query<(), With<Projectile>>,
) -> Option<HitKind> {
    if players.get(entity).is_ok() { return Some(HitKind::Player); }
    if enemies.get(entity).is_ok() { return Some(HitKind::Enemy); }
    if projectiles.get(entity).is_ok() { return Some(HitKind::Projectile); }
    None
}

struct HitHandlers;

impl HitHandlers {
    fn handle(
        &self,
        hit_player: &mut MessageWriter<ProjectileHitPlayer>,
        hit_enemy: &mut MessageWriter<ProjectileHitEnemy>,
        a: Entity,
        b: Entity,
        ka: HitKind,
        kb: HitKind,
    ) {
        let pair = OrderedPair::new((a, ka), (b, kb));
        let (a, ka) = pair.left;
        let (b, kb) = pair.right;

        match (ka, kb) {
            (HitKind::Projectile, HitKind::Player) => {
                hit_player.write(ProjectileHitPlayer {
                    projectile: a,
                    player: b,
                });
            }
            (HitKind::Projectile, HitKind::Enemy) => {
                hit_enemy.write(ProjectileHitEnemy {
                    projectile: a,
                    enemy: b,
                });
            }
            (HitKind::Player, HitKind::Enemy) => {},
            (HitKind::Player, HitKind::Player) => {},
            (HitKind::Enemy, HitKind::Enemy) => {},
            (HitKind::Projectile, HitKind::Projectile) => {},
            _ => {},
        }
    }
}

pub fn hit_dispatcher(
    mut collisions: MessageReader<CollisionEvent>,
    players: Query<(), With<Player>>,
    enemies: Query<(), With<Enemy>>,
    projectiles: Query<(), With<Projectile>>,
    mut hit_player: MessageWriter<ProjectileHitPlayer>,
    mut hit_enemy: MessageWriter<ProjectileHitEnemy>,
) {
    let hander = HitHandlers;
    for col in collisions.read() {
        let (a, b) = match col {
            CollisionEvent::Started(a, b, _) => (*a, *b),
            _ => continue,
        };
        let Some(ka) = classify(a, &players, &enemies, &projectiles) else { continue };
        let Some(kb) = classify(b, &players, &enemies, &projectiles) else { continue };
        hander.handle(&mut hit_player, &mut hit_enemy, a, b, ka, kb);
    }
}