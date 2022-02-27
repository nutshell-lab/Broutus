use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::egui::{
    widgets::{Label, ProgressBar, Separator},
    RichText,
};

use super::attributes::Health;

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
}

/// Display all infos about the turn system in a dedicated window
pub fn debug_ui_turn(
    mut turn: ResMut<Turn>,
    mut ev_turn_started: EventWriter<TurnStart>,
    mut ev_turn_ended: EventWriter<TurnEnd>,
    mut egui_context: ResMut<EguiContext>,
    character_query: Query<(&Name, &Health), With<super::character::Character>>,
) {
    egui::Window::new("Turn").show(egui_context.ctx_mut(), |ui| {
        let mut display_slots = 8;
        let mut index = 0;

        while display_slots > 0 {
            ui.add(Label::new(
                RichText::new(format!("Turn {}", turn.current + index))
                    .strong()
                    .heading(),
            ));

            let offset = if index == 0 { turn.order_index } else { 0 };
            for &entity in turn.order.iter().skip(offset).take(display_slots) {
                let (name, health) = character_query.get(entity).unwrap();

                ui.add(Label::new(RichText::new(name.as_str()).strong()));
                ui.add(
                    ProgressBar::new(health.0.value as f32 / health.0.max as f32)
                        .text(format!("{} / {} hp", health.0.value, health.0.max)),
                );
                ui.add(Separator::default().horizontal());

                display_slots -= 1;
            }

            index += 1;
        }

        if ui.button("Next character").clicked() {
            ev_turn_ended.send(TurnEnd(turn.get_current_character_entity().unwrap()));
            turn.set_next();
            ev_turn_started.send(TurnStart(turn.get_current_character_entity().unwrap()));
        }
    });
}
