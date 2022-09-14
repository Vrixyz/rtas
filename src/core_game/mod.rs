use bevy::prelude::*;
use bevy_inspector_egui::RegisterInspectable;
use pathfinding::PathfindingPlugin;

pub mod components;
pub mod map;
pub mod orders;
pub mod pathfinding;
pub mod physics;
mod systems;

use self::{map::create_map, orders::orders_sys::*};
use systems::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        use crate::core_game::orders::orders_comp::*;
        app.register_inspectable::<Speed>();
        app.register_inspectable::<RotateBeforeMove>();
        app.register_inspectable::<Mover>();

        app.add_plugin(physics::PhysicsPlugin)
        .add_plugin(PathfindingPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, create_map)
        .add_startup_system_to_stage(StartupStage::Startup, create_units)
        .add_system(order_system)
        .add_system_to_stage(CoreStage::PostUpdate, ai_system)
        .add_system(attack_melee_system)
        .add_system(health_system)

        //.add_system(order_system_debug_change)
            ;
    }
}
