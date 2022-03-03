use super::attribute::*;
use super::turn::TeamA;
use super::turn::TeamB;
use super::turn::ToColor32;
use super::weapon::{Effect, EffectType, Weapon};
use super::Map;
use super::MapPosition;
use super::MouseMapPosition;
use super::Tiledmap;
use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_egui::{egui, EguiContext};

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
                Effect::new(10, (0, 1), 3, EffectType::Attack),
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

#[derive(AssetCollection)]
pub struct WarriorAssets {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 15, rows = 1))]
    #[asset(path = "warriors/knight_idle.png")]
    pub idle: Handle<TextureAtlas>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer); // TODO maybe improve this thing to support multiple animations (idle, run, attack...)

/// Animate the sprite based on the AnimationTimer
pub fn animate_warrior_sprite(
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

/// Display all infos about the turn system in a dedicated window
pub fn show_warrior_bubble_on_hover(
    windows: Res<Windows>,
    tiledmaps: Res<Assets<Tiledmap>>,
    mouse_position: Res<MouseMapPosition>,
    map_query: Query<&Handle<Tiledmap>, With<Map>>,
    warrior_query: Query<(Entity, &Name, &Health, &MapPosition), With<Warrior>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut egui_context: ResMut<EguiContext>,
    mut team_query: QuerySet<(
        QueryState<Entity, With<TeamA>>,
        QueryState<Entity, With<TeamB>>,
    )>,
) {
    if map_query.is_empty() {
        return;
    }
    if warrior_query.is_empty() {
        return;
    }

    if let Some(mouse_position) = mouse_position.0 {
        let tiledmap_handle = map_query.single();
        let tiledmap = tiledmaps.get(tiledmap_handle).unwrap();
        let (camera, camera_transform) = camera_query.single();

        for (entity, name, health, position) in warrior_query.iter() {
            if mouse_position.ne(position) {
                continue;
            }

            let world_position = position.to_xyz(
                0u32,
                tiledmap.inner.width,
                tiledmap.inner.height,
                tiledmap.inner.tile_width as f32,
                tiledmap.inner.tile_height as f32,
            );

            if let Some(hover_position) =
                camera.world_to_screen(windows.as_ref(), camera_transform, world_position)
            {
                let color = {
                    let is_team_a = team_query.q0().get(entity).is_ok();
                    let is_team_b = team_query.q1().get(entity).is_ok();

                    if is_team_a {
                        TeamA::to_color32()
                    } else if is_team_b {
                        TeamB::to_color32()
                    } else {
                        egui::Color32::LIGHT_GREEN
                    }
                };

                let main_window = windows.get_primary().unwrap();
                egui::containers::Window::new("WarriorMouseHover")
                    .collapsible(false)
                    .resizable(false)
                    .title_bar(false)
                    .fixed_size((150.0, 80.0))
                    .fixed_pos((
                        hover_position.x - 75.0,
                        (hover_position.y - main_window.height()) * -1.0 - 108.0,
                    ))
                    .frame(
                        egui::containers::Frame::default()
                            .fill(egui::Color32::from_rgb(19, 26, 38))
                            .stroke(egui::Stroke::new(
                                2.0,
                                egui::Color32::from_rgb(207, 209, 211),
                            ))
                            .margin((5.0, 5.0))
                            .corner_radius(5.0),
                    )
                    .show(egui_context.ctx_mut(), |ui| {
                        ui.label(egui::RichText::new(name.as_str()).color(color).heading());
                        ui.add(
                            egui::ProgressBar::new(health.0.value as f32 / health.0.max as f32)
                                .text(format!("{} / {} hp", health.0.value, health.0.max)),
                        )
                    });
            }
        }
    }
}
