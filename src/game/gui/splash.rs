use super::StartupCollection;
use bevy::ecs::prelude::*;
use bevy::window::Windows;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub fn show_splash(
    mut egui_context: ResMut<EguiContext>,
    collection: Res<StartupCollection>,
    windows: Res<Windows>,
) {
    egui_context.set_egui_texture(0, collection.splash.clone());

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
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.image(egui::TextureId::User(0), (372., 73.));
                });
            });
        });
}
