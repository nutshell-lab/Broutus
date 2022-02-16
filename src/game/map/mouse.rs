use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::load::TmxMap;

#[derive(Default)]
pub struct MouseMapPosition(pub Option<TilePos>);

#[derive(Default)]
pub struct PreviousMouseMapPosition(pub Option<TilePos>);

pub fn unproject_iso(pos: Vec2, tile_width: f32, tile_height: f32) -> Vec2 {
    let half_width = tile_width / 2.0;
    let half_height = tile_height / 2.0;
    let x = ((pos.x / half_width) + (-(pos.y) / half_height)) / 2.0;
    let y = ((-(pos.y) / half_height) - (pos.x / half_width)) / 2.0;
    Vec2::new(x.round(), y.round())
}

pub fn update_mouse_position(
    mut position: ResMut<MouseMapPosition>,
    mut previous_position: ResMut<PreviousMouseMapPosition>,
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

                    // Get tmx data to get map size in tiles
                    let tiled_map = &tmx_map.get(tmx_handle).unwrap().map;

                    let tile_position =
                        unproject_iso(mouse_to_map_coords, grid_size.x, grid_size.y);

                    let save = position.0.clone();

                    // Check if the tile position is within map borders, otherwise return None, which is needed to handle correctly mouse events
                    position.0 = if tile_position.x >= 0.0
                        && tile_position.x < (tiled_map.width as f32)
                        && tile_position.y >= 0.0
                        && tile_position.y < (tiled_map.height as f32)
                    {
                        Some(TilePos(tile_position.x as u32, tile_position.y as u32))
                    } else {
                        None
                    };

                    if save.ne(&position.0) {
                        previous_position.0 = save;
                    }
                }
            }
        }
    }
}

pub fn highlight_mouse_tile(
    position: Res<MouseMapPosition>,
    previous_position: Res<PreviousMouseMapPosition>,
    mut map_query: MapQuery,
    mut tile_query: Query<&mut Tile>,
) {
    if position.is_changed() {
        if let Some(position) = position.0 {
            if let Ok(tile_entity) = map_query.get_tile_entity(position, 0u16, 0u16) {
                if let Ok(mut tile) = tile_query.get_mut(tile_entity) {
                    tile.color = Color::SEA_GREEN;
                    map_query.notify_chunk_for_tile(position, 0u16, 0u16);
                }
            }
        }
    }

    if previous_position.is_changed() {
        if let Some(previous_position) = previous_position.0 {
            if let Ok(tile_entity) = map_query.get_tile_entity(previous_position, 0u16, 0u16) {
                if let Ok(mut tile) = tile_query.get_mut(tile_entity) {
                    tile.color = Color::WHITE;
                    map_query.notify_chunk_for_tile(previous_position, 0u16, 0u16);
                }
            }
        }
    }
}

pub fn debug_ui_mouse_position(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    position: Res<MouseMapPosition>,
    previous_position: Res<PreviousMouseMapPosition>,
    mut egui_context: ResMut<EguiContext>,
) {
    let primary_window = windows.get_primary().unwrap();

    egui::Window::new("Mouse Position").show(egui_context.ctx_mut(), |ui| {
        for (camera, camera_transform) in camera_query.iter() {
            let offset = camera.world_to_screen(&windows, camera_transform, Vec3::ZERO).unwrap_or(Vec2::ZERO);
            if let Some(position) = primary_window.cursor_position() {
                ui.label(format!("Screen: {}, {}", position.x, position.y));
                ui.label(format!("World: {}, {}", position.x - offset.x, position.y - offset.y));
            } else {
                ui.label("Screen: #, #");
                ui.label("World: #, #");
            }
        }
    });

    egui::Window::new("Mouse Map Position").show(egui_context.ctx_mut(), |ui| {
        if let Some(TilePos(x, y)) = position.0 {
            ui.label(format!("{}, {}", x, y));
        } else {
            ui.label("#, #");
        }
    });

    egui::Window::new("Mouse Map Previous Position").show(egui_context.ctx_mut(), |ui| {
        if let Some(TilePos(x, y)) = previous_position.0 {
            ui.label(format!("{}, {}", x, y));
        } else {
            ui.label("#, #");
        }
    });
}
