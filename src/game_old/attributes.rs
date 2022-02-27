use bevy::prelude::*;

#[derive(Reflect, Default)]
pub struct Attribute {
    pub value: u32,
    pub max: u32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health(pub Attribute);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ActionPoints(pub Attribute);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct MovementPoints(pub Attribute);

impl Attribute {
    pub fn reset(&mut self) {
        self.value = self.max;
    }
}

impl Health {
    pub fn hurt(&mut self, amount: u32) {
        self.0.value -= amount;
    }

    pub fn heal(&mut self, amount: u32) {
        self.0.value += amount;
    }
}
