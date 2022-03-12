use crate::game::color::Color;
use bevy::prelude::*;

#[derive(Default, Component, Copy, Clone)]
pub struct Team(TeamSide, Color);

impl Team {
    pub fn new(side: TeamSide, color: Color) -> Self {
        Self(side, color)
    }

    pub fn color(&self) -> Color {
        self.1
    }
}

#[derive(Copy, Clone)]
pub enum TeamSide {
    A,
    B,
}

impl Default for TeamSide {
    fn default() -> Self {
        Self::A
    }
}
