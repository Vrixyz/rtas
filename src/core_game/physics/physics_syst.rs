use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::{
    na::{Isometry2, UnitComplex},
    rapier::{dynamics::IntegrationParameters, math::Vector},
};

use super::PHYSICS_PIXEL_PER_METER;
use crate::core_game::{components::*, orders::orders_comp::*};

#[derive(Component)]
pub struct PhysicsInitialized;

pub fn physics_setup(
    mut configuration: ResMut<RapierConfiguration>,
    mut context: ResMut<RapierContext>,
) {
    configuration.gravity = Default::default();

    context.integration_parameters.erp = 0.8;
}

pub fn physics_init(
    mut commands: Commands,
    q: Query<(Entity, &UnitSize, &Transform), Without<PhysicsInitialized>>,
) {
    for (e, size, transform) in q.iter() {
        commands
            .entity(e)
            .insert(RigidBody::Dynamic)
            .insert(Velocity::zero())
            .insert(Collider::ball(size.0))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Transform::from_translation(transform.translation))
            .insert(PhysicsInitialized);
    }
}

fn rotate_towards(current_rotation: Quat, direction: Vec3, max_rotation: f32) -> Option<Quat> {
    let (axis, angle) = Quat::from_rotation_arc(
        current_rotation * Vec3::new(1f32, 0f32, 0f32),
        direction.normalize(),
    )
    .to_axis_angle();
    if angle <= 5_f32.to_radians() {
        return None;
    }
    let angle = angle.min(max_rotation.to_radians());

    Some(Quat::from_axis_angle(axis, angle) * current_rotation)
}

pub fn mover_update(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Mover,
        &Speed,
        &mut Velocity,
        Option<&MeleeAbilityState>,
        Option<&RotateBeforeMove>,
    )>,
    mut q_target: Query<&mut Transform>,
) {
    for (e, mut mover, speed, mut velocity, melee_state, rotate_before_move) in query.iter_mut() {
        if let Some(MeleeAbilityState::WillAttack(will_attack)) = melee_state {
            velocity.linvel = Vec2::new(0.0, 0.0);
            if let Some(rotation) = rotate_before_move {
                if let Ok(target_position) =
                    q_target.get_component::<Transform>(will_attack.target_entity)
                {
                    let target_position = target_position.translation;
                    if let Ok(mut transform) = q_target.get_component_mut::<Transform>(e) {
                        let offset = target_position - transform.translation;
                        if let Some(new_rotation) = rotate_towards(
                            transform.rotation,
                            offset,
                            rotation.rotation_speed * time.delta_seconds(),
                        ) {
                            transform.rotation = new_rotation;
                        }
                    }
                }
            }
            continue;
        }
        if mover.is_target_reached {
            //continue;
        }
        let target = mover.get_target_position();

        if let Ok(mut transform) = q_target.get_component_mut::<Transform>(e) {
            let mut offset = *target - transform.translation;
            let offset_distance = offset.length();
            if offset_distance < 2.0 {
                mover.is_target_reached = true;
                velocity.linvel = Default::default();
                continue;
            }

            if let Some(rotation) = rotate_before_move {
                if let Some(new_rotation) = rotate_towards(
                    transform.rotation,
                    offset,
                    rotation.rotation_speed * time.delta_seconds(),
                ) {
                    transform.rotation = new_rotation;
                    velocity.linvel = Default::default();
                    continue;
                }
            }
            if speed.speed == 0.0 {
                continue;
            }
            offset = offset.normalize();
            let distance_to_move = speed.speed * time.delta_seconds_f64() as f32;
            offset *= f32::min(distance_to_move, offset_distance);

            // If no physics:
            // let new_position: Isometry<f32> = Isometry::new(bevy_rapier2d::na::Vector2::new(new_position.x,new_position.y), Default::default());
            // transform.translation = new_position;
            // Else:
            let mut speed_to_apply = speed.speed;
            let distance_in_a_frame = speed.speed * 1.0 / 60.0;
            if offset_distance < distance_in_a_frame {
                speed_to_apply = offset.length() * 1.0 / 60.0;
            }
            velocity.linvel = Vec2::new(offset.x, offset.y).normalize() * speed_to_apply;
        }

        // TO CHECK: old code had to wake up that

        //body.wake_up(true);

        //
    }
}
