use super::color;
use super::gameplay::*;
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_egui::egui;
use bevy_egui::egui::{Label, ProgressBar, RichText};
use bevy_egui::EguiContext;

#[derive(AssetCollection)]
pub struct ActionsAssets {
    #[asset(path = "actions/sword.png")]
    sword: Handle<Image>,
}

/// Display all infos about the turn system in a dedicated window
pub fn show_turn_ui(
    mut turn: ResMut<Turn>,
    mut ev_turn_started: EventWriter<TurnStart>,
    mut ev_turn_ended: EventWriter<TurnEnd>,
    mut egui_context: ResMut<EguiContext>,
    warrior_query: Query<(&Name, &Health, &ActionPoints, &MovementPoints), With<Warrior>>,
    mut team_query: QuerySet<(
        QueryState<Entity, With<TeamA>>,
        QueryState<Entity, With<TeamB>>,
    )>,
) {
    egui::containers::Window::new("turn_order")
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
            let mut display_slots = turn.order.len();
            let mut index = 0;

            ui.set_max_size([200.0, 1200.0].into());
            ui.visuals_mut().selection.bg_fill = color::HEALTH.into();

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

                    let stroke = if index == 0 && display_slots == turn.order.len() {
                        egui::Stroke::new(2.0, color::HIGHLIGHT_BORDER)
                    } else {
                        egui::Stroke::none()
                    };

                    egui::containers::Frame::default()
                        .corner_radius(5.0)
                        .margin((8.0, 8.0))
                        .stroke(stroke)
                        .fill(color::DEFAULT_BG.into())
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new(name.as_str()).color(color).strong());
                            ui.add(
                                ProgressBar::new(health.as_percentage()).text(
                                    egui::RichText::new(health.as_text()).color(color::BG_TEXT),
                                ),
                            );
                        });

                    display_slots -= 1;
                }

                index += 1;
            }

            ui.add(egui::Label::new(
                egui::RichText::new(format!("turn {}", turn.current + 1)).color(color::BG_TEXT),
            ));
        });

    egui::containers::Area::new("next_turn_button")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-20.0, -20.0])
        .show(egui_context.ctx_mut(), |ui| {
            let mut style = ui.style_mut();
            style.spacing.button_padding = [15.0, 15.0].into();

            let warrior_entity = turn.get_current_warrior_entity().unwrap();
            let (_, _, ap, mp) = warrior_query.get(warrior_entity).unwrap();

            egui::Resize::default()
                .default_width(140.0)
                .resizable(false)
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            egui::containers::Frame::default()
                                .corner_radius(5.0)
                                .fill(color::ACTION_POINTS.into())
                                .margin((10.0, 10.0))
                                .show(ui, |ui| {
                                    let ap_text = RichText::new(format!("★ {}", ap.0.value))
                                        .strong()
                                        .heading()
                                        .color(egui::Color32::BLACK);

                                    ui.add(Label::new(ap_text));
                                });

                            egui::containers::Frame::default()
                                .corner_radius(5.0)
                                .fill(color::MOVEMENT_POINTS.into())
                                .margin((10.0, 10.0))
                                .show(ui, |ui| {
                                    let mp_text = RichText::new(format!("🎠 {}", mp.0.value))
                                        .strong()
                                        .heading()
                                        .color(egui::Color32::BLACK);

                                    ui.add(Label::new(mp_text));
                                });

                            let end_turn_text = RichText::new("🕑 End turn")
                                .strong()
                                .heading()
                                .color(egui::Color32::BLACK);

                            let end_turn_button = egui::Button::new(end_turn_text)
                                .fill(color::END_TURN)
                                .stroke(egui::Stroke::new(2.0, color::HIGHLIGHT_BORDER));

                            if ui.add(end_turn_button).clicked() {
                                ev_turn_ended
                                    .send(TurnEnd(turn.get_current_warrior_entity().unwrap()));
                                turn.set_next();
                                ev_turn_started
                                    .send(TurnStart(turn.get_current_warrior_entity().unwrap()));
                            }
                        },
                    );
                });
        });
}

pub fn show_health_bar_ui(
    mut egui_context: ResMut<EguiContext>,
    turn: Res<Turn>,
    warrior_query: Query<&Health, With<Warrior>>,
) {
    egui::containers::Window::new("health_bar")
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -100.0])
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
            let entity = turn.get_current_warrior_entity().unwrap();
            let health = warrior_query.get(entity).unwrap();

            ui.visuals_mut().selection.bg_fill = color::HEALTH.into();
            ui.add(
                ProgressBar::new(health.as_percentage())
                    .text(egui::RichText::new(health.as_text()).color(color::BG_TEXT)),
            );
        });
}

pub fn show_action_bar_ui(
    mut egui_context: ResMut<EguiContext>,
    mut selected_action: ResMut<SelectedAction>,
    images: Res<ActionsAssets>,
) {
    egui_context.set_egui_texture(0, images.sword.clone_weak());

    egui::containers::Window::new("action_bar")
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -20.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            egui::containers::Frame::default()
                .margin((10.0, 10.0))
                .fill(color::DEFAULT_BG.into())
                .stroke(egui::Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            egui::Grid::new("action_bar_grid")
                .spacing((5.0, 5.0))
                .show(ui, |ui| {
                    // TODO show real actions
                    for index in 0..8usize {
                        if index > 0 && index % 8 == 0 {
                            ui.end_row();
                        }
                        let is_selected = selected_action
                            .0
                            .map(|selected| selected == index)
                            .unwrap_or(false);
                        let button = ui.add(
                            egui::ImageButton::new(egui::TextureId::User(0), (48.0, 48.0))
                                .selected(is_selected),
                        );

                        // Toggle action selection
                        if button.clicked() {
                            selected_action.0 = if is_selected { None } else { Some(index) };
                        }

                        // Display action details in a toolip on hover
                        if button.hovered() {
                            egui::show_tooltip(ui.ctx(), egui::Id::new("action_tooltip"), |ui| {
                                egui::Grid::new(format!("action_bar_grid_{}", index)).show(
                                    ui,
                                    |ui| {
                                        ui.label(egui::RichText::new("Sword").heading());
                                        ui.label(
                                            egui::RichText::new("★ 3")
                                                .heading()
                                                .color(color::ACTION_POINTS),
                                        );
                                        ui.end_row();
                                        ui.label(
                                            egui::RichText::new("15 dmg")
                                                .strong()
                                                .color(color::HEALTH),
                                        );
                                    },
                                )
                            });
                        }
                    }
                });
        });
}

/// Show battle logs window (scrollable)
pub fn show_battlelog_ui(mut egui_context: ResMut<EguiContext>) {
    egui::containers::Window::new("battlelogs")
        .anchor(egui::Align2::LEFT_BOTTOM, [20.0, -20.0])
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
                    // TODO show real battle logs
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

/// Show a bubble on top of the head of warrior on hover
pub fn show_warrior_ui(
    windows: Res<Windows>,
    tiledmaps: Res<Assets<Tiledmap>>,
    mouse_position: Res<MouseMapPosition>,
    map_query: Query<&Handle<Tiledmap>, With<Map>>,
    warrior_query: Query<(Entity, &Name, &Health, &MapPosition), With<Warrior>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut egui_context: ResMut<EguiContext>,
    mut team_query: QuerySet<(
        QueryState<Entity, With<TeamA>>,
        QueryState<Entity, With<TeamB>>,
    )>,
) {
    if map_query.is_empty() {
        return;
    }
    if warrior_query.is_empty() {
        return;
    }

    if let Some(mouse_position) = mouse_position.0 {
        let tiledmap_handle = map_query.single();
        let tiledmap = tiledmaps.get(tiledmap_handle).unwrap();
        let (camera, camera_transform) = camera_query.single();

        for (entity, name, health, position) in warrior_query.iter() {
            if mouse_position.ne(position) {
                continue;
            }

            let world_position = position.to_xyz(
                0u32,
                tiledmap.inner.width,
                tiledmap.inner.height,
                tiledmap.inner.tile_width as f32,
                tiledmap.inner.tile_height as f32,
            );

            if let Some(hover_position) =
                camera.world_to_screen(windows.as_ref(), camera_transform, world_position)
            {
                let color = {
                    let is_team_a = team_query.q0().get(entity).is_ok();
                    let is_team_b = team_query.q1().get(entity).is_ok();

                    if is_team_a {
                        color::TEAM_A_COLOR.into()
                    } else if is_team_b {
                        color::TEAM_B_COLOR.into()
                    } else {
                        egui::Color32::LIGHT_GREEN
                    }
                };

                let main_window = windows.get_primary().unwrap();
                egui::containers::Window::new("warrior_mouse_hover")
                    .collapsible(false)
                    .resizable(false)
                    .title_bar(false)
                    .fixed_size((150.0, 80.0))
                    .fixed_pos((
                        hover_position.x - 75.0,
                        (hover_position.y - main_window.height()) * -1.0 - 108.0, // egui coordinates system has not the same 0.0 as bevy (top left vs bottom left)
                    ))
                    .frame(
                        egui::containers::Frame::default()
                            .fill(color::DEFAULT_BG.into())
                            .stroke(egui::Stroke::new(2.0, color::HIGHLIGHT_BORDER))
                            .margin((8.0, 8.0))
                            .corner_radius(5.0),
                    )
                    .show(egui_context.ctx_mut(), |ui| {
                        ui.label(egui::RichText::new(name.as_str()).color(color).heading());
                        ui.visuals_mut().selection.bg_fill = color::HEALTH.into();
                        ui.add(
                            ProgressBar::new(health.as_percentage())
                                .text(egui::RichText::new(health.as_text()).color(color::BG_TEXT)),
                        );
                    });
            }
        }
    }
}
