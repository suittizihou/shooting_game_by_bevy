use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StartupGameSet {
    Initialize,
    PostInitialize,
    Spawn,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UpdateGameSet {
    PreUpdate,
    Update,
    LateUpdate,
}