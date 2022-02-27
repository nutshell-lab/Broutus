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
pub use warrior::animate_sprite;
pub use warrior::snap_to_map;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Attribute>()
            .register_type::<Health>()
            .register_type::<ActionPoints>()
            .register_type::<MovementPoints>()
            .add_event::<TurnStart>()
            .add_event::<TurnEnd>()
            .add_startup_system(setup)
            .add_system(show_turn_ui)
            .add_system(animate_sprite)
            .add_system(snap_to_map);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load characters animation spritesheet
    let texture_handle = asset_server.load("characters/knight_idle.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 15, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn characters
    let knight_blue = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Blue".to_string(),
            MapPosition::new(17, 5),
            -1.0,
            &texture_atlas_handle,
        ))
        .insert(TeamA)
        .id();

    let knight_red = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Red".to_string(),
            MapPosition::new(2, 5),
            1.0,
            &texture_atlas_handle,
        ))
        .insert(TeamB)
        .id();

    let knight_purple = commands
        .spawn_bundle(WarriorBundle::new(
            "Knight Purple".to_string(),
            MapPosition::new(2, 7),
            1.0,
            &texture_atlas_handle,
        ))
        .insert(TeamB)
        .id();

    // Insert turn system resource
    commands.insert_resource(Turn {
        order: vec![knight_blue, knight_red, knight_purple],
        ..Default::default()
    })
}
