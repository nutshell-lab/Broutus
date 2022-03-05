use crate::game::color::Color;

use super::SelectedAction;
use bevy::prelude::*;

#[derive(Default, Component, Copy, Clone)]
pub struct Team(TeamSide, Color);

impl Team {
    pub fn new(side: TeamSide, color: Color) -> Self {
        Self(side, color)
    }

    pub fn side(&self) -> TeamSide {
        self.0
    }

    pub fn color(&self) -> Color {
        self.1
    }
}

#[derive(Copy, Clone)]
pub enum TeamSide {
    A,
    B,
}

impl Default for TeamSide {
    fn default() -> Self {
        Self::A
    }
}

pub struct TurnStart(pub Entity);

pub struct TurnEnd(pub Entity);

#[derive(Default)]
pub struct Turn {
    /// Current turn, is incremented the order has been consumed
    pub current: usize,

    /// Warriors entities, sorted by their turn order
    pub order: Vec<Entity>,

    /// Current turn current turn order index
    pub order_index: usize,
}

impl Turn {
    /// Move forward in the turn system, select the next warrior, incrementing the turn count if necessary
    pub fn set_next(&mut self, mut start: EventWriter<TurnStart>, mut end: EventWriter<TurnEnd>) {
        end.send(TurnEnd(self.get_current_warrior_entity().unwrap()));
        self.order_index = self.get_next_order_index();
        self.current = if self.order_index == 0 {
            self.current + 1
        } else {
            self.current
        };
        start.send(TurnStart(self.get_current_warrior_entity().unwrap()));
    }

    pub fn get_next_order_index(&self) -> usize {
        (self.order_index + 1) % self.order.len()
    }

    pub fn get_current_warrior_entity(&self) -> Option<Entity> {
        self.order.get(self.order_index).copied()
    }

    pub fn get_entity_index(&self, entity: Entity) -> Option<usize> {
        self.order.iter().position(|e| e.eq(&entity))
    }
}

pub struct TurnTimer(pub Timer);

impl Default for TurnTimer {
    fn default() -> Self {
        TurnTimer(Timer::from_seconds(30.0, true))
    }
}

pub fn run_turn_timer(
    time: Res<Time>,
    mut timer: ResMut<TurnTimer>,
    mut turn: ResMut<Turn>,
    ev_turn_started: EventWriter<TurnStart>,
    ev_turn_ended: EventWriter<TurnEnd>,
    mut selected_action: ResMut<SelectedAction>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        selected_action.0 = None;
        turn.set_next(ev_turn_started, ev_turn_ended);
    }
}

pub fn reset_turn_timer(mut ev_turn_started: EventReader<TurnStart>, mut timer: ResMut<TurnTimer>) {
    for _ in ev_turn_started.iter() {
        timer.0.reset();
    }
}
