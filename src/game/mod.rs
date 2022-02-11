use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub mod tilemap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(tilemap::TilemapPlugin)
            .add_startup_system(spawn_cameras);
    }
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(265.0, -655.0, 1000.0 - 0.1),
        ..OrthographicCameraBundle::new_2d()
    });
}
