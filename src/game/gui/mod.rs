use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy_asset_loader::AssetCollection;
use bevy_egui::egui;
use bevy_egui::EguiContext;
use winit::window::Icon;

pub mod arena;
pub mod menu;
pub mod splash;
mod widgets;

#[derive(AssetCollection)]
pub struct StartupCollection {
    #[asset(path = "icon.png")]
    pub icon: Handle<Image>,

    #[asset(path = "splash.png")]
    pub splash: Handle<Image>,

    #[asset(path = "title.png")]
    pub btn_title: Handle<Image>,

    #[asset(path = "buttons/start.png")]
    pub btn_start: Handle<Image>,

    #[asset(path = "buttons/resume.png")]
    pub btn_resume: Handle<Image>,

    #[asset(path = "buttons/options.png")]
    pub btn_options: Handle<Image>,

    #[asset(path = "buttons/exit.png")]
    pub btn_exit: Handle<Image>,

    #[asset(path = "buttons/return.png")]
    pub btn_return: Handle<Image>,
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
    egui_context.set_egui_texture(1, startup.btn_title.clone());
    egui_context.set_egui_texture(2, startup.btn_start.clone());
    egui_context.set_egui_texture(3, startup.btn_resume.clone());
    egui_context.set_egui_texture(4, startup.btn_options.clone());
    egui_context.set_egui_texture(5, startup.btn_exit.clone());
    egui_context.set_egui_texture(6, startup.btn_return.clone());

    for (index, handle) in icons.get_all().iter().enumerate() {
        egui_context.set_egui_texture(10 + index as u64, handle.clone());
    }
}

pub fn set_window_icon(
    windows: Res<WinitWindows>,
    images: Res<StartupCollection>,
    server: Res<Assets<Image>>,
) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon = server.get(images.icon.clone()).unwrap();
    let icon = Icon::from_rgba(
        icon.data.clone(),
        icon.texture_descriptor.size.width,
        icon.texture_descriptor.size.height,
    )
    .unwrap();

    primary.set_window_icon(Some(icon));
}
