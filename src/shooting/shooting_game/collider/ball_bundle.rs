use crate::shooting::shooting_game::collider::collider_bundle::{
    ColliderBundle, ColliderSensorBundle,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveCollisionTypes, ActiveEvents, Collider, Sensor};

#[derive(Bundle)]
pub struct BallBundle {
    #[bundle()]
    pub collider_bundle: ColliderBundle,
}

impl BallBundle {
    pub fn new(radius: f32) -> Self {
        Self {
            collider_bundle: ColliderBundle {
                collider: Collider::ball(radius),
                active_event: ActiveEvents::COLLISION_EVENTS,
                active_collision_types: ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            },
        }
    }
}

#[derive(Bundle)]
pub struct BallSensorBundle {
    #[bundle()]
    pub collider_bundle: ColliderSensorBundle,
}

impl BallSensorBundle {
    pub fn new(radius: f32) -> Self {
        Self {
            collider_bundle: ColliderSensorBundle {
                collider_bundle: BallBundle::new(radius).collider_bundle,
                sensor: Sensor,
            },
        }
    }
}
