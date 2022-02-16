use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::load::TmxMap;

#[derive(Default)]
pub struct MouseMapPosition(TilePos);

pub fn unproject_iso(pos: Vec2, tile_width: f32, tile_height: f32) -> Vec2 {
    let half_width = tile_width / 2.0;
    let half_height = tile_height / 2.0;
    let x = ((pos.x / half_width) + (-(pos.y) / half_height)) / 2.0;
    let y = ((-(pos.y) / half_height) - (pos.x / half_width)) / 2.0;
    Vec2::new(x.round(), y.round())
}

pub fn update_mouse_position(
    mut position: ResMut<MouseMapPosition>,
    tmx_map: Res<Assets<TmxMap>>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    map_query: Query<(&GlobalTransform, &Handle<TmxMap>), With<Map>>,
    layer_query: Query<&Layer>,
) {
    if layer_query.is_empty() {
        return;
    }

    let primary_window = windows.get_primary().unwrap();
    let layer = layer_query.iter().next().unwrap();
    let grid_size = layer.settings.grid_size;
    let tile_size = layer.settings.tile_size;

    if let Some(mouse) = primary_window.cursor_position() {
        for (camera, camera_transform) in camera_query.iter() {
            for (map_transform, tmx_handle) in map_query.iter() {
                if let Some(map_screen_coords) =
                    camera.world_to_screen(&windows, camera_transform, map_transform.translation)
                {
                    // Get mous coords relative to the map coords
                    let mouse_to_map_coords = Vec2::new(
                        mouse.x - map_screen_coords.x,
                        // In our case, tileset tile height is greater than the map tile height to be able to display obstacles, we need to adjust to that
                        mouse.y - map_screen_coords.y + (tile_size.1 - grid_size.y / 2.0),
                    );
                    let map_position = unproject_iso(mouse_to_map_coords, grid_size.x, grid_size.y);

                    // Get tmx data to get map size in tiles
                    let tiled_map = &tmx_map.get(tmx_handle).unwrap().map;

                    position.0 = TilePos(
                        (map_position.x as u32).min(tiled_map.width - 1).max(0),
                        (map_position.y as u32).min(tiled_map.height - 1).max(0),
                    );
                }
            }
        }
    }
}

pub fn despawn_tile_at_mouse_position(
    position: Res<MouseMapPosition>,
    mut map_query: MapQuery,
    mut tile_query: Query<&mut Tile>,
) {
    if let Ok(tile_entity) = map_query.get_tile_entity(position.0, 0u16, 0u16) {
        if let Ok(mut tile) = tile_query.get_mut(tile_entity) {
            if tile.visible {
                tile.visible = false;
                map_query.notify_chunk_for_tile(position.0, 0u16, 0u16);
            }
        }
    }
}

pub fn debug_ui_mouse_position(
    position: Res<MouseMapPosition>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Mouse Map Position").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("{}, {}", position.0 .0, position.0 .1));
    });
}
