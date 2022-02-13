use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct MouseInspector;

impl Plugin for MouseInspector {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_coords_ui);
    }
}

pub fn mouse_coords_ui(mut egui_context: ResMut<EguiContext>, windows: Res<Windows>) {
    let game_win = windows.get_primary().unwrap();

    if let Some(pos) = game_win.cursor_position() {
        let x = pos.x.to_string();
        let y = pos.y.to_string();
        egui::Window::new("Mouse Inspector").show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("{}, {}", x, y))
        });
    }
}
