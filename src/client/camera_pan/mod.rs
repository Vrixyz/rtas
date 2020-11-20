use bevy::prelude::*;

use self::camera_pan_comp::CameraPan;

pub mod camera_pan_comp;


pub struct CameraPanPlugin;

impl Plugin for CameraPanPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(CameraPan {
            camera: None,
            ..Default::default()
        });
        app
            .add_system(systems::camera_pan.system())
        ;
    }
}

mod systems {
    use bevy::{ecs::Query, ecs::{Res, ResMut}, input::Input, ecs::Mutated, math::Vec3, prelude::MouseButton, prelude::Transform};

    use crate::client::components::{MainCamera, MyCursorState};

    use super::camera_pan_comp::CameraPan;

    pub fn camera_pan(
        mut camera_pan: ResMut<CameraPan>,
        main_camera: Res<MainCamera>,
        mouse_button: Res<MyCursorState>,
        mouse_event: Res<Input<MouseButton>>,
        mut query: Query<&mut Transform>,
    ) {
        if mouse_event.pressed(MouseButton::Middle) {
            if camera_pan.last_click_position.is_some() {
                let mut camera = query.get_component_mut::<Transform>(main_camera.camera_e).unwrap();
                let offset = mouse_button.ui_position - camera_pan.last_click_position.unwrap();
                camera.translation -= Vec3::new(offset.x(), offset.y(), 0.0);    
            }
            camera_pan.last_click_position = Some(mouse_button.ui_position);
        }
        else if camera_pan.last_click_position.is_some() {
            camera_pan.last_click_position = None;
        }
    }
}