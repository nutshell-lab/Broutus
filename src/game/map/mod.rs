use bevy::prelude::*;

mod events;
mod load;
mod mouse;
mod position;
mod texture;

// wip
mod new;

pub use bevy_ecs_tilemap::Chunk;
pub use bevy_ecs_tilemap::Layer;
pub use bevy_ecs_tilemap::Map;
pub use bevy_ecs_tilemap::MapQuery;
pub use bevy_ecs_tilemap::Tile;
pub use bevy_ecs_tilemap::TilePos;
pub use bevy_ecs_tilemap::TileSize;
pub use events::*;
pub use load::TmxMap;
pub use mouse::MouseMapPosition;
pub use mouse::PreviousMouseMapPosition;
pub use position::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<mouse::MouseMapPosition>()
            .init_resource::<mouse::PreviousMouseMapPosition>()
            .add_event::<events::TileClickedEvent>()
            .add_event::<events::TileRightClickedEvent>()
            .add_plugin(bevy_ecs_tilemap::TilemapPlugin)
            .add_plugin(load::TmxPlugin)
            .add_startup_system(startup)
            .add_system(texture::set_texture_filters_to_nearest)
            .add_system(mouse::update_mouse_position)
            .add_system(mouse::debug_ui_mouse_position)
            .add_system(events::detect_tile_clicked_events);
        // .add_system(events::debug_tile_clicked_events);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<load::TmxMap> = asset_server.load("maps/simple.tmx");
    let map_entity = commands.spawn().id();

    commands
        .entity(map_entity)
        .insert_bundle(load::TmxMapBundle {
            tiled_map: handle,
            map: bevy_ecs_tilemap::Map::new(0u16, map_entity),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}

/// Apply a highlight color to a tile at the given position
pub fn highlight_tile(
    map_query: &mut MapQuery,
    tile_query: &mut Query<&mut Tile>,
    position: TilePos,
    color: Color,
) {
    if let Ok(tile_entity) = map_query.get_tile_entity(position, 0u16, 1u16) {
        if let Ok(mut tile) = tile_query.get_mut(tile_entity) {
            tile.color = color;
            map_query.notify_chunk_for_tile(position, 0u16, 1u16);
        }
    }
}

pub fn path(
    mut map_query: &mut MapQuery,
    start: TilePos,
    end: TilePos,
    map_width: u32,
    map_height: u32,
) -> Option<(Vec<TilePos>, u32)> {
    pathfinding::prelude::astar(
        &start,
        |position| tile_neightbours(&mut map_query, &position, map_width, map_height),
        |current| tile_distance(current, &end),
        |position| position.eq(&end),
    )
}

/// Get the list of tile neightbours at the given position
pub fn tile_neightbours(
    mut map_query: &mut MapQuery,
    position: &TilePos,
    map_width: u32,
    map_height: u32,
) -> Vec<(TilePos, u32)> {
    // Allow diagonal movements
    // #[rustfmt::skip]
    // let neightbours = vec![
    //     TilePos(position.0.wrapping_sub(1), position.1.wrapping_add(1)),    TilePos(position.0, position.1.wrapping_add(1)),    TilePos(position.0.wrapping_add(1), position.1.wrapping_add(1)),
    //     TilePos(position.0.wrapping_sub(1), position.1),                                                                        TilePos(position.0.wrapping_add(1), position.1),
    //     TilePos(position.0.wrapping_sub(1), position.1.wrapping_sub(1)),    TilePos(position.0, position.1.wrapping_sub(1)),    TilePos(position.0.wrapping_add(1), position.1.wrapping_sub(1)),
    // ];

    #[rustfmt::skip]
    let neightbours = vec![
        TilePos(position.0, position.1.wrapping_add(1)), // Up
        TilePos(position.0.wrapping_sub(1), position.1), // Left
        TilePos(position.0.wrapping_add(1), position.1),  // Right
        TilePos(position.0, position.1.wrapping_sub(1)), // Down
    ];

    neightbours
        .iter()
        .filter(|&position| position.0 < map_width && position.1 < map_height)
        .filter(|&position| !is_obstacle(&mut map_query, position.clone()))
        .map(|&position| (position.clone(), 1))
        .collect()
}

/// Get the list of tile neightbours at the given position
pub fn tile_distance(start: &TilePos, end: &TilePos) -> u32 {
    (pathfinding::prelude::absdiff(start.0, end.0) + pathfinding::prelude::absdiff(start.1, end.1))
        as u32
}

/// Return true if a tile exists at the given position in the obstacle layer
pub fn is_obstacle(map_query: &mut MapQuery, position: TilePos) -> bool {
    map_query.get_tile_entity(position, 0u16, 2u16).is_ok()
}
