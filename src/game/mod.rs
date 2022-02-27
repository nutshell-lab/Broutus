use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod gameplay;
mod map;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(map::TiledmapPlugin)
            .add_plugin(gameplay::GameplayPlugin)
            .add_startup_system(setup_camera);
    }
}

/// Spawn the main camera
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::identity()
            .with_translation(Vec3::new(250.0, -490.0, 1000.0 - 0.1))
            .with_scale(Vec3::new(1.2, 1.2, 1.0)),
        ..OrthographicCameraBundle::new_2d()
    });
}
