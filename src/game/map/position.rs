use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{LayerIndex, Map};

#[derive(
    Reflect, Component, Default, Debug, Clone, Copy, Hash, Eq, PartialEq, Deserialize, Serialize,
)]
#[reflect(Component)]
pub struct MapPosition {
    pub x: u32,
    pub y: u32,
}

impl MapPosition {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn to_relative_z(self, map_width: u32, map_height: u32) -> f32 {
        (self.x + self.y) as f32 / (map_width - 1 + map_height - 1) as f32
    }

    pub fn to_xyz(self, map: &Map, layer_index: &LayerIndex) -> Vec3 {
        let coords =
            super::super::map::project_iso(&self, map.tile_width as f32, map.tile_height as f32);

        Vec3::new(
            coords.x,
            coords.y,
            self.to_relative_z(map.width, map.height) + layer_index.0 as f32,
        )
    }

    pub fn distance_to(self, other: &MapPosition) -> u32 {
        let x_offset = if self.x < other.x {
            other.x - self.x
        } else {
            self.x - other.x
        };

        let y_offset = if self.y < other.y {
            other.y - self.y
        } else {
            self.y - other.y
        };

        x_offset + y_offset
    }

    /// Get the line path between two positions (supercover line).
    /// see: https://www.redblobgames.com/grids/line-drawing.html
    pub fn line_to(self, other: &MapPosition) -> Vec<MapPosition> {
        let (dx, dy) = (
            other.x as i32 - self.x as i32,
            other.y as i32 - self.y as i32,
        );
        let (nx, ny) = (dx.abs(), dy.abs());
        let (sign_x, sign_y) = (if dx > 0 { 1 } else { -1 }, if dy > 0 { 1 } else { -1 });

        let mut path = Vec::new();
        path.push(self);

        let (mut x, mut y) = (self.x as i32, self.y as i32);
        let (mut ix, mut iy) = (0, 0);
        while ix < nx || iy < ny {
            let decision = (1 + 2 * ix) * ny - (1 + 2 * iy) * nx;
            if decision == 0 {
                x += sign_x;
                y += sign_y;
                ix += 1;
                iy += 1;
            } else if decision < 0 {
                x += sign_x;
                ix += 1;
            } else {
                y += sign_y;
                iy += 1;
            }
            path.push(MapPosition::new(x as u32, y as u32));
        }

        path
    }

    /// Get all positions in the reach of the given position, excluding the given position
    pub fn get_surrounding_positions(
        self,
        min_distance: u32,
        max_distance: u32,
        map_width: u32,
        map_height: u32,
    ) -> Vec<MapPosition> {
        let mut positions = Vec::new();

        // Yes that is horrible
        for y in 0..map_height {
            for x in 0..map_width {
                let p = MapPosition::new(x, y);
                if p.distance_to(&self) >= min_distance && p.distance_to(&self) <= max_distance {
                    positions.push(p);
                }
            }
        }

        positions
    }

    /// Get all positions in the reach of the given position, excluding the given position
    pub fn is_in_map_bounds(self, map_width: u32, map_height: u32) -> bool {
        self.x < map_width && self.y < map_height
    }

    /// Get the direction of a target from the current position, only straight, no diagonals.
    pub fn direction_to(&self, target: &MapPosition) -> Option<MapPositionDirection> {
        if self.x == target.x && self.y < target.y {
            Some(MapPositionDirection::SudWest)
        } else if self.x == target.x && self.y > target.y {
            Some(MapPositionDirection::NordEst)
        } else if self.x < target.x && self.y == target.y {
            Some(MapPositionDirection::SudEst)
        } else if self.x > target.x && self.y == target.y {
            Some(MapPositionDirection::NordWest)
        } else {
            None
        }
    }

    /// Get a straight path torward a position from the current position, unchecked for obstacles.
    pub fn unchecked_path_torward(
        &self,
        direction: MapPositionDirection,
        distance: u32,
    ) -> Vec<MapPosition> {
        let mut distance = distance;
        let mut path = Vec::new();
        let (dx, dy) = match direction {
            MapPositionDirection::NordWest => (-1, 0),
            MapPositionDirection::NordEst => (0, -1),
            MapPositionDirection::SudWest => (0, 1),
            MapPositionDirection::SudEst => (1, 0),
        };

        let (mut x, mut y) = (self.x as i32, self.y as i32);
        while distance != 0 {
            x += dx;
            y += dy;
            path.push(MapPosition::new(x as u32, y as u32));
            distance -= 1;
        }

        path
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MapPositionDirection {
    NordWest, // top-left
    NordEst,  // top-right
    SudWest,  // bottom-left
    SudEst,   // bottom-right
}

impl From<MapPosition> for (u32, u32) {
    fn from(position: MapPosition) -> Self {
        (position.x, position.y)
    }
}

impl From<(u32, u32)> for MapPosition {
    fn from((x, y): (u32, u32)) -> Self {
        Self::new(x, y)
    }
}
