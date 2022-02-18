use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

// TODO merge character and gameplay into a single module, clean modules exposed API
mod character;
mod gameplay;
mod health;
mod map;
mod weapon;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(map::MapPlugin)
            .add_event::<gameplay::TurnStart>()
            .add_event::<gameplay::TurnEnd>()
            .add_startup_system(setup_camera)
            .add_startup_system(setup_gameplay)
            .add_system(character::animate_sprite)
            .add_system(character::snap_to_map)
            .add_system(gameplay::debug_ui_turn)
            .add_system(unhighlight_all_tiles.before("tile_highlighting"))
            .add_system(reset_start_on_turn_end)
            .add_system(handle_map_click)
            .add_system_set(
                SystemSet::new()
                    .label("tile_highlighting")
                    .with_system(highlight_mouse_tile)
                    .with_system(compute_and_highlight_path)
                    .with_system(highlight_characters_tile),
            )
            .register_type::<character::AnimationTimer>()
            .register_type::<character::ActionPoints>()
            .register_type::<character::MovementPoints>();
    }
}

/// Spawn the main camera
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(265.0, -655.0, 1000.0 - 0.1),
        ..OrthographicCameraBundle::new_2d()
    });
}

// Setup all gameplay related stuff
fn setup_gameplay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load characters animation spritesheet
    let texture_handle = asset_server.load("characters/knight_idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 15, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn characters
    let knight_blue = commands
        .spawn_bundle(character::CharacterBundle::new(
            "Knight Blue".to_string(),
            map::TilePos(17, 5),
            -1.0,
            &texture_atlas_handle,
        ))
        .insert(gameplay::TeamA)
        .id();

    let knight_red = commands
        .spawn_bundle(character::CharacterBundle::new(
            "Knight Red".to_string(),
            map::TilePos(2, 5),
            1.0,
            &texture_atlas_handle,
        ))
        .insert(gameplay::TeamB)
        .id();

    let knight_purple = commands
        .spawn_bundle(character::CharacterBundle::new(
            "Knight Purple".to_string(),
            map::TilePos(2, 7),
            1.0,
            &texture_atlas_handle,
        ))
        .insert(gameplay::TeamB)
        .id();

    // Insert turn system resource
    commands.insert_resource(gameplay::Turn {
        order: vec![knight_blue, knight_red, knight_purple],
        ..Default::default()
    })
}

/// Highlight the tile hovered by the mouse
fn highlight_mouse_tile(
    position: Res<map::MouseMapPosition>,
    mut map_query: map::MapQuery,
    mut tile_query: Query<&mut map::Tile>,
) {
    if position.is_changed() {
        if let Some(position) = position.0 {
            let color = if map::is_obstacle(&mut map_query, position) {
                Color::GRAY
            } else {
                Color::SEA_GREEN
            };

            map::highlight_tile(&mut map_query, &mut tile_query, position, color);
        }
    }
}

/// Highlight the character tile based on their team
fn highlight_characters_tile(
    mut characters_queryset: QuerySet<(
        QueryState<&map::TilePos, With<gameplay::TeamA>>,
        QueryState<&map::TilePos, With<gameplay::TeamB>>,
    )>,
    mut map_query: map::MapQuery,
    mut tile_query: Query<&mut map::Tile>,
) {
    for position in characters_queryset.q0().iter() {
        map::highlight_tile(&mut map_query, &mut tile_query, *position, Color::BLUE);
    }

    for position in characters_queryset.q1().iter() {
        map::highlight_tile(&mut map_query, &mut tile_query, *position, Color::RED);
    }
}

/// Unhighlight all the tiles to prepare the highlighting phase
fn unhighlight_all_tiles(
    tmx_map: Res<Assets<map::TmxMap>>,
    tmx_query: Query<&Handle<map::TmxMap>, With<map::Map>>,
    mut map_query: map::MapQuery,
    mut tile_query: Query<&mut map::Tile>,
) {
    if tmx_query.is_empty() {
        return;
    }

    let tmx_handle = tmx_query.single();
    let tmx_map = &tmx_map.get(tmx_handle);

    if let Some(tmx_map) = tmx_map {
        for x in 0..tmx_map.map.width {
            for y in 0..tmx_map.map.height {
                map::highlight_tile(
                    &mut map_query,
                    &mut tile_query,
                    map::TilePos(x, y),
                    Color::WHITE,
                );
            }
        }
    }
}

/// Compute and highlight the path to the mouse position from the current character
fn compute_and_highlight_path(
    turn: Res<gameplay::Turn>,
    mouse_position: Res<map::MouseMapPosition>,
    tmx_map: Res<Assets<map::TmxMap>>,
    tmx_query: Query<&Handle<map::TmxMap>, With<map::Map>>,
    mut map_query: map::MapQuery,
    mut tile_query: Query<&mut map::Tile>,
    character_query: Query<(&map::TilePos, &character::MovementPoints), With<character::Character>>,
) {
    if tmx_query.is_empty() {
        return;
    }

    let tmx_handle = tmx_query.single();
    let tmx_map = &tmx_map.get(tmx_handle);

    let character_position = turn
        .get_current_character_entity()
        .and_then(|e| character_query.get(e).ok());

    if let Some((character_position, movement_points)) = character_position {
        if let Some(tmx_map) = tmx_map {
            if mouse_position.is_changed() {
                if let Some(mouse_position) = mouse_position.0 {
                    let path = map::path(
                        &mut map_query,
                        *character_position,
                        mouse_position,
                        tmx_map.map.width,
                        tmx_map.map.height,
                    );

                    if let Some((path, _cost)) = path {
                        for position in path.iter().take(movement_points.0 as usize + 1) {
                            map::highlight_tile(
                                &mut map_query,
                                &mut tile_query,
                                *position,
                                Color::GREEN,
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Reset character action & movement points at the end of their turn
fn reset_start_on_turn_end(
    mut ev_turn_ended: EventReader<gameplay::TurnEnd>,
    mut q: Query<
        (&mut character::ActionPoints, &mut character::MovementPoints),
        With<character::Character>,
    >,
) {
    for ev in ev_turn_ended.iter() {
        let (mut ap, mut mp) = q.get_mut(ev.0).unwrap();
        ap.reset();
        mp.reset();
    }
}

/// Move the character on click if he can afford the cost of the path in movement points
fn handle_map_click(
    mut ev_clicked: EventReader<map::TileClickedEvent>,
    turn: Res<gameplay::Turn>,
    tmx_map: Res<Assets<map::TmxMap>>,
    tmx_query: Query<&Handle<map::TmxMap>, With<map::Map>>,
    mut map_query: map::MapQuery,
    mut character_query: Query<
        (&mut map::TilePos, &mut character::MovementPoints),
        With<character::Character>,
    >,
) {
    if tmx_query.is_empty() {
        return;
    }

    let tmx_handle = tmx_query.single();
    let tmx_map = &tmx_map.get(tmx_handle);

    for ev in ev_clicked.iter() {
        let character_entity = turn.get_current_character_entity().unwrap();
        if let Ok((mut character_position, mut movement_points)) =
            character_query.get_mut(character_entity)
        {
            if let Some(tmx_map) = tmx_map {
                let path = map::path(
                    &mut map_query,
                    *character_position,
                    ev.0,
                    tmx_map.map.width,
                    tmx_map.map.height,
                );

                // TODO Replace the current sprite sheets by another one containing all 4 directions
                // TODO Animate character movement along the path
                // TODO Change character orientation when it changes direction
                if let Some((_path, cost)) = path {
                    if cost <= movement_points.0 {
                        character_position.0 = ev.0 .0;
                        character_position.1 = ev.0 .1;
                        movement_points.0 -= cost;
                    }
                }
            }
        }
    }
}
