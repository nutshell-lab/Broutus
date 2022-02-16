use bevy::prelude::*;

mod events;
mod load;
mod mouse;
mod texture;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<mouse::MouseMapPosition>()
            .add_event::<events::TileClickedEvent>()
            .add_plugin(bevy_ecs_tilemap::TilemapPlugin)
            .add_plugin(load::TmxPlugin)
            .add_startup_system(startup)
            .add_system(texture::set_texture_filters_to_nearest)
            .add_system(mouse::update_mouse_position)
            // .add_system(mouse::despawn_tile_at_mouse_position)
            .add_system(mouse::debug_ui_mouse_position)
            .add_system(events::detect_mouse_tile_events);
        // .add_system(events::debug_mouse_tile_clicks);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<load::TmxMap> = asset_server.load("maps/arena.tmx");
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
