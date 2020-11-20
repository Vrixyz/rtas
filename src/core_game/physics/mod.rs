use bevy::prelude::*;
use bevy_rapier2d::{physics::RapierPhysicsPlugin, render::RapierRenderPlugin};

use self::physics_syst::*;

mod physics_syst;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(physics_setup.system())
        .add_system_to_stage(stage::PRE_UPDATE, mover_update.system())
        .add_system_to_stage(stage::POST_UPDATE, physics_init.system())

        //.add_plugin(RapierRenderPlugin)
            ;
    }
}