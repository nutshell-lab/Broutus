use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    current_life: u32,
    total_life: u32,
}

impl Health {
    pub fn hurt(&mut self, amount: u32) {
        self.current_life -= amount;
    }

    pub fn heal(&mut self, amount: u32) {
        self.current_life += amount;
    }
}
