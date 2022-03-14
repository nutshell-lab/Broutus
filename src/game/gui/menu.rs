use crate::game::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContext;

enum Menu {
    Main,
    Options,
}

impl Default for Menu {
    fn default() -> Self {
        Self::Main
    }
}

#[derive(Default)]
pub struct SelectedMenu(Menu);

pub fn show_main_menu(
    mut egui_context: ResMut<EguiContext>,
    mut game_state: ResMut<State<GameState>>,
    mut ew_exit: EventWriter<AppExit>,
    windows: Res<Windows>,
    mut local: Local<SelectedMenu>,
) {
    let window = windows.get_primary().unwrap();
    egui::Window::new("broutus")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .fixed_rect(egui::Rect::from_two_pos(
            (0., 0.).into(),
            (window.width(), window.height()).into(),
        ))
        .frame(
            egui::Frame::default()
                .stroke(egui::Stroke::none())
                .fill(egui::Color32::from_black_alpha(0)),
        )
        .show(egui_context.ctx_mut(), |ui| match local.0 {
            Menu::Main => {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.image(egui::TextureId::User(1), (768., 480.));

                        match *game_state.current() {
                            GameState::Menu => {
                                let start = ui.add(egui::ImageButton::new(
                                    egui::TextureId::User(2),
                                    (152., 47.),
                                ));

                                if start.clicked() {
                                    game_state.push(GameState::Arena).unwrap();
                                }
                            }
                            GameState::Paused => {
                                let start = ui.add(egui::ImageButton::new(
                                    egui::TextureId::User(3),
                                    (152., 47.),
                                ));

                                if start.clicked() {
                                    game_state.pop().unwrap();
                                }
                            }
                            _ => (),
                        }

                        ui.add_space(30.0);
                        let options = ui.add(egui::ImageButton::new(
                            egui::TextureId::User(4),
                            (203., 52.),
                        ));
                        if options.clicked() {
                            local.0 = Menu::Options;
                        }

                        ui.add_space(30.0);
                        let exit = ui.add(egui::ImageButton::new(
                            egui::TextureId::User(5),
                            (119., 54.),
                        ));
                        if exit.clicked() {
                            ew_exit.send(AppExit);
                        }
                    });
                });
            }
            Menu::Options => {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.image(egui::TextureId::User(1), (768., 480.));

                        let back = ui.add(egui::ImageButton::new(
                            egui::TextureId::User(6),
                            (186., 45.),
                        ));

                        if back.clicked() {
                            local.0 = Menu::Main;
                        }
                    });
                });
            }
        });
}
