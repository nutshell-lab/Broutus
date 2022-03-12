use crate::game::{
    color,
    map::{MapPosition, MapQuery},
};

use super::*;
use bevy::prelude::*;
use bevy_inspector_egui::egui;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SelectedAction(pub Option<usize>);

/// NewType representing a Warrior's action collection
#[derive(Debug, Clone, Deserialize, Serialize, Component)]
pub struct Actions(pub Vec<Action>);

impl Default for Actions {
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// NewType representing a Warrior's action collection
#[derive(Debug, Clone, Deserialize, Serialize, Component)]
pub struct ActiveEffects(pub Vec<ActionEffect>);

impl Default for ActiveEffects {
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// A Warrior action is usable in Arena mode, consuming ActionPoints
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    pub name: String,
    pub icon_key: String,
    pub animation_key: String,
    pub cost: ActionPoints,
    pub aoe: ActionAoe,
    pub range: ActionRange,
    pub effects: Vec<ActionEffect>,
}

impl Action {
    /// Execute all action effects one by one
    pub fn execute(
        &self,
        from_position: &MapPosition,
        to_position: &MapPosition,
        mut map_query: &mut MapQuery,
        warrior_query: &mut Query<(Entity, &Warrior, &MapPosition)>,
        ev_warrior: &mut events::WarriorEventWriterQuery,
    ) {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();

        let hit_positions = self.aoe.compute_hit_positions(to_position, &mut map_query);

        for effect in self.effects.iter() {
            match effect {
                &ActionEffect::Nothing => (),
                &ActionEffect::Damage {
                    amount,
                    crit_chance,
                    crit_mult,
                    erode,
                } => {
                    for (entity, _w, position) in warrior_query.iter() {
                        if hit_positions.contains(&position) {
                            let is_crit = crit_chance >= rng.gen();
                            let mutl = if is_crit { crit_mult } else { 1.0 };
                            let amount = (amount as f32 * mutl).round() as u32;
                            ev_warrior
                                .ew_damage
                                .send(events::Damage(entity, amount, erode));
                        }
                    }
                }
                &ActionEffect::DamageOverTime {
                    amount,
                    duration,
                    erode,
                } => {}
                &ActionEffect::Heal { amount } => {
                    for (entity, _w, position) in warrior_query.iter() {
                        if hit_positions.contains(&position) {
                            ev_warrior.ew_heal.send(events::Heal(entity, amount));
                        }
                    }
                }
                &ActionEffect::Shield { amount } => {
                    for (entity, _w, position) in warrior_query.iter() {
                        if hit_positions.contains(&position) {
                            ev_warrior.ew_shield.send(events::Shield(entity, amount));
                        }
                    }
                }
                &ActionEffect::RemoveActionPoints { amount } => {}
                &ActionEffect::RemoveMovementPoints { amount } => {}
                &ActionEffect::StealActionPoints { amount } => {}
                &ActionEffect::StealMovementPoints { amount } => {}
                &ActionEffect::Push { distance } => {}
                &ActionEffect::WalkTo { to } => (),
                &ActionEffect::TeleportSelf => {
                    let warrior = warrior_query
                        .iter()
                        .find(|(_, _, &position)| position.eq(from_position))
                        .unwrap();

                    let is_empty = warrior_query
                        .iter()
                        .filter(|(_, _, &position)| position.eq(to_position))
                        .count()
                        == 0;

                    if is_empty {
                        ev_warrior
                            .ew_move
                            .send(events::Move(warrior.0, *to_position));
                    }
                }
                &ActionEffect::TeleportSwitch => {
                    let left = warrior_query
                        .iter()
                        .find(|(_, _, &position)| position.eq(from_position))
                        .unwrap();

                    let right = warrior_query
                        .iter()
                        .find(|(_, _, &position)| position.eq(to_position));

                    if let Some(right) = right {
                        ev_warrior.ew_move.send(events::Move(left.0, *right.2));
                        ev_warrior.ew_move.send(events::Move(right.0, *left.2));
                    }
                }
            }
        }
    }

    pub fn show_tooltip_ui(&self, ui: &mut egui::Ui) {
        egui::show_tooltip(
            ui.ctx(),
            egui::Id::new(format!("action_tooltip_{}", self.name)),
            |ui| {
                egui::Grid::new(format!("action_tooltip_{}_grid", self.name)).show(ui, |mut ui| {
                    ui.label(egui::RichText::new(self.name.as_str()).heading());
                    ui.label(
                        egui::RichText::new(format!("★ {}", self.cost.value()))
                            .heading()
                            .color(color::ACTION_POINTS),
                    );
                    ui.end_row();

                    for effect in self.effects.iter() {
                        effect.show_description_ui(&mut ui);
                    }
                })
            },
        );
    }

    pub fn show_effects_ui(&self, mut ui: &mut egui::Ui) {
        for effect in self.effects.iter() {
            effect.show_description_ui(&mut ui);
        }
    }
}

/// The action Area Of Effect, representing the zone where the action effects will be applied
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum ActionAoe {
    /// ☐☐☐☐☐☐ <br/>
    /// ☐☐☐☐☐☐ <br/>
    /// ⇰☐☐☒☐☐ <br/>
    /// ☐☐☐☐☐☐ <br/>
    /// ☐☐☐☐☐☐ <br/>
    Cell,

    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☐☐☐<br/>
    /// ☐☐☒☒☒☐ &nbsp; ☐☐☐☒☐☐<br/>
    /// ⇰☒☒☒☒☒ &nbsp; ⇰☐☒☐☒☐<br/>
    /// ☐☐☒☒☒☐ &nbsp; ☐☐☐☒☐☐<br/>
    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☐☐☐<br/>
    Zone {
        min_distance: u32,
        max_distance: u32,
    },

    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☒☐☐ <br/>
    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☒☐☐ <br/>
    /// ⇰☒☒☒☒☒ &nbsp; ⇰☒☒☐☒☒ <br/>
    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☒☐☐ <br/>
    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☒☐☐ <br/>
    Cross {
        min_distance: u32,
        max_distance: u32,
    },
}

impl Default for ActionAoe {
    fn default() -> Self {
        Self::Cell
    }
}

impl ActionAoe {
    pub fn compute_hit_positions(
        &self,
        target_position: &MapPosition,
        map_query: &mut MapQuery,
    ) -> Vec<MapPosition> {
        let map = map_query.map.single();
        match *self {
            ActionAoe::Cell => vec![target_position.clone()],
            ActionAoe::Zone {
                min_distance,
                max_distance,
            } => target_position
                .get_surrounding_positions(min_distance, max_distance, map.width, map.height)
                .iter()
                .copied()
                .filter(|position| !map_query.is_obstacle(0u32, position))
                .collect::<Vec<_>>(),
            ActionAoe::Cross {
                min_distance,
                max_distance,
            } => target_position
                .get_surrounding_positions(min_distance, max_distance, map.width, map.height)
                .iter()
                .copied()
                .filter(|position| !map_query.is_obstacle(0u32, position))
                .filter(|position| {
                    position.x == target_position.x || position.y == target_position.y
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn show_description_ui(self, ui: &mut egui::Ui) {
        match self {
            ActionAoe::Cell => {}
            ActionAoe::Zone {
                min_distance,
                max_distance,
            } => {
                ui.label(
                    egui::RichText::new(format!("target {}-{}", min_distance, max_distance))
                        .strong()
                        .color(color::ACTION_NEUTRAL),
                );
            }
            ActionAoe::Cross {
                min_distance,
                max_distance,
            } => {
                ui.label(
                    egui::RichText::new(format!(
                        "target {}-{}, in line only",
                        min_distance, max_distance
                    ))
                    .strong()
                    .color(color::ACTION_NEUTRAL),
                );
            }
        }
    }
}

/// The action range represents the targetable cells from the attacker position
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum ActionRange {
    Around {
        min_distance: u32,
        max_distance: u32,
        check_los: bool,
    },
    Line {
        min_distance: u32,
        max_distance: u32,
        check_los: bool,
    },
    Diagonal {
        min_distance: u32,
        max_distance: u32,
        check_los: bool,
    },
}

impl Default for ActionRange {
    fn default() -> Self {
        Self::Around {
            min_distance: 0,
            max_distance: 0,
            check_los: true,
        }
    }
}

impl ActionRange {
    pub fn can_reach(&self, from: &MapPosition, to: &MapPosition) -> bool {
        let distance = from.distance_to(to);
        match *self {
            ActionRange::Around {
                min_distance,
                max_distance,
                ..
            } => distance >= min_distance && distance <= max_distance,
            ActionRange::Line {
                min_distance,
                max_distance,
                ..
            } => {
                distance >= min_distance
                    && distance <= max_distance
                    && (from.x == to.x || from.y == to.y)
            }
            ActionRange::Diagonal {
                min_distance,
                max_distance,
                ..
            } => {
                distance >= min_distance
                    && distance <= max_distance
                    && (from.x + to.y == from.y + to.x || from.x + from.y == to.x + to.y)
            }
        }
    }
}

impl ActionRange {
    pub fn show_description_ui(self, ui: &mut egui::Ui) {
        match self {
            ActionRange::Around {
                min_distance,
                max_distance,
                check_los,
            } => {
                let mut text = format!("range {}-{}", min_distance, max_distance);
                if !check_los {
                    text.push_str(", no line of sight");
                }

                ui.label(
                    egui::RichText::new(text)
                        .strong()
                        .color(color::ACTION_NEUTRAL),
                );
            }
            ActionRange::Line {
                min_distance,
                max_distance,
                check_los,
            } => {
                let mut text = format!("range {}-{}, in line only", min_distance, max_distance);
                if !check_los {
                    text.push_str(", no line of sight");
                }

                ui.label(
                    egui::RichText::new(text)
                        .strong()
                        .color(color::ACTION_NEUTRAL),
                );
            }
            ActionRange::Diagonal {
                min_distance,
                max_distance,
                check_los,
            } => {
                let mut text = format!("range {}-{}, in diagonal only", min_distance, max_distance);
                if !check_los {
                    text.push_str(", no line of sight");
                }

                ui.label(
                    egui::RichText::new(text)
                        .strong()
                        .color(color::ACTION_NEUTRAL),
                );
            }
        }
    }
}

/// An effect is an outcome of an action execution
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum ActionEffect {
    Nothing,
    Damage {
        amount: u32,
        erode: f32,
        crit_mult: f32,
        crit_chance: f32,
    },
    DamageOverTime {
        amount: u32,
        erode: f32,
        duration: u32,
    },
    Heal {
        amount: u32,
    },
    Shield {
        amount: u32,
    },
    RemoveActionPoints {
        amount: u32,
    },
    StealActionPoints {
        amount: u32,
    },
    RemoveMovementPoints {
        amount: u32,
    },
    StealMovementPoints {
        amount: u32,
    },
    Push {
        distance: u32,
    },
    WalkTo {
        to: MapPosition,
    },
    TeleportSelf,
    TeleportSwitch,
}

impl Default for ActionEffect {
    fn default() -> Self {
        Self::Nothing
    }
}

impl ActionEffect {
    pub fn show_description_ui(self, ui: &mut egui::Ui) {
        match self {
            ActionEffect::Nothing => (),
            ActionEffect::WalkTo { to } => {}
            ActionEffect::Damage { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("-{} health", amount))
                        .strong()
                        .color(color::ACTION_BAD),
                );
            }
            ActionEffect::DamageOverTime {
                amount, duration, ..
            } => {
                ui.label(
                    egui::RichText::new(format!("-{} health, for {} turns", amount, duration))
                        .strong()
                        .color(color::ACTION_BAD),
                );
            }
            ActionEffect::Heal { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("+{} health", amount))
                        .strong()
                        .color(color::ACTION_GOOD),
                );
            }
            ActionEffect::Shield { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("+{} shield", amount))
                        .strong()
                        .color(color::ACTION_GOOD),
                );
            }
            ActionEffect::RemoveActionPoints { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("-{} action points", amount))
                        .strong()
                        .color(color::ACTION_BAD),
                );
            }
            ActionEffect::StealActionPoints { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("steals {} action points", amount))
                        .strong()
                        .color(color::ACTION_BAD),
                );
            }
            ActionEffect::RemoveMovementPoints { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("-{} movement points", amount))
                        .strong()
                        .color(color::ACTION_BAD),
                );
            }
            ActionEffect::StealMovementPoints { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("steals {} movement points", amount))
                        .strong()
                        .color(color::ACTION_BAD),
                );
            }
            ActionEffect::TeleportSwitch => {
                ui.label(
                    egui::RichText::new(format!("switches places with the target"))
                        .strong()
                        .color(color::ACTION_NEUTRAL),
                );
            }
            ActionEffect::TeleportSelf => {
                ui.label(
                    egui::RichText::new(format!("teleports yoursef to the target"))
                        .strong()
                        .color(color::ACTION_NEUTRAL),
                );
            }
            ActionEffect::Push { distance } => {
                ui.label(
                    egui::RichText::new(format!("pushes {} tiles away", distance))
                        .strong()
                        .color(color::ACTION_NEUTRAL),
                );
            }
        }
    }
}
