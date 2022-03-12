use super::super::{MapPosition, MapPositionPath};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_tweening::Animator;

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
    mut query: Query<
        (
            &mut MapPosition,
            &mut MapPositionPath,
            &Animator<Transform>,
            &mut SelectedAnimation,
        ),
        Or<(Changed<MapPositionPath>, Changed<Animator<Transform>>)>,
    >,
) {
    for (mut position, mut path, animator, mut animation) in query.iter_mut() {
        if animator.progress() == 1.0 {
            if let Some(next_position) = path.pop() {
                position.x = next_position.x;
                position.y = next_position.y;
                animation.current_key = "moving".to_string();
            } else {
                animation.current_key = "idle".to_string();
                continue;
            }
        }
    }
}
