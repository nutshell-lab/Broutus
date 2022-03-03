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

impl Health {
    pub fn hurt(&mut self, amount: u32) {
        self.0.drop(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.0.rise(amount);
    }
}

impl ActionPoints {
    pub fn can_spend(&mut self, amount: u32) -> bool {
        self.0.can_drop(amount)
    }

    pub fn spend(&mut self, amount: u32) {
        self.0.drop(amount);
    }

    pub fn reset(&mut self) {
        self.0.rise_max();
    }
}

impl MovementPoints {
    pub fn can_spend(&mut self, amount: u32) -> bool {
        self.0.can_drop(amount)
    }

    pub fn spend(&mut self, amount: u32) {
        self.0.drop(amount);
    }

    pub fn reset(&mut self) {
        self.0.rise_max();
    }
}

impl Attribute {
    pub fn can_drop(&self, amount: u32) -> bool {
        self.value >= amount
    }

    pub fn drop(&mut self, amount: u32) {
        self.value = self.value.checked_sub(amount).unwrap_or(0);
    }

    pub fn rise(&mut self, amount: u32) {
        self.value = self
            .value
            .checked_add(amount)
            .filter(|v| v < &self.max)
            .unwrap_or(self.max)
    }

    pub fn rise_max(&mut self) {
        self.rise(self.max);
    }
}
