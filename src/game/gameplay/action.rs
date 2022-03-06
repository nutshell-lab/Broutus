/**
 * Implementation proposal.
 * To be disscussed, alternatives approaches may be worth considering.
 *
 * NewType pattern is heavily used.
 * It makes the file more verbose, but the explicitness of the type name is worth it ihmo.
 * https://doc.rust-lang.org/rust-by-example/generics/new_types.html
 *
 * Some subjects are not yet solved:
 *  - icons loading (using https://github.com/NiklasEi/bevy_asset_loader ? custom AssetLoader implementation ? a mix of both ?)
 *  - animation loading (same as icons)
 *  - prefab loading (from a .ron description file)
 *
 * Maybe we could dynamically load icons and animations and save them in a hashmap with the name as a key.
 * Is this even possible using bevy_asset_loader ?
 *  - .ron files supports hashmaps.
 *  - bevy_asset_loader supports dynamic assets, but with fixed keys it seems (maybe not a problem).
 */
use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Warrior;

#[derive(Default, Bundle)]
pub struct TestWarriorBundle {
    // Tags
    _w: Warrior,

    // Meta
    name: Name,
    team: super::Team,

    // Gameplay
    position: super::MapPosition,
    health: Attribute<Health>,
    shield: Attribute<Shield>,
    action_points: Attribute<ActionPoints>,
    movement_points: Attribute<MovementPoints>,
    actions: Actions,

    // TODO add animation collection ? How to load it ?
    // Redering
    #[bundle]
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer);

/// An attribute is a utility wrapper type for any warrior attribute (health, action points, armor, etc.)
#[derive(Debug, Copy, Clone, Default, Component)]
pub struct Attribute<T: AttributeValue + Copy + Clone + Default> {
    value: T,
    min: T,
    max: T,
}

impl<T: AttributeValue + Copy + Clone + Default> Attribute<T> {
    pub fn value(self) -> u32 {
        self.value.value()
    }

    pub fn min(self) -> u32 {
        self.min.value()
    }

    pub fn max(self) -> u32 {
        self.max.value()
    }

    pub fn drop(&mut self, amount: u32) -> u32 {
        let v = self
            .value
            .value()
            .checked_sub(amount.max(if self.max() < self.value() {
                self.value() - self.max()
            } else {
                0
            })) // Ensure the drop will set the value below the max
            .filter(|v| v >= &self.min())
            .unwrap_or(self.min());

        self.value.set_value(v);
        v
    }

    pub fn drop_min(&mut self) -> u32 {
        self.drop(self.min())
    }

    pub fn rise(&mut self, amount: u32) -> u32 {
        let v = self
            .value
            .value()
            .checked_add(amount.max(if self.min() > self.value() {
                self.min() - self.value()
            } else {
                0
            })) // Ensure the rise will set the value above the min
            .filter(|v| v <= &self.max())
            .unwrap_or(self.max());

        self.value.set_value(v);
        v
    }

    pub fn rise_max(&mut self) -> u32 {
        self.rise(self.max())
    }
}

impl Attribute<Health> {
    pub fn erode(&mut self, amount: u32, erode: f32) {
        let erosion = (amount as f32 - erode).round() as u32;
        let new_max = self
            .max
            .value()
            .checked_sub(erosion)
            .unwrap_or(self.min.value());
        self.max.set_value(new_max);
    }
}

/// A way to interact with attributes NewTypes (maybe their is a better way ?)
pub trait AttributeValue {
    fn value(&self) -> u32;
    fn set_value(&mut self, value: u32);
}

/// NewType representing a Warrior's action points quantity
#[derive(Debug, Copy, Clone, Default)]
pub struct ActionPoints(u32);

impl AttributeValue for ActionPoints {
    fn value(&self) -> u32 {
        self.0
    }
    fn set_value(&mut self, value: u32) {
        self.0 = value;
    }
}

/// NewType representing a Warrior's movement points quantity
#[derive(Debug, Copy, Clone, Default)]
pub struct MovementPoints(u32);

impl AttributeValue for MovementPoints {
    fn value(&self) -> u32 {
        self.0
    }
    fn set_value(&mut self, value: u32) {
        self.0 = value;
    }
}

/// NewType representing a Warrior's health quantity
#[derive(Debug, Copy, Clone, Default)]
pub struct Health(u32);

impl AttributeValue for Health {
    fn value(&self) -> u32 {
        self.0
    }
    fn set_value(&mut self, value: u32) {
        self.0 = value;
    }
}

/// NewType representing a Warrior's shield quantity
#[derive(Debug, Copy, Clone, Default)]
pub struct Shield(u32);

impl AttributeValue for Shield {
    fn value(&self) -> u32 {
        self.0
    }
    fn set_value(&mut self, value: u32) {
        self.0 = value;
    }
}

/// NewType representing a Warrior's action collection
#[derive(Debug, Component)]
pub struct Actions(Vec<Action>);

impl Default for Actions {
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// A Warrior action is usable in Arena mode, consuming ActionPoints
#[derive(Debug)]
pub struct Action {
    name: String,
    icon_name: String, // TODO how to load these icons and register them into egui ?
    animation_key: String, // TODO how to load these animations and use them on the Warrior's TextureAtlasSprite ?
    cost: ActionPoints,
    aoe: ActionAoe,
    range: ActionRange,
    effects: ActionEffects,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            name: "Generic Action".to_string(),
            icon_name: "generic_action".to_string(),
            animation_key: "generic_animation".to_string(),
            cost: ActionPoints(1),
            aoe: ActionAoe::default(),
            range: ActionRange::default(),
            effects: ActionEffects::default(),
        }
    }
}

impl Action {
    /// Execute all action effects one by one
    pub fn execute(
        &self,
        from_position: &super::MapPosition,
        to_position: &super::MapPosition,
        map_query: &mut super::MapQuery,
        warrior_query: &mut Query<
            (
                &Name,
                &mut super::MapPosition,
                &mut Attribute<Health>,
                &mut Attribute<Shield>,
                &mut Attribute<ActionPoints>,
                &mut Attribute<MovementPoints>,
            ),
            With<Warrior>,
        >,
    ) {
        if let ActionAoe::Cell = self.aoe {
            for (_, mut position, mut health, ..) in warrior_query.iter_mut() {
                if position.ne(to_position) {
                    continue;
                }

                for effect in self.effects.0.iter() {
                    // Implementation example
                    if let ActionEffect::DamageHealthOrShield {
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
#[derive(Debug)]
pub enum ActionAoe {
    /// ☐☐☐☐☐ <br/>
    /// ☐☐☐☐☐ <br/>
    /// ☐☐☑☐☐ <br/>
    /// ☐☐☐☐☐ <br/>
    /// ☐☐☐☐☐ <br/>
    Cell,

    /// ☐☐☑☐☐ &nbsp; ☐☐☑☐☐<br/>
    /// ☐☑☑☑☐ &nbsp; ☐☑☑☑☐<br/>
    /// ☑☑☑☑☑ &nbsp; ☑☑☐☑☑<br/>
    /// ☐☑☑☑☐ &nbsp; ☐☑☑☑☐<br/>
    /// ☐☐☑☐☐ &nbsp; ☐☐☑☐☐<br/>
    Zone {
        min_distance: u32,
        max_distance: u32,
    },

    /// ☐☐☐☐☐ &nbsp; ☐☐☐☐☐ &nbsp; ☐☐☐☐☐ &nbsp; ☐☐☐☐☐  <br/>
    /// ☐☐☐☐☐ &nbsp; ☐☐☐☐☐ &nbsp; ☐☐☐☐☐ &nbsp; ☐☐☐☐☐  <br/>
    /// ☐☐☑☑☑ &nbsp; ☑☑☑☐☐ &nbsp; ☑☑☐☑☑ &nbsp; ☐☐☐☑☑  <br/>
    /// ☐☐☐☐☐ &nbsp; ☐☐☐☐☐ &nbsp; ☐☐☐☐☐ &nbsp; ☐☐☐☐☐  <br/>
    /// ☐☐☐☐☐ &nbsp; ☐☐☐☐☐ &nbsp; ☐☐☐☐☐ &nbsp; ☐☐☐☐☐  <br/>
    Line {
        distance: u32,
        forward_length: u32,
        away_length: u32,
    },

    /// ☐☐☑☐☐ &nbsp; ☐☐☑☐☐ <br/>
    /// ☐☐☑☐☐ &nbsp; ☐☐☑☐☐ <br/>
    /// ☑☑☑☑☑ &nbsp; ☑☑☐☑☑ <br/>
    /// ☐☐☑☐☐ &nbsp; ☐☐☑☐☐ <br/>
    /// ☐☐☑☐☐ &nbsp; ☐☐☑☐☐ <br/>
    Cross { min_length: u32, max_length: u32 },
}

impl Default for ActionAoe {
    fn default() -> Self {
        Self::Cell
    }
}

/// The action range represents the targetable cells from the attacker position
#[derive(Debug)]
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

/// NewType representing a Warrior action's effects collection
#[derive(Debug)]
pub struct ActionEffects(Vec<ActionEffect>);

impl Default for ActionEffects {
    fn default() -> Self {
        Self(Vec::new())
    }
}

/// An effect is an outcome of an action execution
#[derive(Debug)]
pub enum ActionEffect {
    Nothing,
    DamageHealthOrShield {
        amount: u32,
        erode: f32,
        crit_mult: f32,
        crit_chance: f32,
    },
    DamageHealthOrShieldOverTime {
        amount: u32,
        erode: f32,
        duration: u32,
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
