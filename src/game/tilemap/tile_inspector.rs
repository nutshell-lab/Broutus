use bevy::prelude::*;
use bevy_ecs_tilemap::{MapQuery, TilePos};
use bevy_egui::{egui, EguiContext};

pub struct TileInspector;

impl Plugin for TileInspector {
    fn build(&self, app: &mut App) {
        app.add_system(tile_coords_ui);
    }
}

pub fn tile_coords_ui(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    mut map_query: MapQuery,
) {
    let game_win = windows.get_primary().unwrap();
    let tile_width_half: f32 = 128.0 / 2.0;
    let tile_height_half: f32 = 64.0 / 2.0;
    // let map_width_half = 20.0 * tile_width_half;
    // let map_height_half = 12.0 * tile_height_half;
    let offset_x = 693.0;
    let offset_y = 983.0;

    if let Some(mouse) = game_win.cursor_position() {
        let screen_x = mouse.x - offset_x;
        let screen_y = mouse.y - offset_y;
        let map_x =
            (((screen_x / tile_width_half + screen_y / tile_height_half) / 2.0) * -1.0) as u32;
        let map_y =
            (((screen_y / tile_height_half - screen_x / tile_width_half) / 2.0) * -1.0) as u32;

        // if let Some((_, mut map, _)) = map_query.map_query_set.q0.iter_mut().first() {}

        egui::Window::new("Tile Inspector").show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("{}, {}", map_x.to_string(), map_y.to_string()));
            match map_query.despawn_tile(&mut commands, TilePos(map_x, map_y), 0u16, 2u16) {
                Ok(result) => result,
                Err(error) => match error {
                    bevy_ecs_tilemap::MapTileError::OutOfBounds => {
                        ui.label("Out of bound");
                    }
                    bevy_ecs_tilemap::MapTileError::AlreadyExists => {
                        ui.label("Already exist");
                    }
                    bevy_ecs_tilemap::MapTileError::NonExistent => {
                        ui.label("Doesn't exist");
                    }
                },
            }
        });
    }
}
