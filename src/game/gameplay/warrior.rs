use super::super::map::MapPosition;
use super::super::map::MapQuery;
use super::attribute::*;
use super::weapon::{Effect, EffectType, Weapon};
use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Warrior;

#[derive(Default, Bundle)]
pub struct WarriorBundle {
    _c: Warrior,
    name: Name,
    position: MapPosition,
    health: Health,
    weapon: Weapon,
    action_points: ActionPoints,
    movement_points: MovementPoints,
    #[bundle]
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
}

impl WarriorBundle {
    pub fn new(
        name: String,
        position: MapPosition,
        flip: f32,
        texture_atlas_handle: &Handle<TextureAtlas>,
    ) -> Self {
        WarriorBundle {
            name: Name::new(name),
            position,
            health: Health(Attribute { value: 50, max: 50 }),
            action_points: ActionPoints(Attribute { value: 6, max: 6 }),
            movement_points: MovementPoints(Attribute { value: 5, max: 5 }),
            weapon: Weapon::new(
                String::from("Dague du bandit"),
                Effect::new(10, (0, 1), 3, EffectType::Attack()),
            ),
            animation_timer: AnimationTimer(Timer::from_seconds(0.15, true)),
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0))
                    .with_scale(Vec3::new(2.0 * flip, 2.5, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer); // TODO maybe improve this thing to support multiple animations (idle, run, attack...)

/// Animate the sprite based on the AnimationTimer
pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

/// Update the warrior's Transform based on it's TilePos
pub fn snap_to_map(
    mut q: Query<(&mut Transform, &MapPosition), (With<Warrior>, Changed<MapPosition>)>,
    // map_query: &MapQuery,
) {
    let obstacle_layer_id = 2u32;

    for (mut transform, position) in q.iter_mut() {
        transform.translation = position.to_xyz(obstacle_layer_id, 11, 19, 128.0, 64.0);
    }
}
