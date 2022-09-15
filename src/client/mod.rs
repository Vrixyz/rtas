use bevy::prelude::*;

mod camera_pan;
mod components;
mod orders;
mod selection;
mod systems;

use bevy_prototype_lyon::plugin::ShapePlugin;
use systems::*;

use crate::core_game::components::Team;

use self::{
    camera_pan::CameraPanPlugin,
    orders::{orders_comp::TeamResource, orders_sys::*},
    selection::selection_syst::*,
    systems::ability::*,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum CustomStage {
    PreRender,
}

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        use bevy_inspector_egui::WorldInspectorPlugin;
        app.add_plugin(CameraPanPlugin);
        app.add_plugin(ShapePlugin);
        app.add_plugin(WorldInspectorPlugin::new());

        app.insert_resource(TeamResource {
            team: Team { id: 2 },
        });

        app.add_stage_after(
            CoreStage::Update,
            CustomStage::PreRender,
            SystemStage::single_threaded(),
        );

        app.add_startup_system(create_camera)
            .add_startup_system(create_render_resource)
            .add_startup_system_to_stage(StartupStage::PostStartup, create_ui)
            .add_startup_system_to_stage(StartupStage::PostStartup, adapt_units_for_client)
            .add_startup_system_to_stage(StartupStage::PostStartup, adapt_map_for_client)
            .add_system(bevy::window::close_on_esc)
            .add_system(mouse_world_position_system)
            .add_system(selection_system)
            .add_system(selection_visual_system)
            .add_system(selection_ui_visual)
            .add_startup_system(health_visual_startup)
            .add_startup_system(ability_visual_startup)
            .add_system(health_visual_setup_system)
            .add_system(ability_visual_setup)
            .add_system_to_stage(CoreStage::PostUpdate, health_visual_system)
            .add_system_to_stage(CoreStage::PreUpdate, ability_visual)
            // TODO: make the input system trigger before update, and the ai system trigger after update ?
            .add_system(move_order_system)
            .add_startup_system(order_system_visual_startup)
            .add_system(order_system_visual_init)
            .add_system(order_system_visual)
            .add_system_to_stage(CustomStage::PreRender, no_rotation);
    }
}
