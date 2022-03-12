use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_egui::egui;
use bevy_egui::EguiContext;

pub mod arena;
pub mod menu;
pub mod splash;
mod widgets;

#[derive(AssetCollection)]
pub struct StartupCollection {
    #[asset(path = "splash.png")]
    pub splash: Handle<Image>,

    #[asset(path = "title.png")]
    pub title: Handle<Image>,

    #[asset(path = "main_menu_start.png")]
    pub start: Handle<Image>,

    #[asset(path = "main_menu_options.png")]
    pub options: Handle<Image>,

    #[asset(path = "main_menu_exit.png")]
    pub exit: Handle<Image>,
}

// TODO find a way to load a folder into as HashMap<String, Handle<..>>
#[derive(AssetCollection, Reflect)]
pub struct IconCollection {
    #[asset(key = "icons.action_blind")]
    pub action_blind: Handle<Image>,

    #[asset(key = "icons.action_cripple")]
    pub action_cripple: Handle<Image>,

    #[asset(key = "icons.action_heal")]
    pub action_heal: Handle<Image>,

    #[asset(key = "icons.action_push")]
    pub action_push: Handle<Image>,

    #[asset(key = "icons.action_shield")]
    pub action_shield: Handle<Image>,

    #[asset(key = "icons.action_shoot")]
    pub action_shoot: Handle<Image>,

    #[asset(key = "icons.action_slash")]
    pub action_slash: Handle<Image>,

    #[asset(key = "icons.action_teleport")]
    pub action_teleport: Handle<Image>,
}

impl IconCollection {
    /// Get an image handle giving an icon key
    pub fn get(&self, key: &str) -> Option<Handle<Image>> {
        self.field(key)
            .and_then(|field| field.downcast_ref::<Handle<Image>>())
            .cloned()
    }

    pub fn get_index(&self, key: &str) -> Option<usize> {
        if let Some(right) = self.get(key) {
            self.get_all().iter().position(|left| left.eq(&right))
        } else {
            None
        }
    }

    pub fn get_all(&self) -> Vec<Handle<Image>> {
        self.iter_fields()
            .map(|field| field.downcast_ref::<Handle<Image>>())
            .filter(|res| res.is_some())
            .map(|res| res.unwrap())
            .cloned()
            .collect()
    }
}

fn icon_index(collection: &Res<IconCollection>, key: &str) -> Option<egui::TextureId> {
    collection
        .get_index(key)
        .map(|i| egui::TextureId::User(i as u64 + 10))
}

/// Setup ui resources (like bind loaded textures)
pub fn map_gui_textures(
    mut egui_context: ResMut<EguiContext>,
    startup: Res<StartupCollection>,
    icons: Res<IconCollection>,
) {
    egui_context.set_egui_texture(0, startup.splash.clone());
    egui_context.set_egui_texture(1, startup.title.clone());
    egui_context.set_egui_texture(2, startup.start.clone());
    egui_context.set_egui_texture(3, startup.options.clone());
    egui_context.set_egui_texture(4, startup.exit.clone());

    for (index, handle) in icons.get_all().iter().enumerate() {
        egui_context.set_egui_texture(10 + index as u64, handle.clone());
    }
}
