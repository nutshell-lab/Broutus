use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::collections::HashMap;

use super::mouse::MouseMapPosition;

pub struct TileClickedEvent(TilePos, HashMap<u16, Option<Entity>>);

pub fn detect_mouse_tile_events(
    mut ev_clicked: EventWriter<TileClickedEvent>,
    position: Res<MouseMapPosition>,
    buttons: Res<Input<MouseButton>>,
    mut map_query: MapQuery,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = position.0 {
            let mut entities = HashMap::<u16, Option<Entity>>::default();

            for layer_id in 0..5u16 {
                entities.insert(
                    layer_id,
                    map_query.get_tile_entity(position, 0u16, layer_id).ok(),
                );
            }

            ev_clicked.send(TileClickedEvent(position, entities));
        }
    }
}

// pub fn debug_mouse_tile_clicks(
//     mut ev_clicked: EventReader<TileClickedEvent>,
// ) {
//     for ev in ev_clicked.iter() {
//         eprintln!("Tile {:?} clicked!", ev.0);
//     }
// }
