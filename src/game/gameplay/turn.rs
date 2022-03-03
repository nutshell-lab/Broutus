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

pub const TEAM_A_COLOR: Color = Color::rgb(23.0 / 255.0, 169.0 / 255.0, 250.0 / 255.0);
pub const TEAM_B_COLOR: Color = Color::rgb(250.0 / 255.0, 104.0 / 255.0, 23.0 / 255.0);

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
    mut team_query: QuerySet<(
        QueryState<Entity, With<TeamA>>,
        QueryState<Entity, With<TeamB>>,
    )>,
) {
    egui::containers::Area::new("turn_order")
        .anchor(egui::Align2::RIGHT_TOP, [-20.0, 20.0])
        .movable(false)
        .show(egui_context.ctx_mut(), |ui| {
            let mut display_slots = 8;
            let mut index = 0;

            ui.set_max_size([200.0, 1200.0].into());
            ui.visuals_mut().selection.bg_fill = egui::Color32::from_rgb(231, 76, 60);

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
                    let color = {
                        let is_team_a = team_query.q0().get(entity).is_ok();
                        let is_team_b = team_query.q1().get(entity).is_ok();

                        if is_team_a {
                            egui::Color32::from_rgb(
                                (TEAM_A_COLOR.r() * 255.0).round() as u8,
                                (TEAM_A_COLOR.g() * 255.0).round() as u8,
                                (TEAM_A_COLOR.b() * 255.0).round() as u8,
                            )
                        } else if is_team_b {
                            egui::Color32::from_rgb(
                                (TEAM_B_COLOR.r() * 255.0).round() as u8,
                                (TEAM_B_COLOR.g() * 255.0).round() as u8,
                                (TEAM_B_COLOR.b() * 255.0).round() as u8,
                            )
                        } else {
                            egui::Color32::LIGHT_GREEN
                        }
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

            egui::containers::Frame::default()
                .corner_radius(2.0)
                .fill(egui::Color32::from_rgb(101, 88, 245))
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
                .fill(egui::Color32::from_rgb(26, 174, 159))
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
                .add(egui::Button::new(end_turn_text).fill(egui::Color32::from_rgb(247, 195, 37)))
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
