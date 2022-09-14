use std::collections::HashMap;

use super::super::core_game::components::*;
use bevy::{input::mouse::MouseWheel, prelude::*};

#[derive(Component)]
pub struct Selectable {
    pub is_selected: bool,
    pub half_size: f32,
}

#[derive(Component)]
pub struct SelectionVisual;

#[derive(Component, PartialEq, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl From<&Vec3> for Position {
    fn from(from: &Vec3) -> Self {
        Self {
            x: from.x,
            y: from.y,
        }
    }
}

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
    Hover(Option<Entity>),
    OnGoing(SelectionPending),
}

#[derive(Component)]
pub struct MainCamera {
    pub camera_e: Entity,
}

#[derive(Component)]
pub struct RenderSpriteVisual {
    pub color: Color,
    pub image: Handle<Image>,
}

#[derive(Component)]
pub struct NoRotation;

pub struct MyCursorState {
    // need to identify the main camera
    pub camera_e: Entity,
    pub world_position: Position,
    pub ui_position: Vec2,
}
pub struct RenderResource {
    pub render_sprite_visuals: HashMap<RenderSprite, RenderSpriteVisual>,
    pub color_selection: Color,
    pub color_walls: Color,
    pub team_colors: Vec<Color>,
}
