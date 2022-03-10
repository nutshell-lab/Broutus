// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

mod game;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            title: String::from("Broutus"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(game::GamePlugin)
        .run();
}
