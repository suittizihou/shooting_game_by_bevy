use bevy::prelude::*;

#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Faction {
    #[default]
    Player,
    Enemy,
}
