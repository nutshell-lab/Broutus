use std::time::Duration;

use super::super::{Map, MapPosition, MapPositionPath};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningType};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SelectedAnimation {
    /// Currently selected animation
    pub current_key: String,

    /// Bound of each available animation (eg. idle 0-15, walk 16-45, attack 46-82)
    pub animations: HashMap<String, (usize, usize)>,
}

impl SelectedAnimation {
    /// Get the next sprite index in the TextureAtlas based on the current index and the selected animation
    fn next(&self, index: usize) -> usize {
        let (min, max) = self
            .animations
            .get(&self.current_key)
            .copied()
            .unwrap_or((0, 1));
        ((index.clamp(min - 1, max) + 1) % max).clamp(min, max)
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
pub fn update_warrior_position(
    map_query: Query<&Map>,
    mut warrior_query: Query<
        (
            &Transform,
            &mut MapPosition,
            &mut MapPositionPath,
            &mut Animator<Transform>,
            &mut SelectedAnimation,
        ),
        Or<(Changed<MapPositionPath>, Changed<Animator<Transform>>)>,
    >,
) {
    if map_query.is_empty() {
        return;
    }
    if warrior_query.is_empty() {
        return;
    }

    let map = map_query.single();
    let obstacle_layer_id = map.obstacle_layer;
    let map_width = map.width;
    let map_height = map.height;
    let tile_width = map.tile_width as f32;
    let tile_height = map.tile_height as f32;

    for (transform, mut position, mut path, mut animator, mut animation) in warrior_query.iter_mut()
    {
        if animator.progress() == 1.0 {
            if let Some(next_position) = path.0.pop() {
                position.x = next_position.x;
                position.y = next_position.y;

                let mut translation = next_position.to_xyz(
                    obstacle_layer_id,
                    map_width,
                    map_height,
                    tile_width,
                    tile_height,
                );
                translation.y += 135. / 9.;
                let tween = Tween::new(
                    EaseFunction::CircularInOut,
                    TweeningType::Once,
                    Duration::from_millis(300),
                    TransformPositionLens {
                        start: transform.translation.clone(),
                        end: translation,
                    },
                );

                animator.set_tweenable(tween);
                animation.current_key = "moving".to_string();
            } else {
                animation.current_key = "idle".to_string();
                continue;
            }
        }
    }
}
