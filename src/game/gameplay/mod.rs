use bevy::prelude::*;

#[derive(Default)]
pub struct CurrentTurn(u16);

#[derive(Default)]
pub struct CurrentTurnOrder(Vec<Entity>);

#[derive(Default)]
pub struct CurrentTurnOrderIndex(usize);


pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTurn>()
            .init_resource::<CurrentTurnOrder>()
            .init_resource::<CurrentTurnOrderIndex>();
    }
}

// fn system_using_current_character(
//     turn_order: Res<CurrentTurnOrder>,
//     turn_order_index: Res<CurrentTurnOrderIndex>,
//     character_query: Query<&crate::game::character::Character>
// ) {
//     let entity = turn_order.0[turn_order_index.0];
//     let character = character_query.get(entity).unwrap();
// }
