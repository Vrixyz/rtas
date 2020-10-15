use bevy::prelude::*;
use super::super::core_game::components::*;

#[derive(PartialEq, Clone, Debug)]
pub struct SelectionPending {
    pub begin_pos: Position,
    pub end_pos: Position,
}
#[derive(PartialEq, Clone)]
pub struct SelectionValidated {
    pub begin_pos: Position,
    pub end_pos: Position,
}
#[derive(PartialEq, Clone)]
pub enum Selection {
    None,
    OnGoing(SelectionPending),
}
pub struct MyCursorState {
    pub cursor: EventReader<CursorMoved>,
    // need to identify the main camera
    pub camera_e: Entity,
    pub world_position: Position,
}