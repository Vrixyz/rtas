use bevy::prelude::*;

use crate::core_game::components::Team;

pub struct TeamResource {
    pub team: Team,
}

#[derive(Component, Clone)]
pub struct DebugOrderMove;

#[derive(Component, Clone)]
pub struct DebugOrderMoveGraphic {
    pub(super) entity_to_debug: Entity,
}

#[derive(Component)]
pub struct OrderVisualResource {
    pub(super) move_material: Handle<ColorMaterial>,
    pub(super) attack_material: Handle<ColorMaterial>,
}
