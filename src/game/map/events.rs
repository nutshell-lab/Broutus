use super::mouse::MouseMapPosition;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct TileClickedEvent(pub TilePos);
pub struct TileRightClickedEvent(pub TilePos);

pub fn detect_tile_clicked_events(
    mut ev_left_clicked: EventWriter<TileClickedEvent>,
    mut ev_right_clicked: EventWriter<TileRightClickedEvent>,
    position: Res<MouseMapPosition>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = position.0 {
            ev_left_clicked.send(TileClickedEvent(position));
        }
    } else if buttons.just_pressed(MouseButton::Right) {
        if let Some(position) = position.0 {
            ev_right_clicked.send(TileRightClickedEvent(position));
        }
    }
}

// pub fn debug_tile_clicked_events(
//     mut ev_clicked: EventReader<TileClickedEvent>,
// ) {
//     for ev in ev_clicked.iter() {
//         eprintln!("Tile {:?} clicked!", ev.0);
//     }
// }
