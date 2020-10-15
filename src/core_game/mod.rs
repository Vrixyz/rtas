use bevy::{prelude::*};

pub mod components;
mod systems;

use systems::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_units.system())
        .add_system(mover_update.system())
            ;
    }
}