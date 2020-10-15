use bevy::prelude::*;

use super::super::core_game::components::*;
use super::components::*;

pub fn helper_in_rect(position: &Vec3, corner_1: &Position, corner_2: &Position) -> bool {
    let min_x = f32::min(corner_1.x, corner_2.x);
    let min_y = f32::min(corner_1.y, corner_2.y);
    let max_x = f32::max(corner_1.x, corner_2.x);
    let max_y = f32::max(corner_1.y, corner_2.y);

    if position[0] >= min_x && position[0] <= max_x && position[1] >= min_y && position[1] <= max_y {
        return true;
    }
    return false;
}

pub fn create_camera(mut commands: Commands) {
    let camera = Camera2dComponents::default();
    let e = commands.spawn(camera).current_entity().unwrap();
    commands.insert_resource(MyCursorState {
        cursor: Default::default(),
        camera_e: e,
        world_position: Position{x:0f32, y: 0f32},
    });
}

/// Adapted from https://github.com/jamadazi/bevy-cookbook/blob/master/bevy-cookbook.md#convert-screen-coordinates-to-world-coordinates
pub fn mouse_world_position_system (
    mut state: ResMut<MyCursorState>,
    ev_cursor: Res<Events<CursorMoved>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera components
    q_camera: Query<&Transform>) {
    let camera_transform = q_camera.get::<Transform>(state.camera_e).unwrap();

    if let Some(ev) = state.cursor.latest(&ev_cursor) {
        let wnd = wnds.get(ev.id).unwrap();
        let size = Vec2::new(wnd.width as f32, wnd.height as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = ev.position - size / 2.0;

        // apply the camera transform
        let pos_wld = *camera_transform.value() * p.extend(0.0).extend(1.0);
        let position = Position{x:pos_wld.x(),y:pos_wld.y()};
        state.world_position = position;
    }
}


pub fn selection_system(
    cursor_state: Res<MyCursorState>,
    // TODO: selection needs to be mutable only if we're modifying the selection.
    mut selection: ResMut<Selection>,
    mouse_button: Res<Input<MouseButton>>,
    mut query: Query<(&mut Selectable, &Transform)>) {
    if mouse_button.pressed(MouseButton::Left) {
        if *selection == Selection::None {
            let position = cursor_state.world_position.clone();
            *selection = Selection::OnGoing(SelectionPending{begin_pos: position.clone(), end_pos: position});
        }
        else if let Selection::OnGoing(on_going) = &mut *selection {
            on_going.end_pos = cursor_state.world_position.clone();
        }
    }
    else {
        if let Selection::OnGoing(on_going) = &mut *selection {
            let mouse_pos_end = &cursor_state.world_position;
            for (mut s, _) in &mut query.iter() {
                s.is_selected = false;
            }
            for (mut a, b) in &mut query.iter() {
                if helper_in_rect(&b.translation(), &on_going.begin_pos, &mouse_pos_end) {
                    a.is_selected = true;
                }
            }
            *selection = Selection::None;
        }
    }
}

pub fn selection_visual_system(query_selectables: Query<Mutated<Selectable>>,
    mut query_visual: Query<(&SelectionVisual, &mut Transform, &Parent)>) {

        for (_, mut transform, parent) in &mut query_visual.iter() {
            if let Ok(selectable) = query_selectables.get::<Selectable>(parent.0) {
                if selectable.is_selected {
                    transform.set_scale(1f32);
                }
                else {
                    // TODO: know how to hide properly something (scale 0 breaks everything (I guess it's removed or break the transform..?))
                    transform.set_scale(0.1f32);
                }
            }
        }
}

pub fn move_order_system(
    cursor_state: Res<MyCursorState>,
    mouse_button: Res<Input<MouseButton>>,
    mut query: Query<(&mut Mover, &Selectable)>) {
        if mouse_button.just_pressed(MouseButton::Right) {
            for (mut mover, selectable) in &mut query.iter() {
                if selectable.is_selected {
                    mover.target_position = cursor_state.world_position.clone();
                }
            }
        }

}