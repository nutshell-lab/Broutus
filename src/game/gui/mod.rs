use super::gameplay::{IconCollection, PortraitCollection};
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub mod arena;
pub mod menu;
mod widgets;

#[derive(AssetCollection)]
pub struct StartupCollection {
    #[asset(path = "splash.png")]
    pub splash: Handle<Image>,

    #[asset(path = "main_menu_start.png")]
    pub start: Handle<Image>,

    #[asset(path = "main_menu_options.png")]
    pub options: Handle<Image>,

    #[asset(path = "main_menu_exit.png")]
    pub exit: Handle<Image>,
}

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
                    ui.image(egui::TextureId::User(0), (768., 480.));
                });
            });
        });
}

fn icon_index(collection: &Res<IconCollection>, key: &str) -> Option<egui::TextureId> {
    collection
        .get_index(key)
        .map(|i| egui::TextureId::User(i as u64 + 10))
}

fn portrait_index(collection: &Res<PortraitCollection>, key: &str) -> Option<egui::TextureId> {
    collection
        .get_index(key)
        .map(|i| egui::TextureId::User(i as u64 + 100))
}

/// Setup ui resources (like bind loaded textures)
pub fn map_gui_textures(
    mut egui_context: ResMut<EguiContext>,
    startup: Res<StartupCollection>,
    icons: Res<IconCollection>,
    portraits: Res<PortraitCollection>,
) {
    egui_context.set_egui_texture(0, startup.splash.clone());
    egui_context.set_egui_texture(1, startup.start.clone());
    egui_context.set_egui_texture(2, startup.options.clone());
    egui_context.set_egui_texture(3, startup.exit.clone());

    for (index, handle) in icons.get_all().iter().enumerate() {
        egui_context.set_egui_texture(10 + index as u64, handle.clone());
    }

    for (index, handle) in portraits.get_all().iter().enumerate() {
        egui_context.set_egui_texture(100 + index as u64, handle.clone());
    }
}
