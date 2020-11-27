use bevy::{prelude::*, app::startup_stage};

pub mod map;
pub mod components;
pub mod orders;
pub mod physics;
mod systems;

use self::{map::create_map, orders::orders_sys::*};
use systems::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(physics::PhysicsPlugin)
        .add_startup_system_to_stage(startup_stage::PRE_STARTUP, create_map.system())
        .add_startup_system_to_stage(startup_stage::STARTUP, create_units.system())
        .add_system(order_system.system())
        .add_system_to_stage(stage::POST_UPDATE, ai_system.system())
        .add_system(attack_melee_system.system())
        .add_system(health_system.system())

        //.add_system(order_system_debug_change.system())
            ;
    }
}
