use crate::game::map::{MapPosition, Tile};

use super::*;
use bevy::prelude::*;
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
        warrior_query: &mut Query<
            (
                &Name,
                &mut super::super::MapPosition,
                &Actions,
                &mut Attribute<Health>,
                &mut Attribute<Shield>,
                &mut Attribute<ActionPoints>,
                &mut Attribute<MovementPoints>,
            ),
            (With<Warrior>, Without<Tile>),
        >,
    ) {
        let hit_positions = match self.aoe {
            ActionAoe::Cell => vec![to_position],
            _ => vec![],
        };

        for hit_position in hit_positions {
            // Process warriors on the given position
            for (_, mut position, _, mut health, ..) in warrior_query.iter_mut() {
                if position.ne(hit_position) {
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
                        let is_crit = *crit_chance >= 1.0;
                        let mutl = if is_crit { *crit_mult } else { 1.0 };
                        let final_amount = (*amount as f32 * mutl).round() as u32;

                        health.erode(final_amount, *erode);
                        health.drop(final_amount);
                    }

                    // Implementation example
                    if let ActionEffect::PushLinear { distance } = effect {
                        if let Some(direction) = from_position.direction_to(&to_position) {
                            let path = position.unchecked_path_torward(direction, *distance);
                            let mut path_iter = path.iter();
                            while let Some(next_position) = path_iter.next() {
                                // TODO retreive these values nicely
                                if map_query.is_obstacle(0u32, next_position, 20, 12) {
                                    let remaining_length = path_iter.count() as u32;
                                    let damages = 20 * remaining_length;
                                    health.drop(damages);
                                    break;
                                }
                                position.x = next_position.x;
                                position.y = next_position.y;
                            }
                        }
                    }
                }
            }
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
    PushDiagonal {
        distance: u32,
    },
}

impl Default for ActionEffect {
    fn default() -> Self {
        Self::Nothing
    }
}
