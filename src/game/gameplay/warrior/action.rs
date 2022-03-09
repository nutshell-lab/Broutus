use crate::game::{
    color,
    map::{MapPosition, Tile},
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
        from_position: &super::super::MapPosition,
        to_position: &super::super::MapPosition,
        map_query: &mut super::super::MapQuery,
        warrior_query: &mut Query<(
            &Warrior,
            &Name,
            &mut super::super::MapPosition,
            &Actions,
            &mut ActiveEffects,
            &mut Attribute<Health>,
            &mut Attribute<Shield>,
            &mut Attribute<ActionPoints>,
            &mut Attribute<MovementPoints>,
        )>,
    ) {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        let hit_positions = match self.aoe {
            ActionAoe::Cell => vec![to_position],
            _ => vec![],
        };

        // TODO BUG the non-duplication of the pairs make the attacker the attacked sometimes, call it left / right instead of attacked / attacker
        // TODO reverse the order, match effects, then iterates on positions, then on warriors, keeping intermediates state if necessary to update attacker at the end
        let mut combinations = warrior_query.iter_combinations_mut();
        while let Some(
            [(
                _,
                attacker_name,
                mut attacker_position,
                mut attacker_actions,
                mut attacker_effects,
                mut attacker_health,
                mut attacker_shield,
                mut attacker_ap,
                mut attacker_mp,
            ), (
                _,
                attacked_name,
                mut attacked_position,
                mut attacked_actions,
                mut attacked_effects,
                mut attacked_health,
                mut attacked_shield,
                mut attacked_ap,
                mut attacked_mp,
            )],
        ) = combinations.fetch_next()
        {
            if from_position.ne(&attacker_position) {
                continue;
            }

            for hit_position in hit_positions.iter() {
                if attacked_position.ne(hit_position) {
                    continue;
                }

                for effect in self.effects.iter() {
                    // Implementation example
                    if let ActionEffect::Damage {
                        amount,
                        erode,
                        crit_chance,
                        crit_mult,
                    } = effect
                    {
                        let is_crit = *crit_chance >= rng.gen();
                        let mutl = if is_crit { *crit_mult } else { 1.0 };
                        let final_amount = (*amount as f32 * mutl).round() as u32;

                        let remaining = attacked_shield.drop(final_amount);
                        println!("Remaining {}", remaining);
                        attacked_health.drop(remaining);
                        attacked_health.erode(remaining, *erode);
                    }

                    if let ActionEffect::DamageOverTime { .. } = effect {
                        attacked_effects.0.push(effect.clone());
                    }

                    if let ActionEffect::Heal { amount } = effect {
                        attacked_health.rise(*amount);
                    }

                    if let ActionEffect::Shield { amount } = effect {
                        attacked_shield.rise(*amount);
                    }

                    if let ActionEffect::RemoveActionPoints { amount } = effect {
                        attacked_ap.drop(*amount);
                    }

                    if let ActionEffect::StealActionPoints { amount } = effect {
                        let remaining = attacked_ap.drop(*amount);
                        attacker_ap.rise(amount - remaining);
                    }

                    if let ActionEffect::RemoveMovementPoints { amount } = effect {
                        attacked_mp.drop(*amount);
                    }

                    if let ActionEffect::StealMovementPoints { amount } = effect {
                        let remaining = attacked_mp.drop(*amount);
                        attacker_mp.rise(amount - remaining);
                    }

                    if let ActionEffect::TeleportSelf = effect {
                        attacker_position.x = hit_position.x;
                        attacker_position.y = hit_position.y;
                    }

                    if let ActionEffect::TeleportSwitch = effect {
                        let MapPosition { x, y } = attacked_position.clone();
                        attacked_position.x = attacker_position.x;
                        attacked_position.y = attacker_position.y;
                        attacker_position.x = x;
                        attacker_position.y = y;
                    }

                    if let ActionEffect::PushLinear { distance } = effect {
                        if let Some(direction) = from_position.direction_to(&hit_position) {
                            let path = hit_position.unchecked_path_torward(direction, *distance);
                            let mut path_iter = path.iter();
                            while let Some(next_position) = path_iter.next() {
                                if map_query.is_obstacle(0u32, next_position) {
                                    let remaining_length = path_iter.count() as u32;
                                    let damages = 20 * remaining_length;
                                    let remaining = attacked_shield.drop(damages);
                                    attacked_health.drop(remaining);
                                    attacked_health.erode(remaining, 0.1); // TODO include erosion to Attribute::<Heatlh>.drop ?
                                    break;
                                }
                                attacked_position.x = next_position.x;
                                attacked_position.y = next_position.y;
                            }
                        }
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

    /// ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐  <br/>
    /// ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐  <br/>
    /// ⇰☐☐☒☒☒ &nbsp; ⇰☒☒☒☐☐ &nbsp; ⇰☒☒☐☒☒ &nbsp; ⇰☐☐☐☒☒  <br/>
    /// ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐  <br/>
    /// ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐ &nbsp; ☐☐☐☐☐☐  <br/>
    Line {
        distance: u32,
        forward_length: u32,
        away_length: u32,
    },

    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☒☐☐ <br/>
    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☒☐☐ <br/>
    /// ⇰☒☒☒☒☒ &nbsp; ⇰☒☒☐☒☒ <br/>
    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☒☐☐ <br/>
    /// ☐☐☐☒☐☐ &nbsp; ☐☐☐☒☐☐ <br/>
    Cross { min_length: u32, max_length: u32 },
}

impl Default for ActionAoe {
    fn default() -> Self {
        Self::Cell
    }
}

/// The action range represents the targetable cells from the attacker position
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum ActionRange {
    Around {
        min_distance: u32,
        max_distance: u32,
    },
    Line {
        min_distance: u32,
        max_distance: u32,
    },
    Diagonal {
        min_distance: u32,
        max_distance: u32,
    },
}

impl Default for ActionRange {
    fn default() -> Self {
        Self::Around {
            min_distance: 0,
            max_distance: 0,
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
            } => distance >= min_distance && distance <= max_distance,
            ActionRange::Line {
                min_distance,
                max_distance,
            } => {
                distance >= min_distance
                    && distance <= max_distance
                    && (from.x == to.x || from.y == to.y)
            }
            ActionRange::Diagonal {
                min_distance,
                max_distance,
            } => distance >= min_distance && distance <= max_distance && (to.x / to.y == 1), // TODO div 0
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
    TeleportSelf,
    TeleportSwitch,
    PushLinear {
        distance: u32,
    },
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
            ActionEffect::Damage { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("removes {} health", amount))
                        .strong()
                        .color(color::ACTION_BAD),
                );
            }
            ActionEffect::DamageOverTime {
                amount, duration, ..
            } => {
                ui.label(
                    egui::RichText::new(format!(
                        "removes {} health, for {} turns",
                        amount, duration
                    ))
                    .strong()
                    .color(color::ACTION_BAD),
                );
            }
            ActionEffect::Heal { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("restores {} health", amount))
                        .strong()
                        .color(color::ACTION_GOOD),
                );
            }
            ActionEffect::Shield { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("gives {} shield", amount))
                        .strong()
                        .color(color::ACTION_GOOD),
                );
            }
            ActionEffect::RemoveActionPoints { amount, .. } => {
                ui.label(
                    egui::RichText::new(format!("removes {} action points", amount))
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
                    egui::RichText::new(format!("removes {} movement points", amount))
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
            ActionEffect::PushLinear { distance } => {
                ui.label(
                    egui::RichText::new(format!(
                        "pushes the target {} tiles away from you",
                        distance
                    ))
                    .strong()
                    .color(color::ACTION_NEUTRAL),
                );
            }
        }
    }
}
