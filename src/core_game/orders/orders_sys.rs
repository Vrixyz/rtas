use crate::core_game::components::*;
use bevy::prelude::*;

use super::orders_comp::*;

pub fn order_system(
    mut query: Query<(&mut Orders, &mut Mover, &mut AIUnit, &mut MeleeAbilityState)>,
) {
    for (mut orders, mut mover, mut ai, mut melee_ability_state) in query.iter_mut() {
        if orders.override_order.is_some() {
            if let Some(order) = orders.override_order.as_mut() {
                if let Err(not_done) =
                    execute_order(&order, &mut mover, &mut ai, &mut melee_ability_state)
                {
                    orders.override_order = not_done;
                } else {
                    orders.override_order = None;
                }
            }
        }
        while orders.orders.len() > 0 {
            if let Err(not_done) = execute_order(
                &orders.orders[0],
                &mut mover,
                &mut ai,
                &mut melee_ability_state,
            ) {
                if let Some(new_order) = not_done {
                    orders.orders[0] = new_order;
                }
                break;
            }
            orders.orders.drain(0..1);
        }
    }
}

// OK means order was fully executed, Err means order is still ongoing.
type ExecutionResult = Result<(), Option<Order>>;

fn execute_order(
    order: &Order,
    mover: &mut Mut<Mover>,
    ai: &mut Mut<AIUnit>,
    melee_ability_state: &mut Mut<MeleeAbilityState>,
) -> ExecutionResult {
    match order {
        // FIXME: debug with prints, I guess nothing is changing.
        Order::Ai(new_ai) => {
            match new_ai {
                AIUnit::Passive => {
                    // TODO: stop current attack
                    melee_ability_state.interrupt();
                }
                AIUnit::SeekEnemy => {}
                AIUnit::Attack(_) => {}
            }
            **ai = new_ai.clone();
            return Ok(());
        }
        Order::Move(Awaitable::Queued(move_queued)) => {
            **mover = move_queued.clone();
            return Err(Some(Order::Move(Awaitable::Awaiting(move_queued.clone()))));
        }
        Order::Move(Awaitable::Awaiting(_)) => {
            if mover.is_target_reached {
                return Ok(());
            } else {
                return Err(None);
            }
        }
    }
}
