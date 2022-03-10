use crate::game::map::MapPosition;

use super::Attribute;
use super::Health;
use super::Shield;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct WarriorEventWriterQuery<'w, 's> {
    pub ew_damage: EventWriter<'w, 's, Damage>,
    pub ew_heal: EventWriter<'w, 's, Heal>,
    pub ew_move: EventWriter<'w, 's, Move>,
}

pub struct Damage(pub Entity, pub u32, pub f32);
pub struct DrainHealth(pub Entity, pub Entity, pub u32, pub f32);
pub struct Heal(pub Entity, pub u32);
pub struct Move(pub Entity, pub MapPosition);

pub fn process_damage_event(
    mut events: EventReader<Damage>,
    mut warrior: Query<(&mut Attribute<Health>, &mut Attribute<Shield>)>,
) {
    for Damage(entity, amount, erode) in events.iter() {
        if let Ok((mut health, mut shield)) = warrior.get_mut(*entity) {
            let remaining = shield.drop(*amount);
            health.drop(remaining);
            health.erode(remaining, *erode);
        }
    }
}

pub fn process_drain_health_event(
    mut events: EventReader<DrainHealth>,
    mut ew_damage: EventWriter<Damage>,
    mut ew_heal: EventWriter<Heal>,
    warrior: Query<(&Attribute<Health>, &Attribute<Shield>)>,
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
    mut warrior: Query<&mut Attribute<Health>>,
) {
    for Heal(entity, amount) in events.iter() {
        if let Ok(mut health) = warrior.get_mut(*entity) {
            health.rise(*amount);
        }
    }
}

pub fn process_move_event(mut events: EventReader<Move>, mut warrior: Query<&mut MapPosition>) {
    for Move(entity, to) in events.iter() {
        if let Ok(mut position) = warrior.get_mut(*entity) {
            position.x = to.x;
            position.y = to.y;
        }
    }
}
