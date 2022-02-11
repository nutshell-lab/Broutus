use bevy::prelude::*;

mod load;
mod texture;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin)
            .add_plugin(load::TiledMapPlugin)
            .add_startup_system(startup)
            .add_system(texture::set_texture_filters_to_nearest);
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<load::TiledMap> = asset_server.load("maps/arena.tmx");
    let map_entity = commands.spawn().id();

    commands
        .entity(map_entity)
        .insert_bundle(load::TiledMapBundle {
            tiled_map: handle,
            map: bevy_ecs_tilemap::Map::new(0u16, map_entity),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}
