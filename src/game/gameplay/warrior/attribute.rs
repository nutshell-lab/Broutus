use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// An attribute is a utility wrapper type for any warrior attribute (health, action points, armor, etc.)
#[derive(Debug, Copy, Clone, Default, Component, Deserialize, Serialize)]
pub struct Attribute<T: AttributeValue + Copy + Clone + Default> {
    value: T,
    min: T,
    max: T,
}

impl<T: AttributeValue + Copy + Clone + Default> Attribute<T> {
    pub fn value(self) -> u32 {
        self.value.value()
    }

    pub fn set_value(&mut self, value: T) {
        self.value
            .set_value(value.value().clamp(self.min(), self.max()))
    }

    pub fn min(self) -> u32 {
        self.min.value()
    }

    pub fn set_min(&mut self, min: T) {
        self.min.set_value(min.value().min(self.max()));
        if self.value() < self.min() {
            self.set_value(self.min)
        }
    }

    pub fn max(self) -> u32 {
        self.max.value()
    }

    pub fn set_max(&mut self, max: T) {
        self.max.set_value(max.value().max(self.min()));
        if self.value() > self.max() {
            self.set_value(self.max)
        }
    }

    pub fn can_drop(&self, amount: u32) -> bool {
        self.value() - self.min() >= amount
    }

    /// Drop the value by the amount, bounded by min, returning the remaining amount
    pub fn drop(&mut self, amount: u32) -> u32 {
        let old_value = self.value();
        let virtual_value = self.value().checked_sub(amount).unwrap_or(0);
        let real_value = virtual_value.clamp(self.min(), self.max());

        self.value.set_value(real_value);

        amount - (old_value - real_value)
    }

    pub fn drop_min(&mut self) -> u32 {
        self.drop(self.min())
    }

    pub fn rise(&mut self, amount: u32) -> u32 {
        let old_value = self.value();
        let virtual_value = self.value().checked_add(amount).unwrap_or(u32::MAX);
        let real_value = virtual_value.clamp(self.min(), self.max());

        self.value.set_value(real_value);

        amount - (real_value - old_value)
    }

    pub fn rise_max(&mut self) -> u32 {
        self.rise(self.max())
    }

    pub fn as_percentage(&self) -> f32 {
        (self.value() - self.min()) as f32 / (self.max() - self.min()) as f32
    }

    pub fn as_text(&self) -> String {
        format!("{} / {}", self.value(), self.max())
    }
}

impl Attribute<Health> {
    pub fn erode(&mut self, amount: u32, erode: f32) {
        let erosion = (amount as f32 * erode).round() as u32;
        let new_max = self
            .max
            .value()
            .checked_sub(erosion)
            .filter(|&v| v >= self.min())
            .unwrap_or(self.min());

        self.set_max(Health(new_max));
    }
}

/// A way to interact with attributes NewTypes (maybe their is a better way ?)
pub trait AttributeValue {
    fn value(&self) -> u32;
    fn set_value(&mut self, value: u32);
}

/// NewType representing a Warrior's action points quantity
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct ActionPoints(u32);

impl AttributeValue for ActionPoints {
    fn value(&self) -> u32 {
        self.0
    }
    fn set_value(&mut self, value: u32) {
        self.0 = value;
    }
}

/// NewType representing a Warrior's movement points quantity
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct MovementPoints(u32);

impl AttributeValue for MovementPoints {
    fn value(&self) -> u32 {
        self.0
    }
    fn set_value(&mut self, value: u32) {
        self.0 = value;
    }
}

/// NewType representing a Warrior's health quantity
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct Health(u32);

impl AttributeValue for Health {
    fn value(&self) -> u32 {
        self.0
    }
    fn set_value(&mut self, value: u32) {
        self.0 = value;
    }
}

/// NewType representing a Warrior's shield quantity
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct Shield(u32);

impl AttributeValue for Shield {
    fn value(&self) -> u32 {
        self.0
    }
    fn set_value(&mut self, value: u32) {
        self.0 = value;
    }
}
