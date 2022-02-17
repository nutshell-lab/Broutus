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
    camera_query: Query<(&Transform, Option<&OrthographicProjection>), With<Camera>>,
    map_query: Query<(&GlobalTransform, &Handle<TmxMap>), With<Map>>,
    layer_query: Query<&Layer>,
) {
    if layer_query.is_empty() {
        return;
    }

    let primary_window = windows.get_primary().unwrap();
    
    if let Some(mouse) = primary_window.cursor_position() {
        let layer = layer_query.iter().next().unwrap();
        let grid_size = layer.settings.grid_size;
        let tile_size = layer.settings.tile_size;
        
        for (map_transform, tmx_handle) in map_query.iter() {
            // get the size of the window
            let window_size = Vec2::new(primary_window.width() as f32, primary_window.height() as f32);
            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = mouse - window_size / 2.0;

            // assuming there is exactly one main camera entity, so this is OK
            let (camera_transform, ortho_projection) = camera_query.single();

            let mut scale = 1.0;
            if let Some(ortho_projection) = ortho_projection {
                scale /= ortho_projection.scale;
            }

            // undo orthographic scale and apply the camera transform
            let mouse_in_world = camera_transform.compute_matrix() * p.extend(0.0).extend(scale);
            let mouse_in_map = Vec2::new(
                mouse_in_world.x - map_transform.translation.x,
                // In our case, tileset tile height is greater than the map tile height to be able to display obstacles, we need to adjust to that
                mouse_in_world.y - map_transform.translation.y + (tile_size.1 - grid_size.y / 2.0 - 6.0),
            );

            // Get tmx data to get map size in tiles
            let tiled_map = &tmx_map.get(tmx_handle).unwrap().map;

            let tile_position =
                unproject_iso(mouse_in_map, grid_size.x, grid_size.y);

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
