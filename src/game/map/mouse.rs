use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use super::Map;
use super::MapPosition;
use super::Tiledmap;

#[derive(Default)]
pub struct MouseMapPosition(pub Option<MapPosition>);

#[derive(Default)]
pub struct PreviousMouseMapPosition(pub Option<MapPosition>);

pub fn update_map_mouse_position(
    mut position: ResMut<MouseMapPosition>,
    mut previous_position: ResMut<PreviousMouseMapPosition>,
    tmx_map: Res<Assets<Tiledmap>>,
    windows: Res<Windows>,
    camera_query: Query<(&Transform, Option<&OrthographicProjection>), With<Camera>>,
    map_query: Query<(&GlobalTransform, &Handle<Tiledmap>), With<Map>>,
) {
    if map_query.is_empty() {
        return;
    }

    let primary_window = windows.get_primary().unwrap();

    if let Some(mouse) = primary_window.cursor_position() {
        for (map_transform, tmx_handle) in map_query.iter() {
            if let Some(tiledmap) = &tmx_map.get(tmx_handle) {
                // let grid_size = Vec2::new(tileset.tile_width as f32, tileset.tile_height as f32);
                let tile_size = Vec2::new(
                    tiledmap.inner.tile_width as f32,
                    tiledmap.inner.tile_height as f32,
                );

                // get the size of the window
                let window_size = Vec2::new(
                    primary_window.width() as f32,
                    primary_window.height() as f32,
                );
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
                let mouse_in_world =
                    camera_transform.compute_matrix() * p.extend(0.0).extend(scale);
                let mouse_in_map = Vec2::new(
                    mouse_in_world.x - map_transform.translation.x,
                    // In our case, tileset tile height is greater than the map tile height to be able to display obstacles, we need to adjust to that
                    mouse_in_world.y - map_transform.translation.y + (tile_size.y / 2.0),
                );

                let tile_position = super::unproject_iso(
                    mouse_in_map,
                    tile_size.x,
                    tile_size.y,
                    tiledmap.inner.width,
                    tiledmap.inner.height,
                );

                let save = position.0.clone();
                position.0 = tile_position;

                if save.ne(&position.0) {
                    previous_position.0 = save;
                }
            }
        }
    }
}

pub fn show_debug_mouse_position_ui(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    position: Res<MouseMapPosition>,
    previous_position: Res<PreviousMouseMapPosition>,
    mut egui_context: ResMut<EguiContext>,
) {
    let primary_window = windows.get_primary().unwrap();

    egui::Window::new("Mouse Position").show(egui_context.ctx_mut(), |ui| {
        for (camera, camera_transform) in camera_query.iter() {
            let offset = camera
                .world_to_screen(&windows, camera_transform, Vec3::ZERO)
                .unwrap_or(Vec2::ZERO);
            if let Some(position) = primary_window.cursor_position() {
                ui.label(format!("Screen: {}, {}", position.x, position.y));
                ui.label(format!(
                    "World: {}, {}",
                    position.x - offset.x,
                    position.y - offset.y
                ));
            } else {
                ui.label("Screen: #, #");
                ui.label("World: #, #");
            }

            if let Some(MapPosition { x, y }) = position.0 {
                ui.label(format!("Map: {}, {}", x, y));
            } else {
                ui.label("Map: #, #");
            }

            if let Some(MapPosition { x, y }) = previous_position.0 {
                ui.label(format!("Map (prev): {}, {}", x, y));
            } else {
                ui.label("Map (prev): #, #");
            }
        }
    });
}
