use super::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct MapQuery<'w, 's> {
    pub map_queryset: QuerySet<
        'w,
        's,
        (
            QueryState<(Entity, &'static mut Map, &'static Children)>,
            QueryState<(Entity, &'static Map, &'static Children)>,
        ),
    >,
    pub layer_queryset: QuerySet<
        'w,
        's,
        (
            QueryState<(Entity, &'static mut Layer, &'static Children)>,
            QueryState<(Entity, &'static Layer, &'static Children)>,
        ),
    >,
    pub tile_queryset: QuerySet<
        'w,
        's,
        (
            QueryState<
                (
                    Entity,
                    &'static MapPosition,
                    &'static mut TextureAtlasSprite,
                    &'static mut Visibility,
                ),
                With<Tile>,
            >,
            QueryState<
                (
                    Entity,
                    &'static MapPosition,
                    &'static TextureAtlasSprite,
                    &'static Visibility,
                ),
                With<Tile>,
            >,
        ),
    >,
}

impl<'w, 's> MapQuery<'w, 's> {
    // pub fn get_map_entity(&mut self, map_id: u32) -> Option<Entity> {
    //     for (entity, map, _) in self.map_queryset.q1().iter() {
    //         if map.id.ne(&map_id) {
    //             continue;
    //         }

    //         return Some(entity);
    //     }
    //     None
    // }

    // pub fn get_map(&mut self, map_id: u32) -> Option<&Map> {
    //     for (entity, map, _) in self.map_queryset.q1().iter() {
    //         if map.id.ne(&map_id) {
    //             continue;
    //         }
    //         return Some(map);
    //     }
    //     None
    // }

    // pub fn get_layer_entity(&mut self, map_id: u32, layer_id: u32) -> Option<Entity> {
    //     for (_, map, layers) in self.map_queryset.q1().iter() {
    //         if map.id.ne(&map_id) {
    //             continue;
    //         }

    //         for (layer_entity, layer, _) in self.layer_queryset.q1().iter() {
    //             if layer.id.ne(&layer_id) {
    //                 continue;
    //             }
    //             if !layers.contains(&layer_entity) {
    //                 continue;
    //             }

    //             return Some(layer_entity);
    //         }
    //     }
    //     None
    // }

    /// Get the tile Entity at the given position for the given map_id and layer_id
    pub fn get_tile_entity_at(
        &mut self,
        map_id: u32,
        layer_id: u32,
        position: &MapPosition,
    ) -> Option<Entity> {
        for (_, map, layers) in self.map_queryset.q1().iter() {
            if map.id.ne(&map_id) {
                continue;
            }

            for (layer_entity, layer, tiles) in self.layer_queryset.q1().iter() {
                if layer.id.ne(&layer_id) {
                    continue;
                }
                if !layers.contains(&layer_entity) {
                    continue;
                }

                for (tile_entity, tile_position, _, _) in self.tile_queryset.q1().iter() {
                    if position.ne(tile_position) {
                        continue;
                    }
                    if !tiles.contains(&tile_entity) {
                        continue;
                    }

                    return Some(tile_entity);
                }
            }
        }
        None
    }

    /// Get the tile Entity at the given position for the given map_id and layer_id
    pub fn hide_all_tiles(&mut self, map_id: u32, layer_id: u32) {
        for (_, map, layers) in self.map_queryset.q1().iter() {
            if map.id.ne(&map_id) {
                continue;
            }

            for (layer_entity, layer, tiles) in self.layer_queryset.q1().iter() {
                if layer.id.ne(&layer_id) {
                    continue;
                }
                if !layers.contains(&layer_entity) {
                    continue;
                }

                for (tile_entity, _, _, mut visibility) in self.tile_queryset.q0().iter_mut() {
                    if !tiles.contains(&tile_entity) {
                        continue;
                    }

                    visibility.is_visible = false;
                }
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
        for (_, map, layers) in self.map_queryset.q1().iter() {
            if map.id.ne(&map_id) {
                continue;
            }

            for (layer_entity, layer, tiles) in self.layer_queryset.q1().iter() {
                if layer.id.ne(&layer_id) {
                    continue;
                }
                if !layers.contains(&layer_entity) {
                    continue;
                }

                for (tile_entity, tile_position, mut sprite, mut visibility) in
                    self.tile_queryset.q0().iter_mut()
                {
                    if !tiles.contains(&tile_entity) {
                        continue;
                    }
                    if position.ne(tile_position) {
                        continue;
                    }

                    sprite.color = color;
                    visibility.is_visible = true;
                    return Some(tile_entity);
                }
            }
        }
        None
    }

    /// Is a map position an obstacle ?
    pub fn is_obstacle(&mut self, map_id: u32, position: &MapPosition) -> bool {
        for (_, map, _) in self.map_queryset.q1().iter() {
            if map.id.ne(&map_id) {
                continue;
            }

            let obstacle_layer = map.obstacle_layer;
            let tile_entity = self.get_tile_entity_at(map_id, obstacle_layer, position);

            return tile_entity.is_some();
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
            .filter(|&position| position.x < map_width && position.y < map_height)
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
        pathfinding::prelude::astar(
            start,
            |position| self.non_obstacle_tile_neightbours(map_id, position, map_width, map_height),
            |current| tile_distance(current, end),
            |position| position.eq(end),
        )
    }
}
