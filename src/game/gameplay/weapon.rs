use super::attribute::{ActionPoints, Health};
use bevy::prelude::*;

pub const THUG_KNIFE: Weapon = Weapon {
    name: "Dague du bandit",
    effect: WeaponEffect {
        to_health: HealthEffect::Damage(15),
        ap_cost: 3u32,
        range: (1u32, 1u32),
    },
};

pub const HEAL_WAND: Weapon = Weapon {
    name: "Baguette curative",
    effect: WeaponEffect {
        to_health: HealthEffect::Heal(8),
        ap_cost: 4u32,
        range: (2u32, 5u32),
    },
};

#[derive(Component, Default, Copy, Clone)]
pub struct Weapon {
    pub name: &'static str,
    pub effect: WeaponEffect,
}

#[derive(Default, Copy, Clone)]
pub struct WeaponEffect {
    pub to_health: HealthEffect,
    pub ap_cost: u32,
    pub range: (u32, u32),
}

#[derive(Copy, Clone)]
pub enum HealthEffect {
    Damage(u32),
    Heal(u32),
    Ineffective,
}

impl Weapon {
    pub fn use_on(&self, target_health: &mut Health) {
        match self.effect.to_health {
            HealthEffect::Damage(amount) => target_health.hurt(amount),
            HealthEffect::Heal(amount) => target_health.heal(amount),
            HealthEffect::Ineffective => (),
        }
    }
}

impl Default for HealthEffect {
    fn default() -> Self {
        HealthEffect::Ineffective
    }
}
