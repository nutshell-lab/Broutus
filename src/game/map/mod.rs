use bevy::prelude::*;

mod events;
mod mouse;
mod query;
mod tiledmap;

use events::detect_tile_clicked_events;
use mouse::debug_ui_mouse_position;
use mouse::update_mouse_position;
use tiledmap::process_loaded_tiledmaps;
use tiledmap::MapBundle;
use tiledmap::TiledmapLoader;

pub use events::TileClickedEvent;
pub use events::TileRightClickedEvent;
pub use mouse::MouseMapPosition;
pub use mouse::PreviousMouseMapPosition;
pub use query::MapQuery;
pub use tiledmap::Layer;
pub use tiledmap::Map;
pub use tiledmap::MapPosition;
pub use tiledmap::Tile;
pub use tiledmap::Tiledmap;

pub struct TiledmapPlugin;

impl Plugin for TiledmapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapPosition>()
            .register_type::<Map>()
            .register_type::<Layer>()
            .register_type::<Tile>()
            .init_resource::<MouseMapPosition>()
            .init_resource::<PreviousMouseMapPosition>()
            .add_event::<TileClickedEvent>()
            .add_event::<TileRightClickedEvent>()
            .add_asset::<Tiledmap>()
            .add_asset_loader(TiledmapLoader)
            .add_startup_system(startup)
            .add_system(process_loaded_tiledmaps)
            .add_system(update_mouse_position)
            .add_system(detect_tile_clicked_events)
            .add_system(debug_ui_mouse_position);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Tiledmap> = asset_server.load("maps/simple.tmx");
    let map_entity = commands.spawn().id();
    commands
        .entity(map_entity)
        .insert(Name::new("map"))
        .insert_bundle(MapBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            tiledmap: handle,
            map: Map {
                ground_layer: 0,
                highlight_layer: 1,
                obstacle_layer: 2,
                spawn_team_a_layer: 3,
                spawn_team_b_layer: 4,
                ..Default::default()
            },
            ..Default::default()
        });
}

/// TilePos --> WorldPos
pub fn project_iso(pos: &MapPosition, tile_width: f32, tile_height: f32) -> Vec2 {
    let x = (pos.x as f32 - pos.y as f32) * tile_width / 2.0;
    let y = (pos.x as f32 + pos.y as f32) * tile_height / 2.0;
    return Vec2::new(x, -y);
}

/// WorldPos --> TilePos
pub fn unproject_iso(
    pos: Vec2,
    tile_width: f32,
    tile_height: f32,
    map_width: u32,
    map_height: u32,
) -> Option<MapPosition> {
    let half_width = tile_width / 2.0;
    let half_height = tile_height / 2.0;
    let x = (((pos.x / half_width) + (-(pos.y) / half_height)) / 2.0).round();
    let y = (((-(pos.y) / half_height) - (pos.x / half_width)) / 2.0).round();

    if x >= 0.0 && y >= 0.0 && x < map_width as f32 && y < map_height as f32 {
        Some(MapPosition::new(x as u32, y as u32))
    } else {
        None
    }
}

/// Get the list of tile neightbours at the given position
pub fn tile_distance(start: &MapPosition, end: &MapPosition) -> u32 {
    (pathfinding::prelude::absdiff(start.x, end.x) + pathfinding::prelude::absdiff(start.x, end.x))
        as u32
}
