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
    team: super::Team,

    // Gameplay
    position: super::MapPosition,
    health: Attribute<Health>,
    shield: Attribute<Shield>,
    action_points: Attribute<ActionPoints>,
    movement_points: Attribute<MovementPoints>,
    actions: Actions,

    // TODO add animation collection ? How to load it ?
    // Redering
    #[bundle]
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
    animation: SelectedAnimation,
}
