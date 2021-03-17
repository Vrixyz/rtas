use bevy::{math::Vec3, prelude::*};
use bevy_prototype_lyon::{
    prelude::{primitive, ShapeType, StrokeOptions},
    TessellationMode,
};

use crate::{client::components::*, client::selection, core_game::components::Attack, core_game::{components::{AIUnit, Health, Team}, orders::orders_comp::*, pathfinding::pathfinding_comp::Map}};

use super::orders_comp::*;

pub fn move_order_system(
    cursor_state: Res<MyCursorState>,
    mouse_button: Res<Input<MouseButton>>,
    key_button: Res<Input<KeyCode>>,
    team: Res<TeamResource>,
    selection: Res<Selection>,
    map: Res<Map>,
    q_attackables: Query<(Entity, &Transform, &Team, &Health, &Selectable)>,
    mut query: Query<(&mut Orders, &Selectable, &Team, &Transform)>,
) {
    if mouse_button.just_pressed(MouseButton::Right) {
        if let Selection::Hover(Some(selected)) = *selection {
            if let Ok(a_team) = q_attackables.get_component::<Team>(selected) {
                if a_team.id != team.team.id {
                    for (mut orders, selectable, b_team, _) in query.iter_mut() {
                        if b_team.id != team.team.id {
                            continue;
                        }
                        if selectable.is_selected {
                            let mut new_orders = vec![Order::Ai(AIUnit::Attack(Attack {
                                target: selected,
                                chase_when_target_too_far: true,
                            }))];

                            if key_button.pressed(KeyCode::RShift) || key_button.pressed(KeyCode::LShift) {
                                orders.add_orders(new_orders);
                            }
                            else {
                                orders.replace_orders(new_orders);
                            }
                        }
                    }
                    return;
                }
            }
        }

        let mut selected_units = vec![];
        for (orders, selectable, b_team, transform) in query.iter_mut() {
            if b_team.id != team.team.id {
                continue;
            }
            if selectable.is_selected {
                selected_units.push((orders, transform.translation));
            }
        }
        if selected_units.len() > 1 {
            let mut min = Vec3::new(f32::MAX, f32::MAX, 0.0);
            let mut max = Vec3::new(f32::MIN, f32::MIN, 0.0);
            for (_, position) in selected_units.iter() {
                if position.x() < min.x() {
                    min.set_x(position.x());
                }
                if position.y() < min.y() {
                    min.set_y(position.y());
                }
                if position.x() > max.x() {
                    max.set_x(position.x());
                }
                if position.y() > max.y() {
                    max.set_y(position.y());
                }
            }
            // Outside magic box
            if !selection::helpers::helper_in_rect(&cursor_state.world_position, &Position::from(&min), &Position::from(&max)) {
                let center: Vec3 = (min + max) / 2.0;
                for (orders, position) in selected_units.iter_mut() {
                    let offset = position.clone() - center.clone();
                    let target = Vec3::new(
                        cursor_state.world_position.x,
                        cursor_state.world_position.y,
                        0f32,
                    ) + offset;
                    let mut new_orders = vec![
                        if key_button.pressed(KeyCode::A) { 
                            Order::Ai(AIUnit::SeekEnemy)
                        } else
                        {
                            Order::Ai(AIUnit::Passive)
                        }];
                    if map.is_ready() {
                        let start_map = (crate::core_game::map::Map::map_x_at(position.x()) as i32, crate::core_game::map::Map::map_y_at(position.y()) as i32);
                        let target_map = (crate::core_game::map::Map::map_x_at(target.x()) as i32, crate::core_game::map::Map::map_y_at(target.y()) as i32);
                        if let Ok(path) = map.dijkstra(start_map, target_map) {
                            let mut real_path = path.into_iter().map(|pos| {
                                let real_pos = crate::core_game::map::Map::real_position_at(pos.0 as usize, pos.1 as usize);
                                Orders::order_move(Vec3::new(
                                    // TODO: convert to real
                                    real_pos.x(),
                                    real_pos.y(),
                                    0f32
                                ))
                            }).collect::<Vec<Order>>();
                            new_orders.append(&mut real_path);
                        }
                        else {
                            new_orders.push(Orders::order_move(target))
                        }        
                    }
                    else {
                        new_orders.push(Orders::order_move(target))
                    }
                    new_orders.push(
                        Order::Ai(AIUnit::SeekEnemy),
                    );
                    if key_button.pressed(KeyCode::RShift) || key_button.pressed(KeyCode::LShift) {
                        orders.add_orders(new_orders);
                    }
                    else {
                        orders.replace_orders(new_orders);
                    }
                }
                return;
            }
        }
        // Inside magic box
        for (orders, _) in selected_units.iter_mut() {
            let new_orders = vec![
                if key_button.pressed(KeyCode::A) { 
                    Order::Ai(AIUnit::SeekEnemy)
                } else
                {
                    Order::Ai(AIUnit::Passive)
                },
                Orders::order_move(Vec3::new(
                    cursor_state.world_position.x,
                    cursor_state.world_position.y,
                    0f32,
                )),
                Order::Ai(AIUnit::SeekEnemy),
            ];
            if key_button.pressed(KeyCode::RShift) || key_button.pressed(KeyCode::LShift) {
                orders.add_orders(new_orders);
            }
            else {
                orders.replace_orders(new_orders);
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
    q_orders: Query<(Entity, &Transform, &Orders, Option<&DebugOrderMove>)>,
) {
    for (entity, transform, _, debug_marker) in q_orders.iter() {
        if debug_marker.is_none() {
            commands
                .spawn((
                    transform.clone(),
                    DebugOrderMoveGraphic {
                        entity_to_debug: entity,
                    },
                ));
            commands.insert_one(
                entity,
                DebugOrderMove,
            );
        }
    }
}

pub fn order_system_visual(
    mut commands: Commands,
    order_visual_resource: Res<OrderVisualResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_debug: Query<(Entity, &DebugOrderMoveGraphic)>,
    q_exists: Query<(Entity, &Transform, &DebugOrderMove)>,
    q_orders: Query<Mutated<Orders>>,
) {
    for (graphic_debug_entity, debug) in &mut q_debug.iter() {
        let transform = q_exists.get_component::<Transform>(debug.entity_to_debug);
        if transform.is_err() {
            commands.despawn(graphic_debug_entity);
            continue;
        }
        let orders = q_orders.get_component::<Orders>(debug.entity_to_debug);
        if orders.is_err() {
            continue;
        }
        let transform = transform.unwrap();
        let orders = orders.unwrap();

        let position = transform.translation;
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
        let mut material = &order_visual_resource.move_material;
        if let Some(override_order) = &orders.override_order {
            material = &order_visual_resource.attack_material;
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
            if let Order::Move(Awaitable::Queued(mover)) = o {
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
            material.clone(),
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

pub fn order_system_debug_change(q_orders: Query<(Entity, Mutated<Orders>)>) {
    for (entity, orders) in q_orders.iter() {
        dbg!(entity, &*orders);
    }
}
