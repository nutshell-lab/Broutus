mod action;
mod asset;
mod attribute;
mod events;
mod render;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweeningType};
use std::time::Duration;

pub use action::*;
pub use asset::*;
pub use attribute::*;
pub use events::*;
pub use render::*;

use crate::game::map::MapPosition;

use super::Team;

#[derive(Default, Clone, Copy, Component)]
pub struct Warrior;

/// Stores the current path *reversed*
#[derive(Default, Component)]
pub struct MapPositionPath(Vec<MapPosition>);

impl MapPositionPath {
    pub fn pop(&mut self) -> Option<MapPosition> {
        self.0.pop()
    }

    pub fn set(&mut self, path: Vec<MapPosition>) {
        self.0 = path;
        self.0.reverse();
    }
}

#[derive(Default, Bundle)]
pub struct WarriorBundle {
    // Tags
    _w: Warrior,

    // Meta
    name: Name,
    team: Team,
    position: MapPosition,
    path: MapPositionPath,
    animator: Animator<Transform>,

    // Gameplay
    health: Attribute<Health>,
    shield: Attribute<Shield>,
    action_points: Attribute<ActionPoints>,
    movement_points: Attribute<MovementPoints>,
    actions: Actions,
    active_effects: ActiveEffects,

    // TODO add animation collection ? How to load it ?
    // Redering
    #[bundle]
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
    animation: SelectedAnimation,
}

impl WarriorBundle {
    pub fn new(
        asset: &WarriorAsset,
        animation_collection: &Res<AnimationCollection>,
        team: &Team,
        position: MapPosition,
    ) -> Self {
        WarriorBundle {
            _w: Warrior,

            name: Name::new(asset.name.clone()),
            team: team.clone(),
            position,
            path: MapPositionPath(vec![position]),
            animator: Animator::new(Tween::new(
                EaseFunction::CubicIn,
                TweeningType::Once,
                Duration::from_millis(700),
                TransformPositionLens {
                    start: Vec3::new(0., 0., 0.),
                    end: Vec3::new(0., 0., 0.),
                },
            )),

            health: asset.health.clone(),
            shield: asset.shield.clone(),
            action_points: asset.action_points.clone(),
            movement_points: asset.movement_points.clone(),
            actions: Actions(asset.actions.0.to_vec()),
            active_effects: ActiveEffects::default(),

            sprite: SpriteSheetBundle {
                texture_atlas: animation_collection
                    .get(asset.render.atlas_texture.as_str())
                    .unwrap()
                    .clone(),
                transform: Transform::from_scale(Vec3::new(2.0, 2.5, 1.0)),
                ..Default::default()
            },
            animation_timer: AnimationTimer(Timer::from_seconds(0.15, true)),
            animation: SelectedAnimation {
                current_key: "idle".to_string(),
                animations: asset.render.animations.clone(),
            },
        }
    }
}
