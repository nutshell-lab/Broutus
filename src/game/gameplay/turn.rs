use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::egui::{
    widgets::{Label, ProgressBar},
    RichText,
};

use super::super::color;
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

    pub fn get_entity_index(&self, entity: Entity) -> Option<usize> {
        self.order.iter().position(|e| e.eq(&entity))
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
    mut team_query: QuerySet<(
        QueryState<Entity, With<TeamA>>,
        QueryState<Entity, With<TeamB>>,
    )>,
) {
    egui::containers::Window::new("TurnOrder")
        .anchor(egui::Align2::RIGHT_TOP, [-20.0, 20.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            egui::containers::Frame::default()
                .margin((10.0, 10.0))
                .fill(egui::Color32::from_white_alpha(0))
                .stroke(egui::Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let mut display_slots = 8;
            let mut index = 0;

            ui.set_max_size([200.0, 1200.0].into());
            ui.visuals_mut().selection.bg_fill = egui::Color32::from_rgb(231, 76, 60);

            while display_slots > 0 {
                let offset = if index == 0 { turn.order_index } else { 0 };
                for &entity in turn.order.iter().skip(offset).take(display_slots) {
                    let (name, health, _, _) = warrior_query.get(entity).unwrap();
                    let color = {
                        let is_team_a = team_query.q0().get(entity).is_ok();
                        let is_team_b = team_query.q1().get(entity).is_ok();

                        if is_team_a {
                            color::TEAM_A_COLOR
                        } else if is_team_b {
                            color::TEAM_B_COLOR
                        } else {
                            color::TEAM_SPEC_COLOR
                        }
                    };

                    let stroke = if index == 0 && display_slots == 8 {
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(138, 7, 70))
                    } else {
                        egui::Stroke::none()
                    };

                    egui::containers::Frame::default()
                        .corner_radius(5.0)
                        .margin((8.0, 8.0))
                        .stroke(stroke)
                        .fill(egui::Color32::from_rgb(44, 47, 51))
                        .show(ui, |ui| {
                            ui.add(Label::new(
                                RichText::new(name.as_str()).color(color).strong(),
                            ));
                            ui.add(
                                ProgressBar::new(health.0.value as f32 / health.0.max as f32)
                                    .text(format!("{} / {} hp", health.0.value, health.0.max)),
                            );
                        });

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

            egui::containers::Frame::default()
                .corner_radius(2.0)
                .fill(color::ACTION_POINTS.into())
                .margin((34.0, 15.0))
                .show(ui, |ui| {
                    let ap_text = RichText::new(format!("★ {}", ap.0.value))
                        .strong()
                        .heading()
                        .color(egui::Color32::BLACK);

                    ui.add(Label::new(ap_text));
                });

            egui::containers::Frame::default()
                .corner_radius(2.0)
                .fill(color::MOVEMENT_POINTS.into())
                .margin((33.0, 15.0))
                .show(ui, |ui| {
                    let mp_text = RichText::new(format!("🎠 {}", mp.0.value))
                        .strong()
                        .heading()
                        .color(egui::Color32::BLACK);

                    ui.add(Label::new(mp_text));
                });

            let end_turn_text = RichText::new("End turn")
                .strong()
                .heading()
                .color(egui::Color32::BLACK);
            if ui
                .add(egui::Button::new(end_turn_text).fill(color::END_TURN))
                .clicked()
            {
                ev_turn_ended.send(TurnEnd(turn.get_current_warrior_entity().unwrap()));
                turn.set_next();
                ev_turn_started.send(TurnStart(turn.get_current_warrior_entity().unwrap()));
            }
        });

    egui::containers::Window::new("Battlelogs")
        .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -10.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .fixed_size((300.0, 200.0))
        .frame(
            egui::containers::Frame::default()
                .fill(egui::Color32::WHITE)
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            egui::containers::Frame::default()
                .fill(egui::Color32::from_rgb(231, 76, 60))
                .margin((105.0, 10.0))
                .corner_radius(5.0)
                .show(ui, |ui| {
                    let text = RichText::new("Battlelogs")
                        .strong()
                        .heading()
                        .color(egui::Color32::WHITE);

                    ui.add(Label::new(text));
                });

            egui::containers::Frame::default()
                .fill(egui::Color32::WHITE)
                .margin((10.0, 10.0))
                .show(ui, |ui| {
                    let text_style = egui::TextStyle::Body;
                    let row_height = ui.fonts()[text_style].row_height();
                    let num_rows = 10_000;
                    egui::ScrollArea::vertical().stick_to_bottom().show_rows(
                        ui,
                        row_height,
                        num_rows,
                        |ui, row_range| {
                            for row in row_range {
                                let text = format!("Row {}/{}", row + 1, num_rows);
                                ui.label(text);
                            }
                        },
                    );
                });
        });
}
