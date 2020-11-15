use bevy::prelude::*;

pub mod components;
pub mod orders;
pub mod physics;
mod systems;

use self::orders::orders_sys::*;
use systems::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(physics::PhysicsPlugin)
        .add_startup_system(create_units.system())
        .add_system(order_system.system())
        .add_system_to_stage(stage::POST_UPDATE, ai_system.system())
        .add_system(attack_melee_system.system())
        .add_system(health_system.system())

        //.add_system(order_system_debug_change.system())
            ;
    }
}
