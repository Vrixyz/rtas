use bevy::{math::Vec2, prelude::Entity};

#[derive(Default)]
pub struct CameraPan {
    pub camera: Option<Entity>,
    pub last_click_position: Option<Vec2>,
}
