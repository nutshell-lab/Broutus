use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweeningType};

use self::map::{LayerIndex, Map, MapPosition};

mod color;
mod gameplay;
mod gui;
mod map;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// Load splashscreen
    Startup,

    /// Load all game assets
    Loading,

    // /// Show main menu
    Menu,

    /// Fight !
    Arena,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Startup)
            .with_asset_collection_file("dynamic.assets")
            .with_collection::<gui::StartupCollection>()
            .continue_to_state(GameState::Loading)
            .build(app);

        AssetLoader::new(GameState::Loading)
            .with_asset_collection_file("dynamic.assets")
            .with_collection::<gui::IconCollection>()
            .with_collection::<map::MapsAssets>()
            .with_collection::<gameplay::WarriorCollection>()
            .with_collection::<gameplay::AnimationCollection>()
            .continue_to_state(GameState::Menu)
            .build(app);

        app.add_state(GameState::Startup)
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(map::TiledmapPlugin)
            .add_plugin(gameplay::GameplayPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::Loading).with_system(gui::splash::show_splash),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Loading)
                    .with_system(setup_camera)
                    .with_system(gui::map_gui_textures),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(gui::menu::show_main_menu),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .with_system(gui::arena::show_turn_ui)
                    .with_system(gui::arena::show_turn_button_ui)
                    .with_system(gui::arena::show_health_bar_ui)
                    .with_system(gui::arena::show_action_points_ui)
                    .with_system(gui::arena::show_movement_points_ui)
                    .with_system(gui::arena::show_action_bar_ui)
                    .with_system(gui::arena::handle_action_bar_shortcuts)
                    .with_system(gui::arena::show_warrior_ui)
                    .with_system(map_position_update)
                    .with_system(map_position_update_smoolthy::<200>),
            );
    }
}

/// Spawn the main camera
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::identity().with_translation((0.0, -600.0, 1000.0 - 0.1).into()),
        // .with_scale((1.2, 1.2, 1.0).into()),
        ..OrthographicCameraBundle::new_2d()
    });
}

pub fn map_position_update_smoolthy<const DURATION: u64>(
    map_query: Query<&Map>,
    mut query: Query<
        (
            &Transform,
            &MapPosition,
            &LayerIndex,
            &mut Animator<Transform>,
        ),
        Or<(Changed<MapPosition>, Changed<LayerIndex>)>,
    >,
) {
    if map_query.is_empty() {
        return;
    }

    let map = map_query.single();
    for (transform, position, layer_index, mut animator) in query.iter_mut() {
        // Could we append the new tween at the end of the current Animator's Sequence ?
        animator.set_tweenable(Tween::new(
            EaseFunction::ExponentialIn,
            TweeningType::Once,
            Duration::from_millis(DURATION),
            TransformPositionLens {
                start: transform.translation.clone(),
                end: position.to_xyz(map, layer_index),
            },
        ));
    }
}

pub fn map_position_update(
    map_query: Query<&Map>,
    mut query: Query<
        (&mut Transform, &MapPosition, &LayerIndex),
        (
            Without<Animator<Transform>>,
            Or<(Changed<MapPosition>, Changed<LayerIndex>)>,
        ),
    >,
) {
    if map_query.is_empty() {
        return;
    }

    let map = map_query.single();
    for (mut transform, position, layer_index) in query.iter_mut() {
        transform.translation = position.to_xyz(map, layer_index);
    }
}
