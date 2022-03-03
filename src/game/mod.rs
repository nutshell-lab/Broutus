use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::WorldInspectorPlugin;

mod gameplay;
mod map;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// Load all game assets
    LOADING,

    // /// Show main menu
    // MENU,

    // /// Prepare your team
    // PREPARE,
    /// Fight !
    ARENA,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, mut app: &mut App) {
        AssetLoader::new(GameState::LOADING)
            .with_collection::<map::MapsAssets>()
            .with_collection::<gameplay::WarriorAssets>()
            .continue_to_state(GameState::ARENA)
            .build(&mut app);

        app.add_state(GameState::LOADING)
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(map::TiledmapPlugin)
            .add_plugin(gameplay::GameplayPlugin)
            .add_startup_system(setup_camera);
    }
}

/// Spawn the main camera
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::identity().with_translation((250.0, -490.0, 1000.0 - 0.1).into()),
        // .with_scale((1.2, 1.2, 1.0).into()),
        ..OrthographicCameraBundle::new_2d()
    });
}
