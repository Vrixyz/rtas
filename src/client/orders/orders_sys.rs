use bevy::{math::Vec3, prelude::*};
use bevy_prototype_lyon::{
    prelude::{primitive, ShapeType, StrokeOptions},
    TessellationMode,
};

use crate::{
    client::components::*,
    core_game::components::Attack,
    core_game::{
        components::{AIUnit, Health, Team},
        orders::orders_comp::*,
    },
};

use super::orders_comp::*;

pub fn move_order_system(
    cursor_state: Res<MyCursorState>,
    mouse_button: Res<Input<MouseButton>>,
    team: Res<TeamResource>,
    selection: Res<Selection>,
    q_attackables: Query<(Entity, &Transform, &Team, &Health, &Selectable)>,
    mut query: Query<(&mut Orders, &Selectable, &Team)>,
) {
    if mouse_button.just_pressed(MouseButton::Right) {
        if let Selection::Hover(Some(selected)) = *selection {
            if let Ok(a_team) = q_attackables.get::<Team>(selected) {
                if a_team.id != team.team.id {
                    for (mut orders, selectable, b_team) in &mut query.iter() {
                        if b_team.id != team.team.id {
                            continue;
                        }
                        if selectable.is_selected {
                            orders.replace_orders(vec![Order::Ai(AIUnit::Attack(Attack {
                                target: selected,
                                chase_on_motion_buffer_exceeded: true,
                            }))]);
                        }
                    }
                    return;
                }
            }
        }

        for (mut orders, selectable, b_team) in &mut query.iter() {
            if b_team.id != team.team.id {
                continue;
            }
            if selectable.is_selected {
                orders.replace_orders(vec![
                    Order::Ai(AIUnit::Passive),
                    Orders::order_move(Vec3::new(
                        cursor_state.world_position.x,
                        cursor_state.world_position.y,
                        0f32,
                    )),
                    Order::Ai(AIUnit::SeekEnemy),
                ]);
            }
        }
    }
}

pub fn order_system_visual_startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(OrderVisualResource {
        move_material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
        attack_material: materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
    });
}

pub fn order_system_visual_init(
    mut commands: Commands,
    mut q_orders: Query<(Entity, &Transform, &Orders, Option<&DebugOrderMove>)>,
) {
    for (entity, transform, _, debug_marker) in &mut q_orders.iter() {
        if debug_marker.is_none() {
            let graphic_entity = commands
                .spawn((
                    transform.clone(),
                    DebugOrderMoveGraphic {
                        entity_to_debug: entity,
                    },
                ))
                .current_entity()
                .unwrap();
            commands.insert_one(
                entity,
                DebugOrderMove {
                    graphic: graphic_entity,
                },
            );
        }
    }
}

pub fn order_system_visual(
    mut commands: Commands,
    order_visual_resource: Res<OrderVisualResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_debug: Query<(Entity, &DebugOrderMoveGraphic)>,
    q_exists: Query<(Entity, &Transform, &DebugOrderMove)>,
    q_orders: Query<Mutated<Orders>>,
) {
    for (graphic_debug_entity, debug) in &mut q_debug.iter() {
        let transform = q_exists.get::<Transform>(debug.entity_to_debug);
        if transform.is_err() {
            commands.despawn(graphic_debug_entity);
            continue;
        }
        let orders = q_orders.get::<Orders>(debug.entity_to_debug);
        if orders.is_err() {
            continue;
        }
        let transform = transform.unwrap();
        let orders = orders.unwrap();

        let position = transform.translation();
        let first_point = (position.x(), position.y()).into();

        let mut waypoints =
            if let Some(Order::Move(Awaitable::Awaiting(mover))) = &orders.override_order {
                vec![
                    first_point,
                    (
                        mover.get_target_position().x(),
                        mover.get_target_position().y(),
                    )
                        .into(),
                ]
            } else {
                vec![first_point]
            };
        let mut material = order_visual_resource.move_material;
        if let Some(override_order) = &orders.override_order {
            material = order_visual_resource.attack_material;
            if let Order::Move(Awaitable::Queued(mover)) = override_order {
                waypoints.push(
                    (
                        mover.get_target_position().x(),
                        mover.get_target_position().y(),
                    )
                        .into(),
                );
            }
        }
        orders.get_orders().iter().for_each(|o| {
            if let Order::Move(Awaitable::Awaiting(mover)) = o {
                waypoints.push(
                    (
                        mover.get_target_position().x(),
                        mover.get_target_position().y(),
                    )
                        .into(),
                );
            }
        });
        let line = primitive(
            material,
            &mut meshes,
            ShapeType::Polyline {
                points: waypoints,
                closed: false,
            },
            TessellationMode::Stroke(&StrokeOptions::default().with_line_width(2.0)),
            Vec3::new(0.0, 0.0, 0.0),
        );
        commands.insert(graphic_debug_entity, line);
    }
}

pub fn order_system_debug_change(mut q_orders: Query<(Entity, Mutated<Orders>)>) {
    for (entity, orders) in &mut q_orders.iter() {
        dbg!(entity, &*orders);
    }
}
