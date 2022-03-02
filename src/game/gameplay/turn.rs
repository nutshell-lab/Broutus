use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::egui::{
    widgets::{Label, ProgressBar, Separator},
    RichText,
};

use super::attribute::ActionPoints;
use super::attribute::Health;
use super::attribute::MovementPoints;

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
        self.order.get(self.order_index).map(|e| e.clone())
    }
}

/// Display all infos about the turn system in a dedicated window
pub fn show_turn_ui(
    mut turn: ResMut<Turn>,
    mut ev_turn_started: EventWriter<TurnStart>,
    mut ev_turn_ended: EventWriter<TurnEnd>,
    mut egui_context: ResMut<EguiContext>,
    warrior_query: Query<
        (&Name, &Health, &ActionPoints, &MovementPoints),
        With<super::warrior::Warrior>,
    >,
) {
    egui::containers::Area::new("turn_order")
        .anchor(egui::Align2::RIGHT_TOP, [-20.0, 20.0])
        .movable(false)
        .show(egui_context.ctx_mut(), |ui| {
            let mut display_slots = 8;
            let mut index = 0;

            ui.set_max_size([200.0, 1200.0].into());

            while display_slots > 0 {
                if index == 0 {
                    ui.add(Label::new(
                        RichText::new(format!("Turn {}", turn.current + index))
                            .strong()
                            .heading(),
                    ));
                } else {
                    ui.add(Separator::default().horizontal());
                }

                let offset = if index == 0 { turn.order_index } else { 0 };
                for &entity in turn.order.iter().skip(offset).take(display_slots) {
                    let (name, health, _, _) = warrior_query.get(entity).unwrap();
                    let color = if display_slots == 8 {
                        egui::Color32::LIGHT_GREEN
                    } else {
                        egui::Color32::WHITE
                    };

                    ui.add(Label::new(
                        RichText::new(name.as_str()).color(color).strong(),
                    ));
                    ui.add(
                        ProgressBar::new(health.0.value as f32 / health.0.max as f32)
                            .text(format!("{} / {} hp", health.0.value, health.0.max)),
                    );

                    display_slots -= 1;
                }

                index += 1;
            }
        });

    egui::containers::Area::new("next_turn_button")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-20.0, -20.0])
        .show(egui_context.ctx_mut(), |ui| {
            let mut style = ui.style_mut();
            style.spacing.button_padding = [15.0, 15.0].into();

            let warrior_entity = turn.get_current_warrior_entity().unwrap();
            let (_, _, ap, mp) = warrior_query.get(warrior_entity).unwrap();

            let ap_text = RichText::new(ap.0.value.to_string())
                .strong()
                .heading()
                .color(egui::Color32::BLACK);

            ui.add(egui::Button::new(ap_text).fill(egui::Color32::from_rgb(101, 88, 245)));

            let mp_text = RichText::new(mp.0.value.to_string())
                .strong()
                .heading()
                .color(egui::Color32::BLACK);

            ui.add(egui::Button::new(mp_text).fill(egui::Color32::from_rgb(26, 174, 159)));

            let end_turn_text = RichText::new("End turn")
                .strong()
                .heading()
                .color(egui::Color32::BLACK);
            if ui
                .add(egui::Button::new(end_turn_text).fill(egui::Color32::from_rgb(247, 195, 37)))
                .clicked()
            {
                ev_turn_ended.send(TurnEnd(turn.get_current_warrior_entity().unwrap()));
                turn.set_next();
                ev_turn_started.send(TurnStart(turn.get_current_warrior_entity().unwrap()));
            }
        });
}
