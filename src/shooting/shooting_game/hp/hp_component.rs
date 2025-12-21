use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Hp {
    hp: u32,
}

impl Hp {
    pub fn with_hp(mut self, hp: u32) -> Self {
        self.hp = hp;
        self
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.hp = self.hp.saturating_sub(damage);
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }
}