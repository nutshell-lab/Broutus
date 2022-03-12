use bevy::prelude::Color as ColorBevy;
use bevy_egui::egui::Color32;

#[derive(Copy, Clone)]
pub struct Color(u8, u8, u8);

impl Default for Color {
    fn default() -> Self {
        Self(255, 255, 255)
    }
}

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
pub const WHITE_BG: Color = Color(255, 255, 255);
pub const DEFAULT_BG: Color = Color(36, 35, 49);
pub const DEFAULT_BG_LIGHTER: Color = Color(70, 68, 95);
pub const DEFAULT_BG_LIGHTEST: Color = Color(96, 93, 131);
pub const TEXT_LIGHT: Color = Color(231, 235, 238);
pub const HIGHLIGHT_BORDER: Color = Color(195, 197, 213);

pub const HEALTH: Color = Color(200, 18, 44);
pub const SHIELD: Color = Color(147, 205, 240);
pub const ACTION_POINTS: Color = Color(247, 181, 56);
pub const MOVEMENT_POINTS: Color = Color(74, 191, 178);

pub const TEAM_A_COLOR: Color = Color(91, 195, 235);
pub const TEAM_B_COLOR: Color = Color(91, 209, 125);

pub const ACTION_GOOD: Color = Color(4, 167, 119);
pub const ACTION_BAD: Color = Color(255, 113, 91);
pub const ACTION_NEUTRAL: Color = Color(203, 203, 212);
