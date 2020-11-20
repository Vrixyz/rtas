use bevy::{math, prelude::*};
use bevy_rapier2d::{na::Point2, na::{Translation2, UnitComplex}, physics::{RapierConfiguration, RigidBodyHandleComponent}, rapier::math::Isometry, rapier::{dynamics::IntegrationParameters, dynamics::RigidBody, dynamics::RigidBodyBuilder, dynamics::RigidBodySet, geometry::ColliderBuilder, dynamics::MassProperties, math::Vector}};

use crate::core_game::{orders::orders_comp::*, components::*};

pub fn physics_setup(
    mut configuration: ResMut<RapierConfiguration>,
    mut integration: ResMut<IntegrationParameters>
) {
    configuration.gravity = Default::default();

    integration.erp = 0.2;
}

pub fn physics_init(
    mut commands: Commands,
    q: Query<Without<RigidBodyHandleComponent, (Entity, &UnitSize, &Transform)>>
) {
    for (e, size, transform) in q.iter() {
        let rigid_body2 = RigidBodyBuilder::new_dynamic()
            .mass(size.0)
            .translation(transform.translation.x(), transform.translation.y())
            .angular_damping(f32::MAX)
        ;
        let collider2 = ColliderBuilder::ball(size.0);
        commands.insert(e, (rigid_body2, collider2));
    }
}

fn rotate_towards(current_rotation: UnitComplex<f32>, direction: Vec3, max_rotation: f32) -> Option<UnitComplex<f32>> {
    let current_angle = current_rotation.angle();
    
    let corrected_angle = current_angle.to_degrees();
    let angle_target = direction.y().atan2(direction.x()).to_degrees();
    let mut angle_to_rotate = angle_target - corrected_angle;
    if angle_to_rotate > 180f32 {
        angle_to_rotate -= 360f32;
    }
    else if angle_to_rotate < -180f32 {
        angle_to_rotate += 360f32;
    }
    let max_rotation_this_frame = max_rotation;
    let rotation_this_frame = f32::min(max_rotation_this_frame, angle_to_rotate.abs()) * angle_to_rotate.signum();

    if angle_to_rotate.abs() <= 5.0 {
        None
    }
    else {
        Some(UnitComplex::new(rotation_this_frame.to_radians()) * current_rotation)
    }
}

pub fn mover_update(
    time: Res<Time>,
    mut bodies: ResMut<RigidBodySet>,
    mut query: Query<(
        &mut Mover,
        &Speed,
        &mut RigidBodyHandleComponent,
        &Transform,
        Option<&MeleeAbilityState>,
        Option<&RotateBeforeMove>,
    )>,
    mut q_target: Query<&Transform>,
) {
    for (mut mover, speed, body, transform, melee_state, rotate_before_move) in query.iter_mut() {
        
        let body = bodies.get_mut(body.handle());
        if body.is_none() {
            continue;
        }
        let mut body = body.unwrap();
        body.angvel = 0.0;
        let position = body.position.translation;
        let position = Vec3::new(position.x, position.y, 0f32);
        if let Some(MeleeAbilityState::WillAttack(will_attack)) = melee_state {
            body.linvel = Vector::new(0.0, 0.0);
            if let Some(rotation) = rotate_before_move {
                if let Ok(target_position) = q_target.get_component::<Transform>(will_attack.target_entity) {
                    let mut offset = target_position.translation - position;
                    if let Some(new_rotation) = rotate_towards(body.position.rotation, offset, rotation.rotation_speed * time.delta_seconds) {
                        body.position.rotation = new_rotation;
                    }
                }
            }
            continue;
        }
        if mover.is_target_reached {
            //continue;
        }
        let target = mover.get_target_position();
        let mut offset = *target - position;
        let offset_distance = offset.length();
        if offset_distance < 0.02 {
            mover.is_target_reached = true;
            body.linvel = Default::default();
            continue;
        }

        if let Some(rotation) = rotate_before_move {
                if let Some(new_rotation) = rotate_towards(body.position.rotation, offset, rotation.rotation_speed * time.delta_seconds) {
                    body.position.rotation = new_rotation;
                    body.linvel = Default::default();
                    continue;
                }
        }
        if speed.speed == 0.0 {
            continue;
        }
        offset = offset.normalize();
        let distance_to_move = speed.speed * time.delta_seconds_f64 as f32;
        offset *= f32::min(distance_to_move, offset_distance);

        // If no physics:
            // let new_position: Isometry<f32> = Isometry::new(bevy_rapier2d::na::Vector2::new(new_position.x(),new_position.y()), Default::default());
            // transform.translation = new_position;
        // Else:
            let mut speed_to_apply = speed.speed;
            let distance_in_a_frame = speed.speed * 1.0 / 60.0;
            if offset_distance < distance_in_a_frame {
                speed_to_apply = offset.length() * 1.0 / 60.0;
            }
            body.linvel = Vector::new(offset.x(), offset.y()).normalize() * speed_to_apply;
            body.wake_up(true);
        //
    }
}