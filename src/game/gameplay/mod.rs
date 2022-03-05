use super::color;
use super::GameState;
use bevy::prelude::*;

mod attribute;
mod turn;
mod warrior;
mod weapon;

pub use super::map::Map;
pub use super::map::MapPosition;
pub use super::map::MapQuery;
pub use super::map::MouseMapPosition;
pub use super::map::Tile;
pub use super::map::TileLeftClickedEvent;
pub use super::map::TileRightClickedEvent;
pub use super::map::Tiledmap;
pub use attribute::ActionPoints;
pub use attribute::Attribute;
pub use attribute::Health;
pub use attribute::MovementPoints;
pub use turn::reset_turn_timer;
pub use turn::run_turn_timer;
pub use turn::TeamA;
pub use turn::TeamB;
pub use turn::Turn;
pub use turn::TurnEnd;
pub use turn::TurnStart;
pub use turn::TurnTimer;
pub use warrior::animate_warrior_sprite;
pub use warrior::update_warrior_world_position;
pub use warrior::Warrior;
pub use warrior::WarriorAssets;
pub use warrior::WarriorBundle;
pub use weapon::SelectedAction;
pub use weapon::Weapon;
pub use weapon::HEAL_WAND;
pub use weapon::THUG_KNIFE;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attribute>()
            .register_type::<Health>()
            .register_type::<ActionPoints>()
            .register_type::<MovementPoints>()
            .init_resource::<SelectedAction>()
            .add_event::<TurnStart>()
            .add_event::<TurnEnd>()
            .add_system_set(SystemSet::on_enter(GameState::Arena).with_system(spawn_warriors))
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .with_system(run_turn_timer)
                    .with_system(reset_turn_timer)
                    .with_system(animate_warrior_sprite)
                    .with_system(update_warrior_world_position)
                    .with_system(reset_warrior_attributes_on_turn_end)
                    .with_system(handle_warrior_action_on_click)
                    .with_system(despawn_warrior_on_death),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .label("clean_highlithing")
                    .with_system(unhighlight_all_tiles),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .after("clean_highlithing")
                    .with_system(highlight_warriors_tile)
                    .with_system(highlight_potential_movement)
                    .with_system(highlight_potential_action)
                    .with_system(compute_and_highlight_path),
            );
    }
}

fn spawn_warriors(mut commands: Commands, warrior_assets: Res<WarriorAssets>) {
    // Spawn warriors
    let brundal = commands
        .spawn_bundle(WarriorBundle::new(
            "Brundal".to_string(),
            MapPosition::new(17, 5),
            &THUG_KNIFE,
            -1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamA)
        .id();

    let brandy = commands
        .spawn_bundle(WarriorBundle::new(
            "Brandy".to_string(),
            MapPosition::new(17, 10),
            &HEAL_WAND,
            -1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamA)
        .id();

    let brando = commands
        .spawn_bundle(WarriorBundle::new(
            "Brando".to_string(),
            MapPosition::new(17, 2),
            &THUG_KNIFE,
            -1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamA)
        .id();

    let glourf = commands
        .spawn_bundle(WarriorBundle::new(
            "Glourf".to_string(),
            MapPosition::new(2, 5),
            &THUG_KNIFE,
            1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamB)
        .id();

    let glarf = commands
        .spawn_bundle(WarriorBundle::new(
            "Glarf".to_string(),
            MapPosition::new(2, 1),
            &HEAL_WAND,
            1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamB)
        .id();

    let glirf = commands
        .spawn_bundle(WarriorBundle::new(
            "Glirf".to_string(),
            MapPosition::new(2, 8),
            &THUG_KNIFE,
            1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamB)
        .id();

    // Insert turn system resource
    commands.insert_resource(TurnTimer::default());
    commands.insert_resource(Turn {
        order: vec![brundal, glourf, brandy, glarf, brando, glirf],
        ..Default::default()
    })
}

/// Clean tile highlighting with white color
fn unhighlight_all_tiles(mut map_query: MapQuery) {
    let map_id = 0u32;
    let layer_id = 1u32;
    map_query.hide_all_tiles(map_id, layer_id);
}

/// Set tile tiles to the warrior team color at the higghtlith layer
fn highlight_warriors_tile(
    time: Res<Time>,
    turn: Res<Turn>,
    mut warriors_queryset: QuerySet<(
        QueryState<(Entity, &MapPosition), With<TeamA>>,
        QueryState<(Entity, &MapPosition), With<TeamB>>,
    )>,
    mut map_query: MapQuery,
) {
    let map_id = 0u32;
    let layer_id = 1u32;

    let current = turn.get_current_warrior_entity();
    let alpha = (((time.seconds_since_startup() * 4.0).sin() + 1.0) / 2.85) as f32;

    for (entity, position) in warriors_queryset.q0().iter() {
        let alpha = current
            .map(|e| if e.eq(&entity) { alpha } else { 0.8 })
            .unwrap_or(0.8);
        let mut color: bevy::prelude::Color = color::TEAM_A_COLOR.into();

        map_query.update_tile_sprite_color(
            map_id,
            layer_id,
            position,
            color.set_a(alpha).as_rgba(),
        );
    }

    for (entity, position) in warriors_queryset.q1().iter() {
        let alpha = current
            .map(|e| if e.eq(&entity) { alpha } else { 0.8 })
            .unwrap_or(0.8);
        let mut color: bevy::prelude::Color = color::TEAM_B_COLOR.into();

        map_query.update_tile_sprite_color(
            map_id,
            layer_id,
            position,
            color.set_a(alpha).as_rgba(),
        );
    }
}

/// Compute and highlight the path to the mouse position from the current warrior
fn compute_and_highlight_path(
    turn: Res<Turn>,
    selected_action: Res<SelectedAction>,
    mouse_position: Res<MouseMapPosition>,
    tiledmap_map: Res<Assets<Tiledmap>>,
    tiledmap_query: Query<&Handle<Tiledmap>, With<Map>>,
    warrior_query: Query<(&MapPosition, &MovementPoints), With<Warrior>>,
    mut map_query: MapQuery,
) {
    // An action is selected, don't highlight path
    // Or the attack system just deselected action because it was triggered
    if selected_action.0.is_some() || selected_action.is_changed() {
        return;
    }

    if tiledmap_query.is_empty() {
        return;
    }

    let map_id = 0u32;
    let layer_id = 1u32;

    let tiledmap_handle = tiledmap_query.single();
    let tiledmap_map = &tiledmap_map.get(tiledmap_handle);

    let warrior_position = turn
        .get_current_warrior_entity()
        .and_then(|e| warrior_query.get(e).ok());

    if let Some((warrior_position, movement_points)) = warrior_position {
        if let Some(tiledmap_map) = tiledmap_map {
            if mouse_position.is_changed() {
                if let Some(mouse_position) = mouse_position.0 {
                    let path = map_query.pathfinding(
                        map_id,
                        warrior_position,
                        &mouse_position,
                        tiledmap_map.inner.width,
                        tiledmap_map.inner.height,
                    );

                    if let Some((path, cost)) = path {
                        if cost <= movement_points.0.value {
                            for position in path
                                .iter()
                                .skip(1)
                                .take(movement_points.0.value as usize + 1)
                            {
                                map_query.update_tile_sprite_color(
                                    map_id,
                                    layer_id,
                                    position,
                                    bevy::render::color::Color::from(color::MOVEMENT_POINTS)
                                        .set_a(0.8)
                                        .as_rgba(),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A fucking inefficient function to highlight reachable MapPositions for the hovered Warrior
fn highlight_potential_movement(
    mouse_position: Res<MouseMapPosition>,
    selected_action: Res<SelectedAction>,
    tiledmap_map: Res<Assets<Tiledmap>>,
    tiledmap_query: Query<&Handle<Tiledmap>, With<Map>>,
    warrior_query: Query<(&MapPosition, &MovementPoints), With<Warrior>>,
    mut map_query: MapQuery,
) {
    // An action is selected, don't highlight path
    if selected_action.0.is_some() {
        return;
    }
    if tiledmap_query.is_empty() {
        return;
    }

    let map_id = 0u32;
    let layer_id = 1u32;

    let tiledmap_handle = tiledmap_query.single();
    let tiledmap_map = &tiledmap_map.get(tiledmap_handle);

    for (warrior_position, movement_points) in warrior_query.iter() {
        if let Some(tiledmap_map) = tiledmap_map {
            if mouse_position.is_changed() {
                if let Some(mouse_position) = mouse_position.0 {
                    // The mouse is over a warrior, let's highlight it's potential movement
                    if mouse_position.eq(warrior_position) {
                        let surroundings = warrior_position.get_surrounding_positions(
                            movement_points.0.value,
                            tiledmap_map.inner.width,
                            tiledmap_map.inner.height,
                        );

                        for position in surroundings {
                            // Yes that is horrible
                            let path = map_query.pathfinding(
                                map_id,
                                warrior_position,
                                &position,
                                tiledmap_map.inner.width,
                                tiledmap_map.inner.height,
                            );

                            if let Some((_, cost)) = path {
                                if cost <= movement_points.0.value {
                                    map_query.update_tile_sprite_color(
                                        map_id,
                                        layer_id,
                                        &position,
                                        bevy::render::color::Color::from(color::MOVEMENT_POINTS)
                                            .set_a(0.6)
                                            .as_rgba(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// /// Move the warrior on click if he can afford the cost of the path in movement points
fn handle_warrior_action_on_click(
    mut ev_clicked: EventReader<TileLeftClickedEvent>,
    mut selected_action: ResMut<SelectedAction>,
    turn: Res<Turn>,
    tiledmap_map: Res<Assets<Tiledmap>>,
    tiledmap_query: Query<&Handle<Tiledmap>, With<Map>>,
    mut warrior_query: QuerySet<(
        QueryState<
            (
                &Weapon,
                &mut MapPosition,
                &mut ActionPoints,
                &mut MovementPoints,
            ),
            (With<Warrior>, Without<Tile>),
        >, // Initiator
        QueryState<(&MapPosition, &mut Health), (With<Warrior>, Without<Tile>)>, // Potential targets
    )>,
    mut map_query: MapQuery,
) {
    if tiledmap_query.is_empty() {
        return;
    }

    if selected_action.0.is_some() {
        let action_distance = 2; // TODO replace with real action logic

        for click_event in ev_clicked.iter() {
            let warrior_entity = turn.get_current_warrior_entity().unwrap();
            let mut attacker_query = warrior_query.q0();
            let (weapon, position, mut action_points, _) =
                attacker_query.get_mut(warrior_entity).unwrap();

            if position.distance_to(&click_event.0) > action_distance {
                continue;
            }

            if !map_query.line_of_sight_check(0u32, &position, &click_event.0) {
                continue;
            }

            if action_points.can_spend(weapon.effect.ap_cost) {
                action_points.spend(weapon.effect.ap_cost);

                let weapon = *weapon; // Cannot get both queries as mutable at the same time :(
                for (position, mut health) in warrior_query.q1().iter_mut() {
                    if click_event.0.eq(position) {
                        weapon.use_on(&mut health);
                    }
                }
            }

            selected_action.0 = None; // Deselect action automatically
        }
    } else {
        let map_id = 0u32;
        let tiledmap_handle = tiledmap_query.single();
        let tiledmap_map = &tiledmap_map.get(tiledmap_handle);

        for ev in ev_clicked.iter() {
            let warrior_entity = turn.get_current_warrior_entity().unwrap();
            if let Ok((_, mut warrior_position, _, mut movement_points)) =
                warrior_query.q0().get_mut(warrior_entity)
            {
                if let Some(tiledmap_map) = tiledmap_map {
                    let path = map_query.pathfinding(
                        map_id,
                        &warrior_position,
                        &ev.0,
                        tiledmap_map.inner.width,
                        tiledmap_map.inner.height,
                    );

                    // TODO Replace the current sprite sheets by another one containing all 4 directions
                    // TODO Animate warrior movement along the path
                    // TODO Change warrior orientation when it changes direction
                    if let Some((_path, cost)) = path {
                        if movement_points.can_spend(cost) {
                            warrior_position.x = ev.0.x;
                            warrior_position.y = ev.0.y;
                            movement_points.spend(cost);
                        }
                    }
                }
            }
        }
    }
}

/// Reset warrior action & movement points at the end of their turn
fn reset_warrior_attributes_on_turn_end(
    mut ev_turn_ended: EventReader<TurnEnd>,
    mut q: Query<(&mut ActionPoints, &mut MovementPoints), With<Warrior>>,
) {
    for ev in ev_turn_ended.iter() {
        let (mut ap, mut mp) = q.get_mut(ev.0).unwrap();
        ap.reset();
        mp.reset();
    }
}

/// Highlight the targetable cells with the current action
fn highlight_potential_action(
    turn: Res<Turn>,
    mouse_position: Res<MouseMapPosition>,
    selected_action: Res<SelectedAction>,
    tiledmap_map: Res<Assets<Tiledmap>>,
    tiledmap_query: Query<&Handle<Tiledmap>, With<Map>>,
    warrior_query: Query<&MapPosition, With<Warrior>>,
    mut map_query: MapQuery,
) {
    if selected_action.0.is_none() {
        return;
    }
    if tiledmap_query.is_empty() {
        return;
    }

    let action_distance = 2; // TODO replace with real action logic

    let map_id = 0u32;
    let layer_id = 1u32;

    let tiledmap_handle = tiledmap_query.single();
    let tiledmap_map = &tiledmap_map.get(tiledmap_handle);

    if let Some(tiledmap_map) = tiledmap_map {
        let warrior_entity = turn.get_current_warrior_entity().unwrap();
        let warrior_position = warrior_query.get(warrior_entity).unwrap();
        let surroundings = warrior_position.get_surrounding_positions(
            action_distance,
            tiledmap_map.inner.width,
            tiledmap_map.inner.height,
        );

        for position in surroundings {
            if map_query.line_of_sight_check(map_id, warrior_position, &position) {
                let alpha = mouse_position
                    .0
                    .filter(|mouse| mouse.eq(&position))
                    .map(|_| 0.9)
                    .unwrap_or(0.6);
                map_query.update_tile_sprite_color(
                    map_id,
                    layer_id,
                    &position,
                    bevy::render::color::Color::from(color::HEALTH)
                        .set_a(alpha)
                        .as_rgba(),
                );
            }
        }
    }
}

fn despawn_warrior_on_death(
    mut commands: Commands,
    mut turn: ResMut<Turn>,
    warrior_query: Query<(Entity, &Health), (With<Warrior>, Changed<Health>)>,
) {
    for (entity, health) in warrior_query.iter() {
        if health.0.value == 0 {
            let turn_index = turn.get_entity_index(entity);

            if let Some(turn_index) = turn_index {
                turn.order.remove(turn_index);
                turn.order_index = if turn.order_index >= turn_index {
                    turn.order_index.saturating_sub(1)
                } else {
                    turn.order_index
                }
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}
