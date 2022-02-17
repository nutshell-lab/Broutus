use super::map::TilePos;
use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Character;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ActionPoints(pub u32, u32);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct MovementPoints(pub u32, u32);

impl ActionPoints {
    pub fn reset(&mut self) {
        self.0 = self.1;
    }
}

impl MovementPoints {
    pub fn reset(&mut self) {
        self.0 = self.1;
    }
}

#[derive(Default, Bundle)]
pub struct CharacterBundle {
    _c: Character,
    name: Name,
    position: TilePos,
    action_points: ActionPoints,
    movement_points: MovementPoints,
    #[bundle]
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
}

impl CharacterBundle {
    pub fn new(
        name: String,
        position: TilePos,
        flip: f32,
        texture_atlas_handle: &Handle<TextureAtlas>,
    ) -> Self {
        CharacterBundle {
            name: Name::new(name),
            position,
            action_points: ActionPoints(6, 6),
            movement_points: MovementPoints(5, 5),
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
pub struct AnimationTimer(Timer);

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

/// Update the character's Transform based on it's TilePos
pub fn snap_to_map(mut q: Query<(&mut Transform, &TilePos), (With<Character>, Changed<TilePos>)>) {
    for (mut transform, position) in q.iter_mut() {
        let coords =
            super::map::project_iso(Vec2::new(position.0 as f32, position.1 as f32), 128.0, 64.0); // TODO unhardcode this
        transform.translation.x = coords.x;
        transform.translation.y = coords.y - (256.0 - 64.0 / 2.0 - 64.0 / 2.0); // - (tile_height - grid_height / 2 - character_height / 2)
    }
}
