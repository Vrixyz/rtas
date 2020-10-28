use bevy::prelude::*;

use super::super::core_game::components::*;
use super::super::core_game::components::Orderable::*;
use super::components::*;

pub fn helper_in_rect(position: &Vec3, corner_1: &Position, corner_2: &Position) -> bool {
    let min_x = f32::min(corner_1.x, corner_2.x);
    let max_x = f32::max(corner_1.x, corner_2.x);
    let min_y = f32::min(corner_1.y, corner_2.y);
    let max_y = f32::max(corner_1.y, corner_2.y);

    if position[0] >= min_x && position[0] <= max_x && position[1] >= min_y && position[1] <= max_y {
        return true;
    }
    return false;
}

pub fn helper_rect_in_rect(r1: (&Position, &Position), r2: (&Position, &Position)) -> bool {
    let min_x = f32::min(r1.0.x, r1.1.x);
    let max_x = f32::max(r1.0.x, r1.1.x);
    let min_y = f32::min(r1.0.y, r1.1.y);
    let max_y = f32::max(r1.0.y, r1.1.y);

    let other_min_x = f32::min(r2.0.x, r2.1.x);
    let other_max_x = f32::max(r2.0.x, r2.1.x);
    let other_min_y = f32::min(r2.0.y, r2.1.y);
    let other_max_y = f32::max(r2.0.y, r2.1.y);

    let other_x_touch = min_x <= other_min_x && other_min_x <= max_x;
    let other_y_touch = min_y <= other_min_y && other_min_y <= max_y;
    let x_touch = other_min_x <= min_x && min_x <= other_max_x;
    let y_touch = other_min_y <= min_y && min_y <= other_max_y;
    if other_x_touch && other_y_touch
    {
        return true;
    }
    if x_touch && y_touch
    {
        return true;
    }
    if x_touch && other_y_touch
    {
        return true;
    }
    if other_x_touch && y_touch
    {
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
        ui_position: Vec2::default(),
    });
}
pub fn create_ui(mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut selection_rect_visual: Option<Entity> = None;
    commands.insert_resource(Selection::None);
    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        // root node
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // left vertical fill (border)
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        position: Rect {
                            left: Val::Px(600.0),
                            bottom: Val::Px(180.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba(0.15, 0.65, 0.15, 0.5).into()),
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                }).for_current_entity(|e| {
                    selection_rect_visual = Some(e);
                });
            })
    ;
    if let Some(visual) = selection_rect_visual {
        commands.insert_resource(SelectionRectVisual{visual});
    }
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
        state.ui_position = ev.position;
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
            *selection = Selection::OnGoing(SelectionPending{begin_pos: position.clone(), begin_pos_ui: cursor_state.ui_position, end_pos: position, end_pos_ui: cursor_state.ui_position});
        }
        else if let Selection::OnGoing(on_going) = &mut *selection {
            on_going.end_pos = cursor_state.world_position.clone();
            on_going.end_pos_ui = cursor_state.ui_position;
        }
    }
    else {
        if let Selection::OnGoing(on_going) = &mut *selection {
            let mouse_pos_end = &cursor_state.world_position;
            for (mut s, _) in &mut query.iter() {
                s.is_selected = false;
            }
            for (mut a, b) in &mut query.iter() {
                let selectable_position = b.translation();
                let half_size = a.half_size;
                let c1 = Position {x: selectable_position.x() - half_size, y: selectable_position.y() - half_size};
                let c2 = Position {x: selectable_position.x() + half_size, y: selectable_position.y() + half_size};
                if helper_rect_in_rect((&c1, &c2), (&on_going.begin_pos, &mouse_pos_end)) {
                    a.is_selected = true;
                }
            }
            *selection = Selection::None;
        }
    }
}

pub fn selection_ui_visual(rect: Res<SelectionRectVisual>, selection: Res<Selection>, mut q: Query<(&mut Style, &mut Draw)>) {
    if let Selection::OnGoing(selection) = &*selection {
        if let Ok(mut visual) = q.get_mut::<Style>(rect.visual) {
            let min_x = f32::min(selection.begin_pos_ui.x(), selection.end_pos_ui.x());
            let min_y = f32::min(selection.begin_pos_ui.y(), selection.end_pos_ui.y());
            let max_x = f32::max(selection.begin_pos_ui.x(), selection.end_pos_ui.x());
            let max_y = f32::max(selection.begin_pos_ui.y(), selection.end_pos_ui.y());
            visual.position = Rect {
                left: Val::Px(min_x),
                bottom: Val::Px(min_y),
                ..Default::default()
            };
            visual.size = Size::new(
                Val::Px(
                    max_x - min_x)
                    ,
                Val::Px(
                     max_y - min_y));
        }
        if let Ok(mut draw) = q.get_mut::<Draw>(rect.visual) {
            draw.is_visible = true;
        }

    }
    else {
        if let Ok(mut draw) = q.get_mut::<Draw>(rect.visual) {
            draw.is_visible = false;
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
    mut query: Query<(&mut Orders, &Selectable, Option<&mut AIUnit>)>) {
        if mouse_button.just_pressed(MouseButton::Right) {
            for (mut orders, selectable, ai) in &mut query.iter() {
                // TODO: use an order: ai passive -> move -> previous ai state.
                if selectable.is_selected {
                    dbg!("order!");
                    orders.replace_orders(vec![
                        Order::Ai(AIUnit::Passive),
                        Orders::order_move(Vec3::new(cursor_state.world_position.x, cursor_state.world_position.y, 0f32)),
                        Order::Ai(AIUnit::SeekEnemy(SeekEnemy{range: 200f32})),
                    ]);
                }
            }
        }
}