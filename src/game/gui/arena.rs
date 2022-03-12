use super::icon_index;
use super::widgets::*;
use super::IconCollection;
use crate::game::color;
use crate::game::gameplay::*;
use crate::game::map::*;
use bevy::ecs::prelude::*;
use bevy::input::prelude::*;
use bevy::prelude::Camera;
use bevy::prelude::GlobalTransform;
use bevy::prelude::Name;
use bevy::window::Windows;
use bevy_egui::egui::*;
use bevy_egui::EguiContext;

/// Display all infos about the turn system in a dedicated window
pub fn show_turn_ui(
    turn: Res<Turn>,
    warrior_query: Query<(&Name, &Attribute<Health>, &Attribute<Shield>), With<Warrior>>,
    mut egui_context: ResMut<EguiContext>,
    team_query: Query<&Team, With<Warrior>>,
) {
    Window::new("turn_order")
        .anchor(Align2::RIGHT_TOP, [-20.0, 20.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            Frame::default()
                .margin((10.0, 10.0))
                .fill(Color32::from_white_alpha(0))
                .stroke(Stroke::none())
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
                    let (name, health, shield) = warrior_query.get(entity).unwrap();
                    let color = team_query.get(entity).unwrap().color();
                    let stroke = if index == 0 && display_slots == turn.order.len() {
                        Stroke::new(2.0, color::HIGHLIGHT_BORDER)
                    } else {
                        Stroke::none()
                    };

                    Frame::default()
                        .corner_radius(5.0)
                        .margin((8.0, 8.0))
                        .stroke(stroke)
                        .fill(color::DEFAULT_BG.into())
                        .show(ui, |ui| {
                            ui.label(RichText::new(name.as_str()).color(color).strong());

                            ui.add(
                                LightProgressBar::new(health.as_percentage())
                                    .fg_color(color::HEALTH)
                                    .bg_color(color::DEFAULT_BG_LIGHTER)
                                    .text_color(color::TEXT_LIGHT)
                                    .custom_text(health.as_text())
                                    .desired_height(12.0)
                                    .corner_radius(2.0),
                            );

                            ui.add(
                                LightProgressBar::new(shield.as_percentage())
                                    .fg_color(color::SHIELD)
                                    .bg_color(color::DEFAULT_BG_LIGHTER)
                                    .text_color(color::TEXT_LIGHT)
                                    .custom_text(shield.value().to_string())
                                    .desired_height(12.0)
                                    .corner_radius(2.0),
                            );
                        });

                    display_slots -= 1;
                }

                index += 1;
            }

            ui.add(Label::new(
                RichText::new(format!("turn {}", turn.current + 1)).color(color::BG_TEXT),
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
    Window::new("next_turn_button")
        .anchor(Align2::RIGHT_BOTTOM, [-20.0, -20.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .default_width(200.0)
        .frame(
            Frame::default()
                .margin((10.0, 10.0))
                .fill(Color32::from_white_alpha(0))
                .stroke(Stroke::none()),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let mut style = ui.style_mut();
            style.spacing.button_padding = [15.0, 15.0].into();
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                let end_turn_text = RichText::new("⮫ End turn")
                    .strong()
                    .heading()
                    .color(Color32::BLACK);

                let entity = turn.get_current_warrior_entity().unwrap();
                let color = team_query.get(entity).unwrap().color();
                let is_enabled = !turn.is_changed();
                let end_turn_button = ui.add_enabled(
                    is_enabled,
                    Button::new(end_turn_text)
                        .fill(color)
                        .stroke(Stroke::new(2.0, color::HIGHLIGHT_BORDER)),
                );

                if end_turn_button.clicked() {
                    turn.set_next(ev_turn_started, ev_turn_ended);
                }

                let timer_percentage = turn_timer.0.percent_left();
                ui.add(
                    LightProgressBar::new(timer_percentage)
                        .desired_height(6.)
                        .fg_color(color)
                        .bg_color(color::DEFAULT_BG_LIGHTER)
                        .bg_stroke(Stroke::new(1.0, color::HIGHLIGHT_BORDER)),
                );
            });
        });
}

pub fn show_health_bar_ui(
    mut egui_context: ResMut<EguiContext>,
    turn: Res<Turn>,
    warrior_query: Query<&Attribute<Health>, With<Warrior>>,
) {
    Window::new("health_bar")
        .anchor(Align2::CENTER_BOTTOM, [0.0, -120.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            Frame::default()
                .margin((10.0, 10.0))
                .fill(Color32::from_white_alpha(0))
                .stroke(Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let entity = turn.get_current_warrior_entity().unwrap();
            let health = warrior_query.get(entity).unwrap();

            ui.visuals_mut().selection.bg_fill = color::HEALTH.into();
            ui.add(
                LightProgressBar::new(health.as_percentage())
                    .fg_color(color::HEALTH)
                    .bg_color(color::DEFAULT_BG_LIGHTER)
                    .custom_text(RichText::new(health.as_text()).color(color::BG_TEXT))
                    .text_align(LightProgressBarTextAlign::Center)
                    .desired_width(500.0)
                    .desired_height(20.0)
                    .corner_radius(5.0),
            );
        });
}

pub fn show_action_points_ui(
    mut egui_context: ResMut<EguiContext>,
    turn: Res<Turn>,
    warrior_query: Query<&Attribute<ActionPoints>, With<Warrior>>,
) {
    Window::new("action_points")
        .anchor(Align2::CENTER_BOTTOM, [-280.0, -78.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            Frame::default()
                .margin((10.0, 10.0))
                .fill(color::ACTION_POINTS.into())
                .stroke(Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let entity = turn.get_current_warrior_entity().unwrap();
            let action_points = warrior_query.get(entity).unwrap();
            let text = RichText::new(format!("★ {}", action_points.value()))
                .strong()
                .heading()
                .color(Color32::BLACK);

            ui.add(Label::new(text));
        });
}

pub fn show_movement_points_ui(
    mut egui_context: ResMut<EguiContext>,
    turn: Res<Turn>,
    warrior_query: Query<&Attribute<MovementPoints>, With<Warrior>>,
) {
    Window::new("movement_points")
        .anchor(Align2::CENTER_BOTTOM, [280.0, -78.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            Frame::default()
                .margin((10.0, 10.0))
                .fill(color::MOVEMENT_POINTS.into())
                .stroke(Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let entity = turn.get_current_warrior_entity().unwrap();
            let movement_points = warrior_query.get(entity).unwrap();
            let text = RichText::new(format!("🏃 {}", movement_points.value()))
                .strong()
                .heading()
                .color(Color32::BLACK);

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
    Window::new("action_bar")
        .anchor(Align2::CENTER_BOTTOM, [0.0, -20.0])
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .frame(
            Frame::default()
                .margin((10.0, 10.0))
                .fill(color::DEFAULT_BG.into())
                .stroke(Stroke::none())
                .corner_radius(5.0),
        )
        .show(egui_context.ctx_mut(), |ui| {
            Grid::new("action_bar_grid")
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
                            ImageButton::new(
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
                            Layout::centered_and_justified(Direction::LeftToRight),
                            |ui| {
                                ui.add(Label::new(
                                    RichText::new((index + 1).to_string())
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
    buttons: Res<Input<MouseButton>>,
    turn: Res<Turn>,
    warrior_query: Query<&Attribute<ActionPoints>, With<Warrior>>,
) {
    if keys.just_pressed(KeyCode::Escape) || buttons.just_pressed(MouseButton::Right) {
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

/// Show a bubble on top of the head of warrior on hover
pub fn show_warrior_ui(
    turn: Res<Turn>,
    windows: Res<Windows>,
    mouse_position: Res<MouseMapPosition>,
    selected_action: Res<SelectedAction>,
    map_query: Query<&Map>,
    warrior_query: Query<
        (
            Entity,
            &Name,
            &Team,
            &Attribute<Health>,
            &Attribute<Shield>,
            &MapPosition,
        ),
        With<Warrior>,
    >,
    actions_query: Query<&Actions>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut egui_context: ResMut<EguiContext>,
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

        for (entity, name, team, health, shield, position) in warrior_query.iter() {
            if mouse_position.ne(position) {
                continue;
            }

            let world_position = position.to_xyz(map, &LayerIndex(4));

            if let Some(hover_position) =
                camera.world_to_screen(windows.as_ref(), camera_transform, world_position)
            {
                let color = team.color();
                let main_window = windows.get_primary().unwrap();
                Window::new(format!("warrior_mouse_hover_{}", entity.id()))
                    .collapsible(false)
                    .resizable(false)
                    .title_bar(false)
                    .fixed_size((150.0, 80.0))
                    .fixed_pos((
                        hover_position.x - 75.0,
                        (hover_position.y - main_window.height()) * -1.0 - 108.0, // egui coordinates system has not the same 0.0 as bevy (top left vs bottom left)
                    ))
                    .frame(
                        Frame::default()
                            .fill(color::DEFAULT_BG.into())
                            .stroke(Stroke::new(2.0, color::HIGHLIGHT_BORDER))
                            .margin((8.0, 8.0))
                            .corner_radius(5.0),
                    )
                    .show(egui_context.ctx_mut(), |ui| {
                        ui.label(RichText::new(name.as_str()).color(color).monospace());
                        ui.visuals_mut().selection.bg_fill = color::HEALTH.into();
                        ui.add(
                            LightProgressBar::new(health.as_percentage())
                                .fg_color(color::HEALTH)
                                .bg_color(color::DEFAULT_BG_LIGHTER)
                                .desired_height(4.0),
                        );
                        ui.add(
                            LightProgressBar::new(shield.as_percentage())
                                .fg_color(color::SHIELD)
                                .bg_color(color::DEFAULT_BG_LIGHTER)
                                .desired_height(4.0),
                        );

                        // Preview selected action consequences on the hovered warrior
                        if let Some(selected_action) = selected_action.0 {
                            let actions = actions_query
                                .get(turn.get_current_warrior_entity().unwrap())
                                .unwrap();
                            if let Some(action) = actions.0.get(selected_action) {
                                ui.separator();
                                action.show_effects_ui(ui);
                            }
                        }
                    });
            }
        }
    }
}
