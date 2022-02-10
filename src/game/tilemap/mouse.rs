use bevy::math::Vec2;
use bevy_ecs_tilemap::MapQuery;

pub fn test(windows: Res<Windows>, map_query: MapQuery) {
    let map = map_query.iter().first();
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        //let layer = find on-top layer at pixel {x, y} (from mouseInput)
        let grid_size = layer.settings.grid_size;
        let layer_size_in_tiles: Vec2 = layer.get_layer_size_in_tiles().into();
        let map_size: Vec2 = layer_size_in_tiles * grid_size;
    }
}
