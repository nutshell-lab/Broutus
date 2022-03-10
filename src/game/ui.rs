use super::color;
use super::gameplay::*;
use super::map::Map;
use super::map::MapPosition;
use super::map::MouseMapPosition;
use super::GameState;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::egui::{Label, ProgressBar, RichText};
use bevy_egui::EguiContext;

fn icon_index(collection: &Res<IconCollection>, key: &str) -> Option<egui::TextureId> {
    collection
        .get_index(key)
        .map(|i| egui::TextureId::User(i as u64 + 1))
}

fn portrait_index(collection: &Res<PortraitCollection>, key: &str) -> Option<egui::TextureId> {
    collection
        .get_index(key)
        .map(|i| egui::TextureId::User(i as u64 + 100))
}

/// Setup ui resources (like bind loaded textures)
pub fn setup_ui(
    mut egui_context: ResMut<EguiContext>,
    icon_collection: Res<IconCollection>,
    portraits_collection: Res<PortraitCollection>,
) {
    for (index, icon) in icon_collection.get_all().iter().enumerate() {
        egui_context.set_egui_texture(1 + index as u64, icon.clone());
    }

    for (index, icon) in portraits_collection.get_all().iter().enumerate() {
        egui_context.set_egui_texture(100 + index as u64, icon.clone());
    }
}

/// Print team the selection screen
pub fn show_warrior_selection_ui(
    mut egui_context: ResMut<EguiContext>,
    mut game_state: ResMut<State<GameState>>,
    windows: Res<Windows>,
    warriors: Res<Assets<WarriorAsset>>,
    warrior_collection: Res<WarriorCollection>,
    icon_collection: Res<IconCollection>,
    portraits_collection: Res<PortraitCollection>,
) {
    let window = windows.get_primary().unwrap();

    egui::containers::Window::new("warrior_selection")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .fixed_rect(egui::Rect::from_two_pos(
            egui::pos2(0., 0.),
            egui::pos2(window.width(), window.height()),
        ))
        .frame(
            egui::containers::Frame::default()
                .margin((10.0, 10.0))
                .fill(egui::Color32::from_white_alpha(0))
                .stroke(egui::Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            egui::TopBottomPanel::top("warrior_selection_top_panel")
                .resizable(false)
                .min_height(100.)
                .frame(
                    egui::containers::Frame::default()
                        .margin((10.0, 10.0))
                        .fill(egui::Color32::from_white_alpha(0))
                        .stroke(egui::Stroke::none())
                        .corner_radius(5.0),
                )
                .show_inside(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new("DRAFT").monospace().heading(),
                        ));
                    })
                });

            egui::TopBottomPanel::bottom("warrior_selection_bottom_panel")
                .resizable(false)
                .min_height(100.)
                .frame(
                    egui::containers::Frame::default()
                        .margin((10.0, 10.0))
                        .fill(egui::Color32::from_white_alpha(0))
                        .stroke(egui::Stroke::none())
                        .corner_radius(5.0),
                )
                .show_inside(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        if ui.button("Play").clicked() {
                            game_state.set(GameState::Arena).unwrap();
                        }
                    })
                });

            let mut warrior_index = None;

            egui::SidePanel::left("warrior_selection_left_panel")
                .resizable(false)
                .min_width(200.)
                .frame(
                    egui::containers::Frame::default()
                        .margin((10.0, 10.0))
                        .fill(egui::Color32::from_white_alpha(0))
                        .stroke(egui::Stroke::none())
                        .corner_radius(5.0),
                )
                .show_inside(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        for (index, warrior_handle) in
                            warrior_collection.warriors.iter().enumerate()
                        {
                            ui.vertical(|ui| {
                                egui::Frame::default().show(ui, |ui| {
                                    if let Some(warrior) = warriors.get(warrior_handle) {
                                        if ui
                                            .image(
                                                portrait_index(
                                                    &portraits_collection,
                                                    warrior.portrait_key.as_str(),
                                                )
                                                .unwrap(),
                                                (81.25, 92.5),
                                            )
                                            .hovered()
                                        {
                                            warrior_index = Some(index);
                                        }
                                    }
                                });
                            });
                        }
                    });
                });

            egui::SidePanel::right("warrior_selection_right_panel")
                .resizable(false)
                .min_width(500.)
                .frame(
                    egui::containers::Frame::default()
                        .margin((10.0, 10.0))
                        .fill(egui::Color32::from_white_alpha(0))
                        .stroke(egui::Stroke::none())
                        .corner_radius(5.0),
                )
                .show_inside(ui, |ui| {
                    if let Some(warrior_index) = warrior_index {
                        let warrior_handle =
                            warrior_collection.warriors.get(warrior_index).unwrap();
                        let warrior = warriors.get(warrior_handle).unwrap();

                        ui.image(
                            portrait_index(&portraits_collection, warrior.portrait_key.as_str())
                                .unwrap(),
                            (162.5, 185.),
                        );

                        ui.label(RichText::new(warrior.name.as_str()).heading());

                        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                            for action in warrior.actions.0.iter() {
                                ui.image(
                                    icon_index(&icon_collection, action.icon_key.as_str()).unwrap(),
                                    (64., 64.),
                                );

                                ui.label(RichText::new(action.name.as_str()).monospace());

                                action.aoe.show_description_ui(ui);
                                action.range.show_description_ui(ui);
                                for effect in action.effects.iter() {
                                    effect.show_description_ui(ui);
                                }
                            }
                        });
                    }
                });
        });
}

/// Display all infos about the turn system in a dedicated window
pub fn show_turn_ui(
    turn: Res<Turn>,
    warrior_query: Query<
        (
            &Name,
            &Attribute<Health>,
            &Attribute<ActionPoints>,
            &Attribute<MovementPoints>,
        ),
        With<Warrior>,
    >,
    mut egui_context: ResMut<EguiContext>,
    team_query: Query<&Team, With<Warrior>>,
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
                    let color = team_query.get(entity).unwrap().color();
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
}

pub fn show_turn_button_ui(
    mut turn: ResMut<Turn>,
    turn_timer: Res<TurnTimer>,
    ev_turn_started: EventWriter<TurnStart>,
    ev_turn_ended: EventWriter<TurnEnd>,
    mut egui_context: ResMut<EguiContext>,
    team_query: Query<&Team, With<Warrior>>,
) {
    egui::containers::Window::new("next_turn_button")
        .anchor(egui::Align2::RIGHT_BOTTOM, [-20.0, -20.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .default_width(200.0)
        .frame(
            egui::containers::Frame::default()
                .margin((10.0, 10.0))
                .fill(egui::Color32::from_white_alpha(0))
                .stroke(egui::Stroke::none()),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let mut style = ui.style_mut();
            style.spacing.button_padding = [15.0, 15.0].into();
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    let end_turn_text = RichText::new("⮫ End turn")
                        .strong()
                        .heading()
                        .color(egui::Color32::BLACK);

                    let entity = turn.get_current_warrior_entity().unwrap();
                    let color = team_query.get(entity).unwrap().color();
                    let is_enabled = !turn.is_changed();
                    let end_turn_button = ui.add_enabled(
                        is_enabled,
                        egui::Button::new(end_turn_text)
                            .fill(color)
                            .stroke(egui::Stroke::new(2.0, color::HIGHLIGHT_BORDER)),
                    );

                    if end_turn_button.clicked() {
                        turn.set_next(ev_turn_started, ev_turn_ended);
                    }

                    let timer_percentage = turn_timer.0.percent_left();
                    ui.visuals_mut().selection.bg_fill = color.into();
                    ui.visuals_mut().extreme_bg_color = color::DEFAULT_BG.into();
                    ui.add(egui::ProgressBar::new(timer_percentage));
                },
            );
        });
}

pub fn show_health_bar_ui(
    mut egui_context: ResMut<EguiContext>,
    turn: Res<Turn>,
    warrior_query: Query<&Attribute<Health>, With<Warrior>>,
) {
    egui::containers::Window::new("health_bar")
        .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -120.0])
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
                    .text(egui::RichText::new(health.as_text()).color(color::BG_TEXT))
                    .desired_width(500.0),
            );
        });
}

pub fn show_action_points_ui(
    mut egui_context: ResMut<EguiContext>,
    turn: Res<Turn>,
    warrior_query: Query<&Attribute<ActionPoints>, With<Warrior>>,
) {
    egui::containers::Window::new("action_points")
        .anchor(egui::Align2::CENTER_BOTTOM, [-280.0, -78.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            egui::containers::Frame::default()
                .margin((10.0, 10.0))
                .fill(color::ACTION_POINTS.into())
                .stroke(egui::Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let entity = turn.get_current_warrior_entity().unwrap();
            let action_points = warrior_query.get(entity).unwrap();
            let text = RichText::new(format!("★ {}", action_points.value()))
                .strong()
                .heading()
                .color(egui::Color32::BLACK);

            ui.add(Label::new(text));
        });
}

pub fn show_movement_points_ui(
    mut egui_context: ResMut<EguiContext>,
    turn: Res<Turn>,
    warrior_query: Query<&Attribute<MovementPoints>, With<Warrior>>,
) {
    egui::containers::Window::new("movement_points")
        .anchor(egui::Align2::CENTER_BOTTOM, [280.0, -78.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            egui::containers::Frame::default()
                .margin((10.0, 10.0))
                .fill(color::MOVEMENT_POINTS.into())
                .stroke(egui::Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let entity = turn.get_current_warrior_entity().unwrap();
            let movement_points = warrior_query.get(entity).unwrap();
            let text = RichText::new(format!("🏃 {}", movement_points.value()))
                .strong()
                .heading()
                .color(egui::Color32::BLACK);

            ui.add(Label::new(text));
        });
}

pub fn show_action_bar_ui(
    mut egui_context: ResMut<EguiContext>,
    mut selected_action: ResMut<SelectedAction>,
    icon_collection: Res<IconCollection>,
    turn: Res<Turn>,
    warrior_query: Query<(&Actions, &Attribute<ActionPoints>), With<Warrior>>,
) {
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
                .show(ui, |mut ui| {
                    let entity = turn.get_current_warrior_entity().unwrap();
                    let (actions, action_points) = warrior_query.get(entity).unwrap();
                    for (index, action) in actions.0.iter().enumerate() {
                        if index > 0 && index % 8 == 0 {
                            ui.end_row();
                        }

                        let is_selected = selected_action
                            .0
                            .map(|selected| selected == index)
                            .unwrap_or(false);

                        let enabled = action_points.can_drop(action.cost.value());
                        let button = ui.add_enabled(
                            enabled,
                            egui::ImageButton::new(
                                icon_index(&icon_collection, action.icon_key.as_str()).unwrap(),
                                (48.0, 48.0),
                            )
                            .selected(is_selected),
                        );

                        // Toggle action selection
                        if button.clicked() && enabled {
                            selected_action.0 = if is_selected { None } else { Some(index) };
                        }

                        // Display action details in a toolip on hover
                        if button.hovered() {
                            action.show_tooltip_ui(&mut ui);
                        }
                    }

                    // Show keybindigs below
                    ui.end_row();
                    for index in 0..actions.0.len() {
                        ui.with_layout(
                            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                ui.add(egui::Label::new(
                                    egui::RichText::new((index + 1).to_string())
                                        .small()
                                        .color(color::BG_TEXT),
                                ));
                            },
                        );
                    }
                });
        });
}

pub fn handle_action_bar_shortcuts(
    mut selected_action: ResMut<SelectedAction>,
    keys: Res<Input<KeyCode>>,
    turn: Res<Turn>,
    warrior_query: Query<&Attribute<ActionPoints>, With<Warrior>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        selected_action.0 = None;
    }

    let entity = turn.get_current_warrior_entity().unwrap();
    let action_points = warrior_query.get(entity).unwrap();
    let is_disabled = action_points.value() < 3; // TODO replace by the real action cost, for each action

    if is_disabled {
        return;
    }

    // TODO switch to ScanCode to be layout agnostic
    // see: https://bevy-cheatbook.github.io/input/keyboard.html#layout-agnostic-key-bindings
    if keys.just_pressed(KeyCode::Key1) {
        selected_action.0 = Some(0);
    }

    if keys.just_pressed(KeyCode::Key2) {
        selected_action.0 = Some(1);
    }

    if keys.just_pressed(KeyCode::Key3) {
        selected_action.0 = Some(2);
    }

    if keys.just_pressed(KeyCode::Key4) {
        selected_action.0 = Some(3);
    }

    if keys.just_pressed(KeyCode::Key5) {
        selected_action.0 = Some(4);
    }

    if keys.just_pressed(KeyCode::Key6) {
        selected_action.0 = Some(5);
    }

    if keys.just_pressed(KeyCode::Key7) {
        selected_action.0 = Some(6);
    }

    if keys.just_pressed(KeyCode::Key8) {
        selected_action.0 = Some(7);
    }
}

/// Show battle logs window (scrollable)
pub fn show_battlelog_ui(mut egui_context: ResMut<EguiContext>) {
    egui::containers::Area::new("battlelogs")
        .anchor(egui::Align2::LEFT_BOTTOM, [20.0, -20.0])
        .show(egui_context.ctx_mut(), |ui| {
            egui::containers::Frame::default()
                .fill(color::WHITE_BG.into())
                .corner_radius(5.0)
                .show(ui, |ui| {
                    egui::containers::Frame::default()
                        .fill(color::HEALTH.into())
                        .corner_radius(4.0)
                        .show(ui, |ui| {
                            egui::Resize::default()
                                .default_width(300.0)
                                .default_height(30.0)
                                .resizable(false)
                                .show(ui, |ui| {
                                    let text = RichText::new("Battlelogs")
                                        .strong()
                                        .heading()
                                        .color(egui::Color32::WHITE);

                                    ui.with_layout(
                                        egui::Layout::centered_and_justified(
                                            egui::Direction::LeftToRight,
                                        ),
                                        |ui| {
                                            ui.add(Label::new(text));
                                        },
                                    )
                                });
                        });

                    egui::Resize::default()
                        .default_width(300.0)
                        .default_height(200.0)
                        .resizable(false)
                        .show(ui, |ui| {
                            let text_style = egui::TextStyle::Body;
                            let row_height = ui.fonts()[text_style].row_height();
                            let num_rows = 1_000;
                            egui::ScrollArea::vertical().stick_to_bottom().show_rows(
                                ui,
                                row_height * 2.0,
                                num_rows,
                                |ui, row_range| {

                                    for _ in row_range {
                                        let message = egui::RichText::new("system: Brunto attacked Glurf with a Rusty Sword for 32 damages.").strong().color(color::DEFAULT_BG);
                                            ui.label(message);
                                    }
                                },
                            );
                        });
                });
        });
}

/// Show a bubble on top of the head of warrior on hover
pub fn show_warrior_ui(
    windows: Res<Windows>,
    mouse_position: Res<MouseMapPosition>,
    selected_action: Res<SelectedAction>,
    map_query: Query<&Map>,
    warrior_query: Query<(Entity, &Name, &Attribute<Health>, &MapPosition), With<Warrior>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut egui_context: ResMut<EguiContext>,
    team_query: Query<&Team, With<Warrior>>,
) {
    if map_query.is_empty() {
        return;
    }
    if warrior_query.is_empty() {
        return;
    }

    if let Some(mouse_position) = mouse_position.0 {
        let map = map_query.single();
        let (camera, camera_transform) = camera_query.single();

        for (entity, name, health, position) in warrior_query.iter() {
            if mouse_position.ne(position) {
                continue;
            }

            let world_position = position.to_xyz(
                0u32,
                map.width,
                map.height,
                map.tile_width as f32,
                map.tile_height as f32,
            );

            if let Some(hover_position) =
                camera.world_to_screen(windows.as_ref(), camera_transform, world_position)
            {
                let color = team_query.get(entity).unwrap().color();
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

                        // Preview selected action consequences on the hovered warrior
                        if selected_action.0.is_some() {
                            ui.separator();
                            ui.label(
                                egui::RichText::new("Slash")
                                    .color(color::ACTION_POINTS)
                                    .text_style(egui::TextStyle::Button),
                            );
                            ui.label(
                                egui::RichText::new("-15 health")
                                    .color(color::HEALTH)
                                    .strong(),
                            );
                        }
                    });
            }
        }
    }
}
