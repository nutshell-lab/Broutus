use super::color;
use super::GameState;
use bevy::prelude::*;

mod team;
mod turn;
mod warrior;

use super::map::*;
pub use team::*;
pub use turn::*;
pub use warrior::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedAction>()
            .add_asset::<WarriorAsset>()
            .add_asset_loader(WarriorAssetLoader)
            .register_type::<SelectedAction>()
            .register_type::<SelectedAnimation>()
            .register_type::<AnimationTimer>()
            .add_event::<TurnStart>()
            .add_event::<TurnEnd>()
            .add_event::<Damage>()
            .add_event::<Heal>()
            .add_event::<DrainHealth>()
            .add_event::<Move>()
            .add_system_set(SystemSet::on_enter(GameState::Arena).with_system(spawn_warriors))
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .with_system(run_turn_timer)
                    .with_system(reset_turn_timer)
                    .with_system(animate_warrior_sprite)
                    .with_system(update_warrior_position)
                    .with_system(reset_warrior_attributes_on_turn_end)
                    .with_system(handle_warrior_action_on_click)
                    .with_system(apply_active_effects)
                    .with_system(process_damage_event)
                    .with_system(process_heal_event)
                    .with_system(process_drain_health_event)
                    .with_system(process_move_event)
                    .with_system(despawn_warrior_on_death),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .label("clean_highlithing")
                    .with_system(unhighlight_all_tiles),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .label("highlight_1")
                    .after("clean_highlithing")
                    .with_system(highlight_warriors_tile)
                    .with_system(highlight_potential_movement)
                    .with_system(compute_and_highlight_path),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Arena)
                    .label("highlight_2")
                    .after("highlight_1")
                    .with_system(highlight_potential_action),
            );
    }
}

fn spawn_warriors(
    mut commands: Commands,
    warrior_collection: Res<WarriorCollection>,
    warriors: Res<Assets<WarriorAsset>>,
    animation_collection: Res<AnimationCollection>,
) {
    let team_a = Team::new(TeamSide::A, color::TEAM_A_COLOR);
    let team_b = Team::new(TeamSide::B, color::TEAM_B_COLOR);

    let edificadores_asset = warriors
        .get(warrior_collection.warriors[0].clone())
        .unwrap();
    let ella_asset = warriors
        .get(warrior_collection.warriors[1].clone())
        .unwrap();

    // Spawn warriors
    let edificadores = commands
        .spawn_bundle(WarriorBundle::new(
            edificadores_asset,
            &animation_collection,
            &team_a,
            MapPosition::new(1, 1),
        ))
        .id();

    let ella = commands
        .spawn_bundle(WarriorBundle::new(
            ella_asset,
            &animation_collection,
            &team_b,
            MapPosition::new(14, 14),
        ))
        .id();

    // Insert turn system resource
    commands.insert_resource(TurnTimer::default());
    commands.insert_resource(Turn {
        order: vec![edificadores, ella],
        ..Default::default()
    })
}

/// Clean tile highlighting with white color
fn unhighlight_all_tiles(mut map_query: MapQuery) {
    map_query.hide_all_tiles(0u32, 1u32);
}

/// Set tile tiles to the warrior team color at the higghtlith layer
fn highlight_warriors_tile(
    time: Res<Time>,
    turn: Res<Turn>,
    warriors_query: Query<(Entity, &MapPosition, &Team), With<Warrior>>,
    mut map_query: MapQuery,
) {
    let map_id = 0u32;
    let layer_id = 1u32;

    let current = turn.get_current_warrior_entity();
    let alpha = (((time.seconds_since_startup() * 4.0).sin() + 1.0) / 2.85) as f32;

    for (entity, position, team) in warriors_query.iter() {
        let mut color: bevy::prelude::Color = team.color().into();
        let alpha = current
            .map(|e| if e.eq(&entity) { alpha } else { 0.8 })
            .unwrap_or(0.8);

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
    warrior_query: Query<(&MapPosition, &Attribute<MovementPoints>), With<Warrior>>,
    mut map_query: MapQuery,
) {
    // An action is selected, don't highlight path
    // Or the attack system just deselected action because it was triggered
    if selected_action.0.is_some() || selected_action.is_changed() {
        return;
    }

    let map = map_query.map.single();
    let map_id = map.id;
    let highlight_layer_id = 1; // map.highlight_layer;
    let map_width = map.width;
    let map_height = map.height;

    let warrior_position = turn
        .get_current_warrior_entity()
        .and_then(|e| warrior_query.get(e).ok());

    if let Some((warrior_position, movement_points)) = warrior_position {
        if mouse_position.is_changed() {
            if let Some(mouse_position) = mouse_position.0 {
                let path = map_query.pathfinding(
                    map_id,
                    warrior_position,
                    &mouse_position,
                    map_width,
                    map_height,
                );

                if let Some((path, cost)) = path {
                    if cost <= movement_points.value() {
                        for position in path
                            .iter()
                            .skip(1)
                            .take(movement_points.value() as usize + 1)
                        {
                            map_query.update_tile_sprite_color(
                                map_id,
                                highlight_layer_id,
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

/// A fucking inefficient function to highlight reachable MapPositions for the hovered Warrior
fn highlight_potential_movement(
    mouse_position: Res<MouseMapPosition>,
    selected_action: Res<SelectedAction>,
    warrior_query: Query<(&MapPosition, &Attribute<MovementPoints>), With<Warrior>>,
    mut map_query: MapQuery,
) {
    // An action is selected, don't highlight path
    if selected_action.0.is_some() {
        return;
    }

    let map = map_query.map.single();
    let map_id = map.id;
    let highlight_layer_id = 1; //map.highlight_layer;
    let map_width = map.width;
    let map_height = map.height;

    for (warrior_position, movement_points) in warrior_query.iter() {
        if mouse_position.is_changed() {
            if let Some(mouse_position) = mouse_position.0 {
                // The mouse is over a warrior, let's highlight it's potential movement
                if mouse_position.eq(warrior_position) {
                    let surroundings = warrior_position.get_surrounding_positions(
                        1,
                        movement_points.value(),
                        map_width,
                        map_height,
                    );

                    for position in surroundings {
                        // Yes that is horrible
                        let path = map_query.pathfinding(
                            map_id,
                            warrior_position,
                            &position,
                            map_width,
                            map_height,
                        );

                        if let Some((_, cost)) = path {
                            if cost <= movement_points.value() {
                                map_query.update_tile_sprite_color(
                                    map_id,
                                    highlight_layer_id,
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

/// Move the warrior on click if he can afford the cost of the path in movement points
fn handle_warrior_action_on_click(
    turn: Res<Turn>,
    mut ev_clicked: EventReader<TileLeftClickedEvent>,
    mut ev_warrior: WarriorEventWriterQuery,
    mut selected_action: ResMut<SelectedAction>,
    mut warrior_query: QuerySet<(
        QueryState<(Entity, &Warrior, &MapPosition)>,
        QueryState<(&MapPosition, &mut Attribute<ActionPoints>, &Actions)>,
        QueryState<(
            &MapPosition,
            &mut Attribute<MovementPoints>,
            &mut MapPositionPath,
        )>,
    )>,
    mut map_query: MapQuery,
) {
    let map = map_query.map.single();
    let map_id = map.id;
    let map_width = map.width;
    let map_height = map.height;

    if let Some(index) = selected_action.0 {
        for click_event in ev_clicked.iter() {
            let warrior_entity = turn.get_current_warrior_entity().unwrap();
            let mut query = warrior_query.q1();
            let (position, mut action_points, actions) = query.get_mut(warrior_entity).unwrap();

            let action = actions.0.get(index).cloned().unwrap();
            if !action.range.can_reach(&position, &click_event.0) {
                continue;
            }

            let ckeck_los = match action.range {
                ActionRange::Around { check_los, .. } => check_los,
                ActionRange::Line { check_los, .. } => check_los,
                ActionRange::Diagonal { check_los, .. } => check_los,
                _ => true,
            };
            if ckeck_los && !map_query.line_of_sight_check(map_id, &position, &click_event.0) {
                continue;
            }

            action_points.drop(action.cost.value());
            action.execute(
                &position.clone(),
                &click_event.0,
                &mut map_query,
                &mut warrior_query.q0(),
                &mut ev_warrior,
            );
            selected_action.0 = None; // Deselect action automatically
        }
    } else {
        for ev in ev_clicked.iter() {
            let warrior_entity = turn.get_current_warrior_entity().unwrap();
            if let Ok((warrior_position, mut movement_points, mut path)) =
                warrior_query.q2().get_mut(warrior_entity)
            {
                let computed_path =
                    map_query.pathfinding(map_id, &warrior_position, &ev.0, map_width, map_height);

                // TODO Replace the current sprite sheets by another one containing all 4 directions
                // TODO Animate warrior movement along the path
                // TODO Change warrior orientation when it changes direction
                if let Some((computed_path, cost)) = computed_path {
                    if movement_points.can_drop(cost) {
                        movement_points.drop(cost);
                        path.set(computed_path);
                    }
                }
            }
        }
    }
}

fn apply_active_effects(
    mut ev_turn_started: EventReader<TurnStart>,
    mut warrior_query: Query<(
        &Warrior,
        &Name,
        &mut MapPosition,
        &mut ActiveEffects,
        &mut Attribute<Health>,
        &mut Attribute<Shield>,
        &mut Attribute<ActionPoints>,
        &mut Attribute<MovementPoints>,
    )>,
) {
    for ev in ev_turn_started.iter() {
        let (_, _, _, mut effects, mut health, mut shield, ..) =
            warrior_query.get_mut(ev.0).unwrap();
        for effect in effects.0.iter_mut() {
            match effect {
                ActionEffect::DamageOverTime {
                    amount,
                    erode,
                    ref mut duration,
                } => {
                    if *duration > 0 {
                        let remaining = shield.drop(*amount);
                        health.drop(remaining);
                        health.erode(remaining, *erode);
                        *duration -= 1;
                    }
                }
                _ => (),
            }
        }
    }
}

/// Reset warrior action & movement points at the end of their turn
fn reset_warrior_attributes_on_turn_end(
    mut ev_turn_ended: EventReader<TurnEnd>,
    mut q: Query<(&mut Attribute<ActionPoints>, &mut Attribute<MovementPoints>), With<Warrior>>,
) {
    for ev in ev_turn_ended.iter() {
        let (mut ap, mut mp) = q.get_mut(ev.0).unwrap();
        ap.rise_max();
        mp.rise_max();
    }
}

/// Highlight the targetable cells with the current action
// TODO only compute on selected_action changed
fn highlight_potential_action(
    turn: Res<Turn>,
    mouse_position: Res<MouseMapPosition>,
    selected_action: Res<SelectedAction>,
    warrior_query: Query<(&MapPosition, &Actions), With<Warrior>>,
    mut map_query: MapQuery,
) {
    if selected_action.0.is_none() {
        return;
    }

    let map = map_query.map.single();
    let map_id = map.id;
    let highlight_layer_id = 1; //map.highlight_layer;

    let warrior_entity = turn.get_current_warrior_entity().unwrap();
    let (warrior_position, warrior_actions) = warrior_query.get(warrior_entity).unwrap();
    let action = warrior_actions
        .0
        .get(selected_action.0.unwrap())
        .cloned()
        .unwrap();

    let check_los = match action.range {
        ActionRange::Around { check_los, .. } => check_los,
        ActionRange::Line { check_los, .. } => check_los,
        ActionRange::Diagonal { check_los, .. } => check_los,
        _ => true,
    };

    let all_positions = map.all_positions();
    for position in all_positions {
        if action.range.can_reach(&warrior_position, &position)
            && (!check_los || map_query.line_of_sight_check(map_id, warrior_position, &position))
        {
            map_query.update_tile_sprite_color(
                map_id,
                highlight_layer_id,
                &position,
                bevy::render::color::Color::from(color::HEALTH)
                    .set_a(0.6)
                    .as_rgba(),
            );
        }
    }

    if let Some(mouse_position) = mouse_position.0 {
        let hit_positions = action
            .aoe
            .compute_hit_positions(&mouse_position, &mut map_query);

        if action.range.can_reach(&warrior_position, &mouse_position)
            && (!check_los
                || map_query.line_of_sight_check(map_id, warrior_position, &mouse_position))
        {
            for position in hit_positions {
                map_query.update_tile_sprite_color(
                    map_id,
                    highlight_layer_id,
                    &position,
                    bevy::render::color::Color::from(color::HEALTH)
                        .set_a(0.9)
                        .as_rgba(),
                );
            }
        }
    }
}

fn despawn_warrior_on_death(
    mut commands: Commands,
    mut turn: ResMut<Turn>,
    warrior_query: Query<(Entity, &Attribute<Health>), (With<Warrior>, Changed<Attribute<Health>>)>,
) {
    for (entity, health) in warrior_query.iter() {
        if health.value() == 0 {
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
