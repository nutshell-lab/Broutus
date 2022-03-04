use bevy::prelude::Color as ColorBevy;
use bevy_egui::egui::Color32;

// https://coolors.co/c8122c-f7b538-388057-4e4187-5bd17d

pub struct Color(u8, u8, u8);

impl From<Color> for Color32 {
    fn from(color: Color) -> Color32 {
        Color32::from_rgb(color.0, color.1, color.2)
    }
}

impl From<Color> for ColorBevy {
    fn from(color: Color) -> ColorBevy {
        ColorBevy::rgba_u8(color.0, color.1, color.2, 255)
    }
}

pub const BG_TEXT: Color = Color(243, 243, 247);
pub const DEFAULT_BG: Color = Color(36, 35, 49);
pub const WHITE_BG: Color = Color(255, 255, 255);
pub const HIGHLIGHT_BORDER: Color = Color(195, 197, 213);
pub const END_TURN: Color = Color(247, 181, 56);
pub const HEALTH: Color = Color(200, 18, 44);
pub const ACTION_POINTS: Color = Color(247, 181, 56);
pub const MOVEMENT_POINTS: Color = Color(56, 128, 87);
pub const TEAM_A_COLOR: Color = Color(91, 195, 235);
pub const TEAM_B_COLOR: Color = Color(91, 209, 125);
pub const TEAM_SPEC_COLOR: Color = Color(255, 255, 255);
