use super::GameState;
use bevy::prelude::*;

mod events;
mod mouse;
mod position;
mod query;
mod tiledmap;

use events::trigger_map_mouse_events;
use mouse::update_map_mouse_position;
use tiledmap::spawn_tiledmap;
use tiledmap::TiledmapLoader;

pub use events::TileLeftClickedEvent;
pub use events::TileRightClickedEvent;
pub use mouse::MouseMapPosition;
pub use mouse::PreviousMouseMapPosition;
pub use position::*;
pub use query::MapQuery;
pub use tiledmap::Layer;
pub use tiledmap::Map;
pub use tiledmap::MapsAssets;
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
            .add_event::<TileLeftClickedEvent>()
            .add_event::<TileRightClickedEvent>()
            .add_asset::<Tiledmap>()
            .add_asset_loader(TiledmapLoader)
            .add_system_set(SystemSet::on_enter(GameState::Arena).with_system(spawn_tiledmap))
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .with_system(update_map_mouse_position)
                    .with_system(trigger_map_mouse_events),
            );
    }
}

/// TilePos --> WorldPos
pub fn project_iso(pos: &MapPosition, tile_width: f32, tile_height: f32) -> Vec2 {
    let x = (pos.x as f32 - pos.y as f32) * tile_width / 2.0;
    let y = (pos.x as f32 + pos.y as f32) * tile_height / 2.0;
    Vec2::new(x, -y)
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
