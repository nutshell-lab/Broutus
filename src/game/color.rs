use bevy_egui::egui::Color32;
use bevy::prelude::Color as ColorBevy;

// https://coolors.co/c8122c-f7b538-388057-4e4187-5bd17d

pub struct Color(u8, u8, u8);

impl Into<Color32> for Color {
    fn into(self) -> Color32 {
        Color32::from_rgb(self.0, self.1, self.2)
    }
}

impl Into<ColorBevy> for Color {
    fn into(self) -> ColorBevy {
        ColorBevy::rgba_u8(self.0, self.1, self.2, 255)
    }
}

pub const END_TURN: Color = Color(247, 181, 56);
pub const HEALTH: Color = Color(200, 18, 44);
pub const ACTION_POINTS: Color = Color(247, 181, 56);
pub const MOVEMENT_POINTS: Color = Color(56, 128, 87);
pub const TEAM_A_COLOR: Color = Color(78, 65, 135);
pub const TEAM_B_COLOR: Color = Color(91, 209, 125);
pub const TEAM_SPEC_COLOR: Color = Color(91, 209, 125);
