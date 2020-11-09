use bevy::prelude::*;

use crate::core_game::components::Team;

pub struct TeamResource {
    pub team: Team,
}

#[derive(Clone)]
pub struct DebugOrderMove {
    pub(super) graphic: Entity,
}

#[derive(Clone)]
pub struct DebugOrderMoveGraphic {
    pub(super) entity_to_debug: Entity,
}

pub struct OrderVisualResource {
    pub(super) move_material: Handle<ColorMaterial>,
    pub(super) attack_material: Handle<ColorMaterial>,
}

