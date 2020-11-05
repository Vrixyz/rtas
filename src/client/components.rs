use std::collections::HashMap;

use bevy::prelude::*;
use super::super::core_game::components::*;

pub struct Selectable {
    pub is_selected: bool,
    pub half_size: f32,
}
pub struct SelectionVisual;

#[derive(PartialEq, Clone, Debug)]
pub struct Position { pub x: f32, pub y: f32 }

#[derive(PartialEq, Clone, Debug)]
pub struct SelectionPending {
    pub begin_pos: Position,
    pub end_pos: Position,
    pub begin_pos_ui: Vec2,
    pub end_pos_ui: Vec2,
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
    pub ui_position: Vec2,
}
pub struct RenderResource {
    pub render_sprite_visuals: HashMap<RenderSprite, RenderSpriteVisual>,
    pub color_selection: Handle<ColorMaterial>,
    pub team_colors: Vec<Handle<ColorMaterial>>,
}
pub struct RenderSpriteVisual {
    pub color: Handle<ColorMaterial>,
    pub material: Handle<ColorMaterial>
}

pub struct HealthVisual {
    pub max_hp_visual: Entity,
    pub current_hp_visual: Entity,
}
