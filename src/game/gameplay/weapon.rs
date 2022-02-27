use super::attribute::Health;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Weapon {
    name: String,
    texture: String,
    base_effect: Effect,
}

#[derive(Component, Default)]
pub struct Effect {
    amount: u32,
    range: (u32, u32),
    cost: u32,
    effect_type: EffectType,
}

pub enum EffectType {
    Attack(),
    Heal(),
    Ineffective,
}

impl Weapon {
    pub fn new(name: String, base_effect: Effect) -> Weapon {
        Weapon {
            name,
            base_effect,
            texture: String::from("path/to/file.png"),
        }
    }
    pub fn use_on(&self, target: &mut Health) {
        match self.base_effect.effect_type {
            EffectType::Attack() => target.hurt(self.base_effect.amount),
            EffectType::Heal() => target.heal(self.base_effect.amount),
            EffectType::Ineffective => (),
        }
    }
}

impl Effect {
    pub fn new(amount: u32, range: (u32, u32), cost: u32, effect_type: EffectType) -> Effect {
        Effect {
            amount,
            range,
            cost,
            effect_type,
        }
    }
}

impl Default for EffectType {
    fn default() -> Self {
        EffectType::Ineffective
    }
}
