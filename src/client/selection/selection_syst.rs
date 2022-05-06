use bevy::prelude::*;

use crate::client::components::*;

use super::{
    helpers::helper_in_rect, helpers::helper_rect_in_rect, selection_comp::SelectionRectVisual,
};

pub fn selection_system(
    cursor_state: Res<MyCursorState>,
    mut selection: ResMut<Selection>,
    mouse_button: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &mut Selectable, &Transform)>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        if matches!(*selection, Selection::Hover(_)) {
            let position = cursor_state.world_position.clone();
            *selection = Selection::OnGoing(SelectionPending {
                begin_pos: position.clone(),
                begin_pos_ui: cursor_state.ui_position,
                end_pos: position,
                end_pos_ui: cursor_state.ui_position,
            });
        } else if let Selection::OnGoing(on_going) = &mut *selection {
            on_going.end_pos = cursor_state.world_position.clone();
            on_going.end_pos_ui = cursor_state.ui_position;
        }
        return;
    }
    if let Selection::OnGoing(on_going) = &mut *selection {
        let mouse_pos_end = &cursor_state.world_position;
        for (_, mut s, _) in query.iter_mut() {
            s.is_selected = false;
        }
        for (_, mut a, b) in query.iter_mut() {
            let selectable_position = b.translation;
            let half_size = a.half_size;
            let c1 = Position {
                x: selectable_position.x - half_size,
                y: selectable_position.y - half_size,
            };
            let c2 = Position {
                x: selectable_position.x + half_size,
                y: selectable_position.y + half_size,
            };
            if helper_rect_in_rect((&c1, &c2), (&on_going.begin_pos, &mouse_pos_end)) {
                a.is_selected = true;
            }
        }
    }
    for (e, a, b) in query.iter_mut() {
        let selectable_position = b.translation;
        let half_size = a.half_size;
        let c1 = Position {
            x: selectable_position.x - half_size,
            y: selectable_position.y - half_size,
        };
        let c2 = Position {
            x: selectable_position.x + half_size,
            y: selectable_position.y + half_size,
        };
        if helper_in_rect(&cursor_state.world_position, &c1, &c2) {
            *selection = Selection::Hover(Some(e));
            return;
        }
    }
    *selection = Selection::Hover(None);
}

pub fn selection_ui_visual(
    rect: Res<SelectionRectVisual>,
    selection: Res<Selection>,
    mut q: Query<(&mut Style, &mut Visibility)>,
) {
    if let Selection::OnGoing(selection) = &*selection {
        if let Ok(mut visual) = q.get_component_mut::<Style>(rect.visual) {
            let min_x = f32::min(selection.begin_pos_ui.x, selection.end_pos_ui.x);
            let min_y = f32::min(selection.begin_pos_ui.y, selection.end_pos_ui.y);
            let max_x = f32::max(selection.begin_pos_ui.x, selection.end_pos_ui.x);
            let max_y = f32::max(selection.begin_pos_ui.y, selection.end_pos_ui.y);
            visual.position = Rect {
                left: Val::Px(min_x),
                bottom: Val::Px(min_y),
                ..Default::default()
            };
            visual.size = Size::new(Val::Px(max_x - min_x), Val::Px(max_y - min_y));
        }
        if let Ok(mut draw) = q.get_component_mut::<Visibility>(rect.visual) {
            draw.is_visible = true;
        }
    } else {
        if let Ok(mut draw) = q.get_component_mut::<Visibility>(rect.visual) {
            draw.is_visible = false;
        }
    }
}

pub fn selection_visual_system(
    query_selectables: Query<Changed<Selectable>>,
    mut query_visual: Query<(&mut Transform, &Parent, &mut Visibility), With<SelectionVisual>>,
) {
    for (_, parent, mut visibility) in query_visual.iter_mut() {
        if let Ok(selectable) = query_selectables.get_component::<Selectable>(parent.0) {
            if selectable.is_selected {
                visibility.is_visible = true;
            } else {
                // TODO: know how to hide properly something (scale 0 breaks everything (I guess it's removed or break the transform..?))
                visibility.is_visible = false;
            }
        }
    }
}
