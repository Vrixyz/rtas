use bevy::{prelude::*};


mod components;
mod systems;

use systems::*;
use components::*;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(create_camera.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(mouse_world_position_system.system())
        .add_system(selection_system.system())
        .add_system(selection_visual_system.system())
        .add_system(move_order_system.system())
        .add_resource(Selection::None)
        ;
    }
}