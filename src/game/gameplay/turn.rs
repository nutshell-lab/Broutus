use bevy::prelude::*;
#[derive(Default, Component)]
pub struct TeamA;

#[derive(Default, Component)]
pub struct TeamB;

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
    pub fn set_next(&mut self) {
        self.order_index = self.get_next_order_index();
        self.current = if self.order_index == 0 {
            self.current + 1
        } else {
            self.current
        };
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
