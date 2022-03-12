use crate::game::map::MapPosition;

use super::attribute::{self, Attribute};
use super::MapPositionPath;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct WarriorEventWriterQuery<'w, 's> {
    pub ew_damage: EventWriter<'w, 's, Damage>,
    pub ew_heal: EventWriter<'w, 's, Heal>,
    pub ew_shield: EventWriter<'w, 's, Shield>,
    pub ew_move: EventWriter<'w, 's, Move>,
}

pub struct Damage(pub Entity, pub u32, pub f32);
pub struct DrainHealth(pub Entity, pub Entity, pub u32, pub f32);
pub struct Heal(pub Entity, pub u32);
pub struct Shield(pub Entity, pub u32);
pub struct Move(pub Entity, pub MapPosition);

pub fn process_damage_event(
    mut events: EventReader<Damage>,
    mut warrior: Query<(
        &mut Attribute<attribute::Health>,
        &mut Attribute<attribute::Shield>,
    )>,
) {
    for Damage(entity, amount, erode) in events.iter() {
        if let Ok((mut health, mut shield)) = warrior.get_mut(*entity) {
            let remaining = shield.damage(*amount);
            health.drop(remaining);
            health.erode(remaining, *erode);
        }
    }
}

pub fn process_drain_health_event(
    mut events: EventReader<DrainHealth>,
    mut ew_damage: EventWriter<Damage>,
    mut ew_heal: EventWriter<Heal>,
    warrior: Query<(&Attribute<attribute::Health>, &Attribute<attribute::Shield>)>,
) {
    for DrainHealth(from, to, amount, erode) in events.iter() {
        let mut drained = 0;

        if let Ok((health, shield)) = warrior.get(*from) {
            drained = (amount - shield.value()).min(health.value());
        }

        ew_damage.send(Damage(*to, *amount, *erode));
        ew_heal.send(Heal(*from, drained));
    }
}

pub fn process_heal_event(
    mut events: EventReader<Heal>,
    mut warrior: Query<&mut Attribute<attribute::Health>>,
) {
    for Heal(entity, amount) in events.iter() {
        if let Ok(mut health) = warrior.get_mut(*entity) {
            health.rise(*amount);
        }
    }
}

pub fn process_shield_event(
    mut events: EventReader<Shield>,
    mut warrior: Query<&mut Attribute<attribute::Shield>>,
) {
    for Shield(entity, amount) in events.iter() {
        if let Ok(mut shield) = warrior.get_mut(*entity) {
            shield.protect(*amount);
        }
    }
}

pub fn process_move_event(mut events: EventReader<Move>, mut warrior: Query<&mut MapPositionPath>) {
    for Move(entity, to) in events.iter() {
        if let Ok(mut path) = warrior.get_mut(*entity) {
            path.0.push(to.clone())
        }
    }
}
