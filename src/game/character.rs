use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Character;

#[derive(Default, Bundle)]
pub struct CharacterBundle {
    _c: Character,
    name: Name,
    animation_timer: AnimationTimer,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl CharacterBundle {
    pub fn new(
        name: String,
        position: Vec2,
        flip: f32,
        texture_atlas_handle: &Handle<TextureAtlas>,
    ) -> Self {
        CharacterBundle {
            name: Name::new(name),
            animation_timer: AnimationTimer(Timer::from_seconds(0.15, true)),
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 2.0))
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
