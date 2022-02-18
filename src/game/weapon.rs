use super::attributes::Health;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Weapon {
    name: String,
    texture: String,
    base_effect: EffectType,
}

#[derive(Component)]
pub enum EffectType {
    Attack(u32),
    Heal(u32),
    Ineffective,
}

impl Weapon {
    pub fn new(name: String, base_effect: EffectType) -> Weapon {
        Weapon {
            name,
            base_effect,
            texture: String::from("path/to/file.png"),
        }
    }
    pub fn use_on(&self, target: &mut Health) {
        match self.base_effect {
            EffectType::Attack(damages) => target.hurt(damages),
            EffectType::Heal(amount) => target.heal(amount),
            EffectType::Ineffective => (),
        }
    }
}

impl Default for EffectType {
    fn default() -> Self {
        EffectType::Ineffective
    }
}
