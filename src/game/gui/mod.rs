use super::gameplay::{IconCollection, PortraitCollection};
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub mod arena;
mod widgets;

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
