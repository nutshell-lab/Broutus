mod action;
mod asset;
mod attribute;
mod render;

use bevy::prelude::*;

pub use action::*;
pub use asset::*;
pub use attribute::*;
pub use render::*;

#[derive(Default, Component)]
pub struct Warrior;

#[derive(Default, Bundle)]
pub struct WarriorBundle {
    // Tags
    _w: Warrior,

    // Meta
    name: Name,

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
    pub fn new(asset: &WarriorAsset, animation_collection: &Res<AnimationCollection>) -> Self {
        WarriorBundle {
            _w: Warrior,

            name: Name::new(asset.name.clone()),

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
