use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

#[derive(Default)]
pub struct Turn {
    /// Current turn, is incremented the order has been consumed
    pub current: usize,

    /// Characters entities, sorted by their turn order
    pub order: Vec<Entity>,

    /// Current turn current turn order index
    pub order_index: usize
}

impl Turn {
    pub fn next(&mut self) {
        self.order_index = (self.order_index + 1) % (self.order.len() - 1);
        self.current = if self.order_index == 0 { self.current + 1 } else { self.current };
    }

    pub fn get_current_character_entity(self) -> Option<Entity> {
        self.order.get(self.order_index).map(|e| e.clone())
    }
}

pub fn debug_ui_turn(
    turn: Res<Turn>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Turn").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Current turn: {}", turn.current));
        ui.label(format!("Current turn order: {:?}", turn.order));
        ui.label(format!("Current turn order index: {}", turn.order_index));
    });
}
