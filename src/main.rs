use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod game;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: String::from("Broutus"),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<game::tilemap::load::TiledMap>::default())
        .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<Image>::default())
        .add_plugin(game::GamePlugin)
        .run();
}
