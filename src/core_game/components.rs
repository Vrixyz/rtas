#[derive(PartialEq, Clone, Debug)]
pub struct Position { pub x: f32, pub y: f32 }
pub struct Mover {
    pub target_position: Position,
    pub speed: f32
}

// FIXME: should be in client but would need to adapt unit creation in 2 steps.
pub struct Selectable {
    pub is_selected: bool,
}
pub struct SelectionVisual;