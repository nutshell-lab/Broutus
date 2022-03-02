use super::GameState;
use bevy::prelude::*;

mod attribute;
mod turn;
mod warrior;
mod weapon;

use super::map::Map;
use super::map::MapPosition;
use super::map::MapQuery;
use super::map::MouseMapPosition;
use super::map::Tile;
use super::map::TileLeftClickedEvent;
use super::map::TileRightClickedEvent;
use super::map::Tiledmap;
use attribute::ActionPoints;
use attribute::Attribute;
use attribute::Health;
use attribute::MovementPoints;
use turn::TeamA;
use turn::TeamB;
use turn::Turn;
use turn::TurnEnd;
use turn::TurnStart;
use warrior::Warrior;
use warrior::WarriorBundle;
use weapon::Weapon;

pub use turn::show_turn_ui;
pub use warrior::animate_warrior_sprite;
pub use warrior::update_warrior_world_position;
pub use warrior::WarriorAssets;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attribute>()
            .register_type::<Health>()
            .register_type::<ActionPoints>()
            .register_type::<MovementPoints>()
            .add_event::<TurnStart>()
            .add_event::<TurnEnd>()
            .add_system_set(SystemSet::on_enter(GameState::ARENA).with_system(spawn_warriors))
            .add_system_set(
                SystemSet::on_update(GameState::ARENA)
                    .with_system(animate_warrior_sprite)
                    .with_system(update_warrior_world_position)
                    .with_system(reset_warrior_attributes_on_turn_end)
                    .with_system(handle_warrior_movement_on_click)
                    .with_system(handle_warrior_attack_on_click)
                    .with_system(show_turn_ui),
            )
            .add_system_set(
                SystemSet::on_update(GameState::ARENA)
                    .label("clean_highlithing")
                    .with_system(unhighlight_all_tiles),
            )
            .add_system_set(
                SystemSet::on_update(GameState::ARENA)
                    .after("clean_highlithing")
                    .with_system(highlight_warriors_tile)
                    .with_system(compute_and_highlight_path),
            );
    }
}

fn spawn_warriors(mut commands: Commands, warrior_assets: Res<WarriorAssets>) {
    // Spawn warriors
    let knight_blue = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Blue".to_string(),
            MapPosition::new(17, 5),
            -1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamA)
        .id();

    let knight_red = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Red".to_string(),
            MapPosition::new(2, 5),
            1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamB)
        .id();

    let knight_purple = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Purple".to_string(),
            MapPosition::new(4, 10),
            1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamB)
        .id();

    // Insert turn system resource
    commands.insert_resource(Turn {
        order: vec![knight_blue, knight_red, knight_purple],
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

    let mut team_a_color = Color::MIDNIGHT_BLUE;
    let mut team_b_color = Color::ORANGE_RED;

    for (entity, position) in warriors_queryset.q0().iter() {
        let alpha = current
            .map(|e| if e.eq(&entity) { alpha } else { 0.7 })
            .unwrap_or(0.7);

        map_query.update_tile_sprite_color(
            map_id,
            layer_id,
            position,
            team_a_color.set_a(alpha).as_rgba(),
        );
    }

    for (entity, position) in warriors_queryset.q1().iter() {
        let alpha = current
            .map(|e| if e.eq(&entity) { alpha } else { 0.7 })
            .unwrap_or(0.7);

        map_query.update_tile_sprite_color(
            map_id,
            layer_id,
            position,
            team_b_color.set_a(alpha).as_rgba(),
        );
    }
}

/// Compute and highlight the path to the mouse position from the current warrior
fn compute_and_highlight_path(
    turn: Res<Turn>,
    mouse_position: Res<MouseMapPosition>,
    tiledmap_map: Res<Assets<Tiledmap>>,
    tiledmap_query: Query<&Handle<Tiledmap>, With<Map>>,
    warrior_query: Query<(&MapPosition, &MovementPoints), With<Warrior>>,
    mut map_query: MapQuery,
) {
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
                                    Color::rgba(0.18, 0.55, 0.34, 0.7),
                                );
                            }
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
        ap.0.reset();
        mp.0.reset();
    }
}

// /// Move the warrior on click if he can afford the cost of the path in movement points
fn handle_warrior_movement_on_click(
    mut ev_clicked: EventReader<TileLeftClickedEvent>,
    turn: Res<Turn>,
    tiledmap_map: Res<Assets<Tiledmap>>,
    tiledmap_query: Query<&Handle<Tiledmap>, With<Map>>,
    mut warrior_query: Query<
        (&mut MapPosition, &mut MovementPoints),
        (With<Warrior>, Without<Tile>),
    >,
    mut map_query: MapQuery,
) {
    if tiledmap_query.is_empty() {
        return;
    }

    let map_id = 0u32;
    let tiledmap_handle = tiledmap_query.single();
    let tiledmap_map = &tiledmap_map.get(tiledmap_handle);

    for ev in ev_clicked.iter() {
        let warrior_entity = turn.get_current_warrior_entity().unwrap();
        if let Ok((mut warrior_position, mut movement_points)) =
            warrior_query.get_mut(warrior_entity)
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
                    if cost <= movement_points.0.value {
                        warrior_position.x = ev.0.x;
                        warrior_position.y = ev.0.y;
                        movement_points.0.value -= cost;
                    }
                }
            }
        }
    }
}

fn handle_warrior_attack_on_click(
    mut ev_clicked: EventReader<TileRightClickedEvent>,
    turn: Res<Turn>,
    mut warrior_query: Query<(&MapPosition, &Warrior, &Weapon, &mut Health)>,
) {
    let warrior_entity = turn.get_current_warrior_entity().unwrap();
    let (_pos, _warrior, weapon, _h) = warrior_query.get(warrior_entity).unwrap();

    if let Some(target_location) = ev_clicked.iter().next() {
        for (_, _, _, mut health) in warrior_query
            .iter_mut()
            .filter(|(&warrior_position, _c, _weapon, _h)| warrior_position == target_location.0)
        {
            weapon.use_on(&mut health);
        }
    }
}
