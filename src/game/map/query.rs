use crate::game::gameplay::Warrior;

use super::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct MapQuery<'w, 's> {
    pub map: Query<'w, 's, &'static Map>,
    pub tile_queryset: QuerySet<
        'w,
        's,
        (
            QueryState<
                (
                    Entity,
                    &'static LayerIndex,
                    &'static MapPosition,
                    &'static mut TextureAtlasSprite,
                    &'static mut Visibility,
                ),
                (With<Tile>, Without<Warrior>),
            >,
            QueryState<
                (
                    Entity,
                    &'static LayerIndex,
                    &'static MapPosition,
                    &'static TextureAtlasSprite,
                    &'static Visibility,
                ),
                (With<Tile>, Without<Warrior>),
            >,
        ),
    >,
}

impl<'w, 's> MapQuery<'w, 's> {
    /// Get the tile Entity at the given position for the given map_id and layer_id
    pub fn hide_all_tiles(&mut self, map_id: u32, layer_id: u32) {
        for map in self.map.iter() {
            if map.id.ne(&map_id) {
                continue;
            }

            let mut query = self.tile_queryset.q0();
            for ((layer, _, _), &tile_entity) in map.tiles.iter() {
                if layer.ne(&layer_id) {
                    continue;
                }

                let (_, _, _, _, mut visibility) = query.get_mut(tile_entity).unwrap();
                visibility.is_visible = false;
            }
        }
    }

    /// Get the tile Entity at the given position for the given map_id and layer_id
    pub fn update_tile_sprite_color(
        &mut self,
        map_id: u32,
        layer_id: u32,
        position: &MapPosition,
        color: Color,
    ) -> Option<Entity> {
        for map in self.map.iter() {
            if map.id.ne(&map_id) {
                continue;
            }

            let mut query = self.tile_queryset.q0();
            if let Some(tile_entity) = map.tiles.get(&(layer_id, position.x, position.y)) {
                let (_, _, _, mut sprite, mut visibility) = query.get_mut(*tile_entity).unwrap();
                sprite.color = color;
                visibility.is_visible = true;
                return Some(*tile_entity);
            }
        }
        None
    }

    /// Return is a line of sight to the given position is blocked by an obstacle or not
    pub fn line_of_sight_check(
        &mut self,
        map_id: u32,
        me: &MapPosition,
        target: &MapPosition,
    ) -> bool {
        me.line_to(&target)
            .iter()
            .all(|position| !self.is_obstacle(map_id, position))
    }

    /// Is a map position an obstacle ?
    pub fn is_obstacle(&mut self, map_id: u32, position: &MapPosition) -> bool {
        let obstacle_layer_id = 2u32;

        for map in self.map.iter() {
            if map.id.ne(&map_id) {
                continue;
            }
            if !position.is_in_map_bounds(map.width, map.height) {
                return true;
            }

            return map
                .tiles
                .get(&(obstacle_layer_id, position.x, position.y))
                .is_some();
        }
        false
    }

    /// Get the list of tile neightbours at the given position
    pub fn non_obstacle_tile_neightbours(
        &mut self,
        map_id: u32,
        position: &MapPosition,
        map_width: u32,
        map_height: u32,
    ) -> Vec<(MapPosition, u32)> {
        #[rustfmt::skip]
        let neightbours = vec![
            MapPosition::new(position.x, position.y.wrapping_add(1)), // Up
            MapPosition::new(position.x.wrapping_sub(1), position.y), // Left
            MapPosition::new(position.x.wrapping_add(1), position.y),  // Right
            MapPosition::new(position.x, position.y.wrapping_sub(1)), // Down
        ];

        neightbours
            .iter()
            .filter(|&position| position.is_in_map_bounds(map_width, map_height))
            .filter(|&position| !self.is_obstacle(map_id, position))
            .map(|&position| (position, 1))
            .collect()
    }

    /// Compute optimal path between two positions, avoiding obstacles, returning the path and it's cost
    pub fn pathfinding(
        &mut self,
        map_id: u32,
        start: &MapPosition,
        end: &MapPosition,
        map_width: u32,
        map_height: u32,
    ) -> Option<(Vec<MapPosition>, u32)> {
        pathfinding::prelude::dijkstra(
            start,
            |position| self.non_obstacle_tile_neightbours(map_id, position, map_width, map_height),
            |position| position.eq(end),
        )
    }
}
