use super::attribute::Health;
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
    name: &'static str,
    effect: WeaponEffect,
}

#[derive(Default, Copy, Clone)]
pub struct WeaponEffect {
    to_health: HealthEffect,
    ap_cost: u32,
    range: (u32, u32),
}

#[derive(Copy, Clone)]
pub enum HealthEffect {
    Damage(u32),
    Heal(u32),
    Ineffective,
}

impl Weapon {
    pub fn use_on(&self, target: &mut Health) {
        match self.effect.to_health {
            HealthEffect::Damage(amount) => target.hurt(amount),
            HealthEffect::Heal(amount) => target.heal(amount),
            HealthEffect::Ineffective => (),
        }
    }
}

impl Default for HealthEffect {
    fn default() -> Self {
        HealthEffect::Ineffective
    }
}
