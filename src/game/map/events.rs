use super::mouse::MouseMapPosition;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct TileClickedEvent(pub TilePos);

pub fn detect_tile_clicked_events(
    mut ev_clicked: EventWriter<TileClickedEvent>,
    position: Res<MouseMapPosition>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = position.0 {
            ev_clicked.send(TileClickedEvent(position));
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
