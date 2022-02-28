use super::GameState;
use bevy::prelude::*;

mod attribute;
mod turn;
mod warrior;
mod weapon;

use super::map::MapPosition;
use attribute::ActionPoints;
use attribute::Attribute;
use attribute::Health;
use attribute::MovementPoints;
use turn::TeamA;
use turn::TeamB;
use turn::Turn;
use turn::TurnEnd;
use turn::TurnStart;
use warrior::WarriorBundle;

pub use turn::show_turn_ui;
pub use warrior::animate_warrior_sprite;
pub use warrior::update_warrior_world_position;
pub use warrior::WarriorAssets;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attribute>()
            .register_type::<Health>()
            .register_type::<ActionPoints>()
            .register_type::<MovementPoints>()
            .add_event::<TurnStart>()
            .add_event::<TurnEnd>()
            .add_system_set(SystemSet::on_enter(GameState::ARENA).with_system(spawn_characters))
            .add_system_set(
                SystemSet::on_update(GameState::ARENA)
                    .with_system(animate_warrior_sprite)
                    .with_system(update_warrior_world_position)
                    .with_system(show_turn_ui),
            );
    }
}

fn spawn_characters(mut commands: Commands, warrior_assets: Res<WarriorAssets>) {
    // Spawn characters
    let knight_blue = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Blue".to_string(),
            MapPosition::new(17, 5),
            -1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamA)
        .id();

    let knight_red = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Red".to_string(),
            MapPosition::new(2, 5),
            1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamB)
        .id();

    let knight_purple = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Purple".to_string(),
            MapPosition::new(4, 10),
            1.0,
            &warrior_assets.idle,
        ))
        .insert(TeamB)
        .id();

    // Insert turn system resource
    commands.insert_resource(Turn {
        order: vec![knight_blue, knight_red, knight_purple],
        ..Default::default()
    })
}
