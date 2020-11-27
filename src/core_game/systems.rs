use super::{map::Map, components::*, orders::orders_comp::*};
use bevy::prelude::*;

// Bundles
#[derive(Bundle)]
pub struct UnitBundle {
    size: UnitSize,
    transform: Transform,
    global_transform: GlobalTransform,
    render_sprite: RenderSprite,

    // should be added after (for all units having "Speed")
    mover: Mover,
    rotate_before_move: RotateBeforeMove,
    speed: Speed,
    team: Team,
    ai_unit: AIUnit,
    seek_enemy_range: SeekEnemyRange,
    melee_ability: MeleeAbility,
    // should be added after (for all units having "MeleeAbility")
    melee_ability_state: MeleeAbilityState,
    offensive_stats: OffensiveStats,
    health: Health,
    // should be added after (for all units having "Health")
    suffer_damage: SufferDamage,
    // should be added after (for all units having "Mover")
    orders: Orders,
}
pub fn create_bandit_unit(team: Team, position: Vec3) -> UnitBundle {
    UnitBundle {
        size: UnitSize(20f32),
        transform: Transform::from_translation(position),
        global_transform: GlobalTransform::from_translation(position),
        render_sprite: RenderSprite::Bandit,
        mover: Mover::new(position),
        rotate_before_move: RotateBeforeMove {rotation_speed: 700f32},
        speed: Speed { speed: 80f32 },
        team,
        ai_unit: AIUnit::SeekEnemy,
        seek_enemy_range: SeekEnemyRange { range: 200f32 },
        melee_ability: MeleeAbility {
            range: 100f32,
            motion_buffer_range: 3f32,
            time_to_strike: 0.4f32,
            cooldown: 0.2f32,
        },
        offensive_stats: OffensiveStats { power: 4f32 },
        melee_ability_state: MeleeAbilityState::Ready,
        health: Health {
            max_hp: 10f32,
            current_hp: 10f32,
        },
        suffer_damage: SufferDamage::default(),
        orders: Orders::default(),
    }
}
pub fn create_goblin_unit(team: Team, position: Vec3) -> UnitBundle {
    UnitBundle {
        size: UnitSize(20f32),
        transform: Transform::from_translation(position),
        global_transform: GlobalTransform::from_translation(position),
        render_sprite: RenderSprite::Goblin,
        mover: Mover::new(position),
        rotate_before_move: RotateBeforeMove {rotation_speed: 720f32},
        speed: Speed { speed: 120f32 },
        team,
        ai_unit: AIUnit::SeekEnemy,
        seek_enemy_range: SeekEnemyRange { range: 200f32 },
        melee_ability: MeleeAbility {
            range: 5f32,
            motion_buffer_range: 3f32,
            time_to_strike: 0.2f32,
            cooldown: 0.2f32,
        },
        offensive_stats: OffensiveStats { power: 2f32 },
        melee_ability_state: MeleeAbilityState::Ready,
        health: Health {
            max_hp: 20f32,
            current_hp: 20f32,
        },
        suffer_damage: SufferDamage::default(),
        orders: Orders::default(),
    }
}
pub fn create_ogre_unit(team: Team, position: Vec3) -> UnitBundle {
    UnitBundle {
        size: UnitSize(40f32),
        transform: Transform::from_translation(position),
        global_transform: GlobalTransform::from_translation(position),
        render_sprite: RenderSprite::Ogre,
        mover: Mover::new(position),
        rotate_before_move: RotateBeforeMove {rotation_speed: 90f32},
        speed: Speed { speed: 30f32,
        },
        team,
        ai_unit: AIUnit::SeekEnemy,
        seek_enemy_range: SeekEnemyRange { range: 200f32 },
        melee_ability: MeleeAbility {
            range: 10f32,
            motion_buffer_range: 3f32,
            time_to_strike: 1.2f32,
            cooldown: 0.34f32,
        },
        offensive_stats: OffensiveStats { power: 13f32 },
        melee_ability_state: MeleeAbilityState::Ready,
        health: Health {
            max_hp: 250f32,
            current_hp: 250f32,
        },
        suffer_damage: SufferDamage::default(),
        orders: Orders::default(),
    }
}

pub fn create_units(mut commands: Commands,
    map: Res<Map>,
) {
    const OFFSET_POSITION: f32 = 40f32;
    const NB_GOBLINS: u32 = 5;
    const NB_BANDITS: u32 = 5;
    for i in 0..NB_GOBLINS {
        let position = Vec3::new((i as f32 - (NB_GOBLINS as f32) / 2f32) * OFFSET_POSITION, 0.0, 0.0);
        commands.spawn(create_goblin_unit(Team { id: 0 }, position))
        ;
    }
    if let Some(start) = map.map.starting_point {
        let real_start = Map::real_position_at(start.x, start.y);
        for i in 0..NB_BANDITS {

            let position = Vec3::new((i as f32 - (NB_BANDITS as f32) / 2f32) * OFFSET_POSITION + real_start.x(), real_start.y(), 0.0);
            
            commands.spawn(create_bandit_unit(Team { id: 2 }, position))
            ;
        }
    }
    const OFFSET_POSITION_OGRE: f32 = 100f32;
    const NB_OGRES: u32 = 1;
    for i in 0..NB_OGRES {
        let ogre_position = Vec3::new((i as f32 - (NB_OGRES as f32) / 2f32) * OFFSET_POSITION_OGRE, -300.0, 0.0);
        commands.spawn(create_ogre_unit(Team { id: 1 }, ogre_position))
        ;
    }
}

pub fn ai_system(
    time: Res<Time>,
    mut ais: Query<(
        &Team,
        &SeekEnemyRange,
        &mut AIUnit,
        &mut Orders,
        &MeleeAbility,
        &mut MeleeAbilityState,
        &Transform,
        &UnitSize,
    )>,
    attackable: Query<(&Team, &Transform, Entity, &UnitSize)>,
) {
    for (
        a_team,
        seek_enemy_range,
        mut ai,
        mut a_orders,
        melee_ability,
        mut melee_state,
        a_transform,
        a_size,
    ) in ais.iter_mut()
    {
        if matches!(*ai, AIUnit::Passive) {
            continue;
        }
        if matches!(*melee_state, MeleeAbilityState::WillAttack(_)) {
            continue;
        }
        let a_position = a_transform.translation;
        let mut new_ai: Option<AIUnit> = None;
        if matches!(*ai, AIUnit::SeekEnemy) {
            let mut closest_distance = f32::MAX;
            for (b_team, b_transform, b_entity, _) in attackable.iter() {
                if a_team.id == b_team.id {
                    continue;
                }
                let new_distance = (a_position - b_transform.translation).length();
                if new_distance <= seek_enemy_range.range && new_distance < closest_distance {
                    closest_distance = new_distance;
                    new_ai = Some(AIUnit::Attack(Attack {
                        target: b_entity.clone(),
                        chase_when_target_too_far: false,
                    }));
                }
            }
        }
        if let Some(new_ai) = new_ai {
            *ai = new_ai;
        }
        if let AIUnit::Attack(ai_attacker) = &*ai {
            if let Ok(target_transform) = attackable.get_component::<Transform>(ai_attacker.target.clone()) {
                if !ai_attacker.chase_when_target_too_far {
                    let new_distance = (a_position - target_transform.translation).length();
                    if new_distance > seek_enemy_range.range {
                        *ai = AIUnit::SeekEnemy;
                        continue;
                    }    
                }
                if matches!(*melee_state, MeleeAbilityState::MotionBufferExceeded) {
                    if !ai_attacker.chase_when_target_too_far {
                        *ai = AIUnit::SeekEnemy;
                        continue;
                    }
                }
                let size = attackable.get_component::<UnitSize>(ai_attacker.target).unwrap();
                if (target_transform.translation - a_position).length()
                    < melee_ability.range + size.0 + a_size.0
                {
                    if matches!(*melee_state, MeleeAbilityState::Ready) {
                        a_orders.override_order = Some(Order::Move(Awaitable::Queued(
                            Mover::new_to_target(a_transform.translation),
                        )));
                        *melee_state = MeleeAbilityState::WillAttack(MeleeAbilityStateWillAttack {
                            start_time: time.time_since_startup().as_secs_f32(),
                            target_entity: ai_attacker.target.clone(),
                        });
                    }
                } else {
                    // FIXME: if the override_order is already at this value, we shouldn't update it (target is not moving), so:
                    // - we don't trigger a modification on the Orders.
                    // - and orders are not redrawn
                    a_orders.override_order = Some(Order::Move(Awaitable::Queued(
                        Mover::new_to_target(target_transform.translation),
                    )));
                }
            } else {
                *ai = AIUnit::SeekEnemy;
            }
        }
    }
}

pub fn attack_melee_system(
    time: Res<Time>,
    mut q: Query<(
        &Transform,
        &MeleeAbility,
        &mut MeleeAbilityState,
        &OffensiveStats,
        &UnitSize,
    )>,
    mut q_victim: Query<(&Transform, &UnitSize, &mut SufferDamage)>,
) {
    for (transform, ability, mut state, offensive_stats, size) in q.iter_mut() {
        // TODO: use an additional "recovering" state, (+ Client: spawn particles ; floating text for damage)
        match &*state {
            MeleeAbilityState::Ready => {}
            MeleeAbilityState::WillAttack(attack_state) => {
                if let Ok(a_transform) = q_victim.get_component::<Transform>(attack_state.target_entity) {
                    // Check if still in range
                    let a_size =
                        if let Ok(a_size) = q_victim.get_component::<UnitSize>(attack_state.target_entity) {
                            a_size.0
                        } else {
                            0f32
                        };
                    let distance = (a_transform.translation - transform.translation).length();
                    if distance > ability.range + ability.motion_buffer_range + size.0 + a_size {
                        *state = MeleeAbilityState::MotionBufferExceeded;
                        return;
                    }
                } else {
                    // target is not valid
                    *state = MeleeAbilityState::Ready;
                    return;
                }
                let time = time.time_since_startup().as_secs_f32();
                if time > attack_state.start_time + ability.time_to_strike {
                    if let Ok(mut suffer_damage) =
                        q_victim.get_component_mut::<SufferDamage>(attack_state.target_entity)
                    {
                        suffer_damage.new_damage(offensive_stats.power);
                        *state = MeleeAbilityState::AttackCooldown(MeleeAbilityStateCooldown {
                            start_time: time,
                        });
                    } else {
                        // target is not valid
                        *state = MeleeAbilityState::Ready;
                        return;
                    }
                }
            }
            MeleeAbilityState::MotionBufferExceeded => {
                *state = MeleeAbilityState::Ready;
            }
            MeleeAbilityState::AttackCooldown(cooldown) => {
                if time.time_since_startup().as_secs_f32() > cooldown.start_time + ability.cooldown
                {
                    *state = MeleeAbilityState::Ready;
                }
            }
        }
    }
}

pub fn health_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Health, &mut SufferDamage)>,
) {
    for (entity, mut health, mut suffer_damage) in q.iter_mut() {
        while suffer_damage.amount.len() > 0 {
            health.current_hp -= suffer_damage.amount.last().unwrap();
            suffer_damage.amount.pop();
        }
        if health.current_hp <= 0f32 {
            commands.despawn_recursive(entity);
        }
    }
}
