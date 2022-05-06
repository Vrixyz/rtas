use bevy::prelude::*;

use self::camera_pan_comp::CameraPan;

pub mod camera_pan_comp;

pub struct CameraPanPlugin;

impl Plugin for CameraPanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraPan {
            camera: None,
            ..Default::default()
        });
        app.add_system(systems::camera_pan)
            .add_system(systems::camera_zoom);
    }
}

mod systems {
    use bevy::{
        ecs::event::Events, input::mouse::MouseWheel, input::Input, math::Vec3, prelude::*,
        render::camera::CameraProjection, render::camera::OrthographicProjection,
    };

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
                let mut camera = query
                    .get_component_mut::<Transform>(main_camera.camera_e)
                    .unwrap();
                let offset = mouse_button.ui_position - camera_pan.last_click_position.unwrap();
                camera.translation -= Vec3::new(offset.x, offset.y, 0.0);
            }
            camera_pan.last_click_position = Some(mouse_button.ui_position);
        } else if camera_pan.last_click_position.is_some() {
            camera_pan.last_click_position = None;
        }
    }
    pub fn camera_zoom(
        mut commands: Commands,
        mut camera_pan: ResMut<CameraPan>,
        main_camera: Res<MainCamera>,
        mut my_cursor_state: ResMut<MyCursorState>,
        mut query: Query<&mut Transform>,
    ) {
        if let Some(ev) = &my_cursor_state.mouse_wheel.last() {
            // FIXME: Scale is very ugly: zooming out messes up with cursor movement, and we can't zoom in.
            let mut transform = query
                .get_component_mut::<Transform>(main_camera.camera_e)
                .unwrap();
            let offset = ev.y * -0.01f32;
            transform.scale = transform.scale + Vec3::ONE * offset;
        }
    }
}
