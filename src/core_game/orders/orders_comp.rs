use crate::core_game::components::*;
use bevy::prelude::*;
use bevy::{math::Vec3, prelude::Component};
use bevy_inspector_egui::Inspectable;

// Hide mover to avoid doing bad things, because only
#[derive(Component, Clone, Debug, Inspectable)]
pub struct Mover {
    pub(super) target_position: Vec3,
    pub is_target_reached: bool,
}
#[derive(Component, Inspectable)]
pub struct RotateBeforeMove {
    pub rotation_speed: f32,
}
#[derive(Component, Clone, Debug, Inspectable)]
pub struct Speed {
    pub speed: f32,
}

impl Mover {
    pub fn new(position: Vec3) -> Self {
        Mover {
            target_position: position,
            is_target_reached: true,
        }
    }
    pub fn new_to_target(position: Vec3) -> Self {
        Mover {
            target_position: position,
            is_target_reached: false,
        }
    }
    pub fn get_target_position(&self) -> &Vec3 {
        &self.target_position
    }
}

#[derive(Debug)]
pub enum Order {
    Ai(AIUnit),             // effect is instant
    Move(Awaitable<Mover>), // wait for reaching target.
}

#[derive(Debug)]
pub enum Awaitable<T> {
    Queued(T),
    Awaiting(T),
}

#[derive(Component, Default, Debug)]
pub struct Orders {
    pub(super) orders: Vec<Order>,
    pub override_order: Option<Order>,
}
impl Orders {
    pub fn add_orders(&mut self, mut new_orders: Vec<Order>) {
        self.orders.append(&mut new_orders);
    }
    pub fn replace_orders(&mut self, new_orders: Vec<Order>) {
        self.orders = new_orders;
        self.override_order = None;
    }
    pub fn get_orders(&self) -> &Vec<Order> {
        &self.orders
    }
    pub fn order_move(target: Vec3) -> Order {
        Order::Move(Awaitable::Queued(Mover::new_to_target(target)))
    }
}
