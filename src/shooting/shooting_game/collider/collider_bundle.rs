use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub active_event: ActiveEvents,
    pub active_collision_types: ActiveCollisionTypes,
}

#[derive(Bundle)]
pub struct ColliderSensorBundle {
    #[bundle()]
    pub collider_bundle: ColliderBundle,
    pub sensor: Sensor,
}
