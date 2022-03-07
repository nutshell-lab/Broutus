use super::super::{Map, MapPosition, Tiledmap};
use super::Warrior;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SelectedAnimation {
    /// Currently selected animation
    current_key: String,

    /// Bound of each available animation (eg. idle 0-15, walk 16-45, attack 46-82)
    animations: HashMap<String, (usize, usize)>,
}

impl SelectedAnimation {
    /// Get the next sprite index in the TextureAtlas based on the current index and the selected animation
    fn next(&self, index: usize) -> usize {
        let (min, max) = self.animations.get(&self.current_key).unwrap();
        index % max - min
    }
}

/// Animate the sprite based on the AnimationTimer
pub fn animate_warrior_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &SelectedAnimation,
    )>,
) {
    for (mut timer, mut sprite, animations) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = animations.next(sprite.index);
        }
    }
}

/// Update the warrior's Transform based on it's MapPosition
pub fn update_warrior_world_position(
    tiledmaps: Res<Assets<Tiledmap>>,
    map_query: Query<(Entity, &Map, &Handle<Tiledmap>)>,
    mut warrior_query: Query<(&mut Transform, &MapPosition), (With<Warrior>, Changed<MapPosition>)>,
) {
    if map_query.is_empty() {
        return;
    }
    if warrior_query.is_empty() {
        return;
    }

    let (_, map, tiledmap_handle) = map_query.single();
    let tiledmap = tiledmaps.get(tiledmap_handle);

    if let Some(tiledmap) = tiledmap {
        let obstacle_layer_id = map.obstacle_layer;
        let map_width = tiledmap.inner.width;
        let map_height = tiledmap.inner.height;
        let tile_width = tiledmap.inner.tile_width as f32;
        let tile_height = tiledmap.inner.tile_height as f32;

        for (mut transform, position) in warrior_query.iter_mut() {
            transform.translation = position.to_xyz(
                obstacle_layer_id,
                map_width,
                map_height,
                tile_width,
                tile_height,
            );
        }
    }
}
