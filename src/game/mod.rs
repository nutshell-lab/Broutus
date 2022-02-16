use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod map;
mod character;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(map::MapPlugin)
            .add_plugin(character::CharacterPlugin)
            .add_startup_system(startup);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(265.0, -655.0, 1000.0 - 0.1),
        ..OrthographicCameraBundle::new_2d()
    });
}
