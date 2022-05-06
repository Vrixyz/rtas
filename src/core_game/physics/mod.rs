use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use self::physics_syst::*;

mod physics_syst;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
            .add_startup_system(physics_setup)
            .add_system_to_stage(CoreStage::PreUpdate, mover_update)
            .add_system_to_stage(CoreStage::PostUpdate, physics_init);
    }
}
