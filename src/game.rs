use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app;
        // .init_resource::<MyOtherResource>()
        // .add_event::<MyEvent>()
        // .add_startup_system(plugin_init)
        // .add_system(my_system);
    }
}
