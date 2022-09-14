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
        mut scroll_evr: EventReader<MouseWheel>,
        mut query: Query<&mut Transform>,
    ) {
        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll_evr.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    println!(
                        "Scroll (line units): vertical: {}, horizontal: {}",
                        ev.y, ev.x
                    );
                    zoom(&mut query, &main_camera, ev);
                }
                MouseScrollUnit::Pixel => {
                    println!(
                        "Scroll (pixel units): vertical: {}, horizontal: {}",
                        ev.y, ev.x
                    );
                    zoom(&mut query, &main_camera, ev);
                }
            }
        }
    }

    fn zoom(query: &mut Query<&mut Transform>, main_camera: &Res<MainCamera>, ev: &MouseWheel) {
        let mut transform = query
            .get_component_mut::<Transform>(main_camera.camera_e)
            .unwrap();
        let offset = ev.y * -0.1f32;
        let newScale = transform.scale + Vec3::ONE * offset;
        // FIXME: Scale is very ugly: zooming out messes up with cursor drag movement (should take zoom into account), and we can't zoom in.
        transform.scale = newScale.max(Vec3::ONE);
    }
}
