use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::WorldInspectorPlugin;

mod color;
mod gameplay;
mod map;
mod ui;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// Load all game assets
    Loading,

    // /// Show main menu
    // MENU,

    // /// Prepare your team
    // PREPARE,
    /// Fight !
    Arena,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<map::MapsAssets>()
            .with_collection::<gameplay::WarriorAssets>()
            .with_collection::<ui::ActionsAssets>()
            .continue_to_state(GameState::Arena)
            .build(app);

        app.add_state(GameState::Loading)
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(map::TiledmapPlugin)
            .add_plugin(gameplay::GameplayPlugin)
            .add_startup_system(setup_camera)
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .with_system(ui::show_turn_ui)
                    .with_system(ui::show_health_bar_ui)
                    .with_system(ui::show_action_bar_ui)
                    .with_system(ui::show_battlelog_ui)
                    .with_system(ui::show_warrior_ui),
            );
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
