
use bevy::{math::Vec3, prelude::*};
use crate::core_game::components::*;

use super::orders_comp::*;

pub fn order_system(mut query: Query<(&mut Orders, &mut Mover, &mut AIUnit)>) {
    for (mut orders, mut mover, mut ai) in &mut query.iter() {
        let mut is_order_complete = false;
        if orders.override_order.is_some() {
            if let Some(mut order) = orders.override_order.as_mut() {
                if let Err(not_done) = execute_order(&order, &mut mover, &mut ai) {
                    orders.override_order = not_done;
                }
                else {
                    orders.override_order = None;
                }
            }    
        }
        while !is_order_complete && orders.orders.len() > 0 {
            if let Err(not_done) = execute_order(&orders.orders[0], &mut mover, &mut ai) {
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

/// Returns true if order is fully executed
fn execute_order(order: &Order, mover: &mut Mut<Mover>, ai: &mut Mut<AIUnit>) -> ExecutionResult {
    match order {
                        // FIXME: debug with prints, I guess nothing is changing.
        Order::Ai(new_ai) =>  {
            **ai = new_ai.clone();
            return Ok(());
        },
        Order::Move(Awaitable::Queued(move_queued)) => {
            **mover = move_queued.clone();
            return Err(Some(Order::Move(Awaitable::Awaiting(move_queued.clone()))));
        },
        Order::Move(Awaitable::Awaiting(move_wait)) => {
            if mover.is_target_reached {
                return Ok(());
            }
            else {
                return Err(None);
            }
        },
        
    }
}