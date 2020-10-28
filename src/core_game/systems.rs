use bevy::{math, prelude::*};
use super::components::*;
use super::components::Orderable::*;


pub fn create_units(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                ..Default::default()
            })
        .with(Selectable {is_selected: false, half_size: 10f32})
        .with(Mover::new(Vec3::new(0.0, 0.0, 0.0), 50f32))
        .with(Team {id: 0})
        .with(AIUnit::SeekEnemy(SeekEnemy{range:200f32}))
        .with(MeleeAbility {
            range: 15f32,
            cooldown: 1.5f32,
        })
        .with(MeleeAbilityState::Hold)
        .with(Health{max_hp: 20f32, current_hp: 20f32})
        .with(Orders::default())
        .with_children(|parent| {
            parent.spawn(SpriteComponents {
                material: materials.add(Color::rgba(1.0, 0.5, 0.0, 0.2).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(26.0, 26.0)),
                ..Default::default()
            }).with(SelectionVisual);
        })
        
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Selectable {is_selected: false, half_size: 10f32})
        .with(Mover::new(Vec3::new(0.0, -200.0, 0.0), 50f32))
        .with(Team {id: 1})
        .with(AIUnit::SeekEnemy(SeekEnemy{range:200f32}))
        .with(MeleeAbility {
            range: 15f32,
            cooldown: 0.6f32,
        })
        .with(MeleeAbilityState::Hold)
        .with(Health{max_hp: 20f32, current_hp: 20f32})
        .with(Orders::default())
        .with_children(|parent| {
            parent.spawn(SpriteComponents {
                material: materials.add(Color::rgba(1.0, 0.5, 0.0, 0.2).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(26.0, 26.0)),
                ..Default::default()
            }).with(SelectionVisual);
        })

        /*
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.0, 0.5, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Selectable {is_selected: false})
        .with(Mover{target_position:Vec3::default(), speed: 150f32, is_moving: false})
        .with(Team {id: 2})
        .with(AIUnit::SeekEnemy(SeekEnemy{range:200f32}))
        .with(MeleeAbilityState::Hold)
        .with(Health{max_hp: 20f32, current_hp: 20f32})
        .with_children(|parent| {
            parent.spawn(SpriteComponents {
                material: materials.add(Color::rgba(1.0, 0.5, 0.0, 0.2).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                sprite: Sprite::new(Vec2::new(26.0, 26.0)),
                ..Default::default()
            }).with(SelectionVisual);
        })*/
        ;
}

pub fn mover_update(time: Res<Time>, mut query: Query<(&mut Mover, &mut Transform, Option<&MeleeAbilityState>)>) {
    for (mut mover, mut transform, meleeState) in &mut query.iter() {
        if matches!(meleeState, Some(MeleeAbilityState::Attacking(_))) {
            continue;
        }
        // TODO: check if the unit is attacking, if it's the case, we can't move.
        if mover.is_target_reached {
            continue;
        }
        let position = transform.translation();
        let target = mover.get_target_position();
        let mut offset = *target - position;
        let offset_distance = offset.length();
        if offset_distance < 0.01 {
            mover.is_target_reached = true;
            continue;
        }
        offset = offset.normalize();
        let distance_to_move = mover.speed * time.delta_seconds_f64 as f32; 
        offset *= f32::min(distance_to_move, offset_distance);

        transform.set_translation(position + math::vec3(offset.x(), offset.y(), 0f32));
    }
}

pub fn ai_system(time: Res<Time>, mut ais: Query<(&Team, &mut AIUnit, &mut Orders, &MeleeAbility, &mut MeleeAbilityState, &Transform)>, mut attackable: Query<(&Team, &Transform, Entity)>) {
    for (a_team, mut ai, mut a_orders, melee_ability, mut melee_state, a_transform) in &mut ais.iter() {
        if matches!(*ai, AIUnit::Passive) {
            continue;
        }
        if matches!(*melee_state, MeleeAbilityState::Attacking(_))  {
            continue;
        }
        let a_position = a_transform.translation();
        let mut new_ai: Option<AIUnit> = None;
        if let AIUnit::SeekEnemy(ai_seeker) = &mut *ai {
            for (b_team, b_transform, b_entity) in &mut attackable.iter() {
                if a_team.id == b_team.id {
                    continue;
                }
                if (a_position - b_transform.translation()).length() <= ai_seeker.range {
                    new_ai = Some(AIUnit::Attack(Attack{target: b_entity.clone()}));
                    break;
                }
            }
        }
        else if let AIUnit::Attack(ai_attacker) = &mut *ai {
            if let Ok(target_transform) =  attackable.get::<Transform>(ai_attacker.target.clone()) {
                if (target_transform.translation() - a_position).length() < melee_ability.range {
                    *melee_state = MeleeAbilityState::Attacking(MeleeAbilityStateAttacking{start_time: time.time_since_startup().as_secs_f32()});
                }
                else {
                    // FIXME: if the override_order is already at this value, we shouldn't update it (target is not moving), so we don't trigger a modification on the Orders.
                    a_orders.override_order = Some(Order::Move(Awaitable::Queued(Mover::new_to_target(target_transform.translation(), 50f32))));
                }
            }
        }
        if let Some(new_ai) = new_ai {
            *ai = new_ai;
        }
    }
}

pub fn attack_melee_system(time: Res<Time>, mut q: Query<(&MeleeAbility, &mut MeleeAbilityState)>) {
    for (ability, mut state) in &mut q.iter() {
        // TODO: use a "recovering" state, use an event rather than a state to materialize attack? + spawn particles and stuff ; how will it work on network though ?
        if let MeleeAbilityState::Attacking(attack_state) = &*state {
            if time.time_since_startup().as_secs_f32() >  attack_state.start_time + ability.cooldown {
                *state = MeleeAbilityState::Hold;
            }
        }
    }
}