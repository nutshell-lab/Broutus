use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

#[derive(Default, Component)]
pub struct TeamA;

#[derive(Default, Component)]
pub struct TeamB;

#[derive(Default)]
pub struct Turn {
    /// Current turn, is incremented the order has been consumed
    pub current: usize,

    /// Characters entities, sorted by their turn order
    pub order: Vec<Entity>,

    /// Current turn current turn order index
    pub order_index: usize,
}

impl Turn {
    /// Move forward in the turn system, select the next character, incrementing the turn count if necessary
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

    pub fn get_current_character_entity(&self) -> Option<Entity> {
        self.order.get(self.order_index).map(|e| e.clone())
    }

    pub fn get_next_character_entity(&self) -> Option<Entity> {
        self.order
            .get(self.get_next_order_index())
            .map(|e| e.clone())
    }
}

/// Display all infos about the ruen system in a dedicated window
pub fn debug_ui_turn(
    mut turn: ResMut<Turn>,
    mut egui_context: ResMut<EguiContext>,
    character_query: Query<&Name, With<super::character::Character>>,
) {
    egui::Window::new("Turn").show(egui_context.ctx_mut(), |ui| {
        let order: Vec<&str> = turn
            .order
            .iter()
            .map(|&entity| character_query.get(entity))
            .filter(|name| name.is_ok())
            .map(|name| name.unwrap().as_str())
            .collect();

        ui.label(format!("Current turn: {}", turn.current));
        ui.label(format!("Current turn order: {:#?}", order));
        ui.label(format!("Current turn order index: {}", turn.order_index));

        if let Some(entity) = turn.get_current_character_entity() {
            if let Ok(name) = character_query.get(entity) {
                ui.label(format!("Current character: {}", name.as_str()));
            }
        }

        if let Some(entity) = turn.get_next_character_entity() {
            if let Ok(name) = character_query.get(entity) {
                ui.label(format!("Next character: {}", name.as_str()));
            }
        }

        if ui.button("Next").clicked() {
            turn.set_next();
        }
    });
}
