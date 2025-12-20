use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, RigidBody, Sensor};

use crate::shooting::shooting_game::movement::movement_bundle::Movement2dBundle;

#[derive(Bundle)]
pub struct MoveEntityBundle {
    pub transform: Transform,
    pub collider: Collider,
    pub active_event: ActiveEvents,
    pub active_collision_types: ActiveCollisionTypes,
    pub sensor: Sensor,
    pub rigidbody: RigidBody,
    #[bundle()]
    pub movement: Movement2dBundle,
}

impl MoveEntityBundle {
    pub fn new(
        position: Vec3,
        angle: f32,
        size: f32,
        move_speed: f32,
        move_dir: Option<Vec2>,
    ) -> Self {
        Self {
            transform: Transform::default()
            .with_translation(position)
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, f32::to_radians(angle)))
            .with_scale(Vec3::splat(size)),
            collider: Collider::ball(0.5),
            active_event: ActiveEvents::COLLISION_EVENTS,
            active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            sensor: Sensor,
            rigidbody: RigidBody::KinematicVelocityBased,
            movement: Movement2dBundle::new(
                move_dir.unwrap_or_default(), move_speed,
            ),
        }
    }
}