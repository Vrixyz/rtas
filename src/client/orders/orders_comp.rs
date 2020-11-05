use bevy::prelude::*;

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
}

