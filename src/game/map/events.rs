use super::MapPosition;
use super::MouseMapPosition;
use bevy::prelude::*;

pub struct TileClickedEvent(pub MapPosition);
pub struct TileRightClickedEvent(pub MapPosition);

pub fn trigger_map_mouse_events(
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
