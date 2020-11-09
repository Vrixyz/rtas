use bevy::{prelude::*, app::startup_stage};


mod components;
mod systems;
mod orders;
mod selection;

use systems::*;
use components::*;
use orders::*;

use crate::core_game::components::Team;

use self::{orders::{orders_comp::TeamResource, orders_sys::*}, selection::selection_syst::*, systems::ability::*};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {

        app.add_resource(TeamResource{
            team: Team {id: 0},
        });

        app
        .add_startup_system(create_camera.system())
        .add_startup_system(create_render_resource.system())
        .add_startup_system_to_stage(startup_stage::POST_STARTUP, create_ui.system())
        .add_startup_system_to_stage(startup_stage::POST_STARTUP, adapt_units_for_client.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        
        .add_system(mouse_world_position_system.system())

        .add_system(selection_system.system())
        .add_system(selection_visual_system.system())
        .add_system(selection_ui_visual.system())


        .add_startup_system(health_visual_startup.system())
        .add_system(health_visual_setup_system.system())
        .add_system(health_visual_system.system())

        
        .add_startup_system(ability_visual_startup.system())
        .add_system(ability_visual_setup.system())
        .add_system(ability_visual.system())
        // TODO: make the input system trigger before update, and the ai system trigger after update
        
        .add_system(move_order_system.system())

        .add_startup_system(order_system_visual_startup.system())
        .add_system(order_system_visual_init.system())
        .add_system(order_system_visual.system())
        ;
    }
}