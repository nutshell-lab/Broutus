use bevy::prelude::*;

mod camera;
mod load;
mod texture;

pub struct TilemapPlugin;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let handle: Handle<load::TiledMap> = asset_server.load("arena3.tmx");

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

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin)
            .add_plugin(load::TiledMapPlugin)
            .add_system(camera::movement)
            .add_system(texture::set_texture_filters_to_nearest)
            .add_startup_system(startup);
    }
}
