use bevy::{prelude::*, app::startup_stage};

pub mod components;
mod systems;

use systems::*;
use components::orders::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(create_units.system())
        .add_system(order_system.system())
        .add_system(mover_update.system())
        .add_system_to_stage(stage::POST_UPDATE, ai_system.system())
        .add_system(attack_melee_system.system())
        .add_system(health_system.system())

        .add_system(order_system_debug_init.system())
        .add_system(order_system_debug.system())
        //.add_system(order_system_debug_change.system())
            ;
    }
}