use bevy::prelude::*;

mod tilemap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(tilemap::TilemapPlugin);
    }
}
