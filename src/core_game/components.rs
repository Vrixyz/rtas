use bevy::{math::Vec3, prelude::Entity};

pub struct UnitSize(pub f32);
#[derive(PartialEq, Eq, Hash)]
/// Useful for client to know which sprite to use
pub enum RenderSprite {
    Ogre,
    Goblin,
}

pub struct OffensiveStats {
    pub power: f32,
}

pub struct MeleeAbility {
    pub range: f32,
    pub time_to_strike: f32,
}

// TODO: use a mod to encapsulate state and structures, so the naming and and their scope is cleaner.
pub enum MeleeAbilityState {
    Hold,
    WillAttack(MeleeAbilityStateWillAttack),
}

pub struct MeleeAbilityStateWillAttack {
    pub start_time: f32,
    pub target_entity: Entity,
}
pub struct Team {
    pub id: usize,
}

pub struct Health {
    pub max_hp: f32,
    pub current_hp: f32,
}
#[derive(Default)]
pub struct SufferDamage {
    pub amount : Vec<f32>
}

impl SufferDamage {
    pub fn new_damage(&mut self, amount: f32) {
        self.amount.push(amount);
    }
}

#[derive(Clone, Debug)]
pub struct SeekEnemyRange {
    pub range: f32,
}

#[derive(Clone, Debug)]
pub enum AIUnit {
    Passive,
    SeekEnemy,
    Attack(Attack),
}
#[derive(Clone, Debug)]
pub struct Attack {
    pub target: Entity,
}
pub mod orders {
    use bevy::{math::Vec3, prelude::*};
    use bevy_prototype_lyon::prelude::*;
    use bevy_prototype_lyon::{TessellationMode, prelude::{ShapeType, StrokeOptions, primitive}};
    use super::AIUnit;

    #[derive(Clone)]
    pub struct DebugOrderMove {
        pub graphic: Entity,
    }
    #[derive(Clone)]
    pub struct DebugOrderMoveGraphic {
        entity_to_debug: Entity,
    }
    // Hide mover to avoid doing bad things, because only
    #[derive(Clone, Debug)]
    pub struct Mover {
        target_position: Vec3,
        pub is_target_reached: bool,
    }
    #[derive(Clone, Debug)]
    pub struct Speed {
        pub speed: f32,
    }

    impl Mover {
        pub fn new(position: Vec3) -> Self {
            Mover { target_position: position, is_target_reached: true}
        }
        pub fn new_to_target(position: Vec3) -> Self {
            Mover { target_position: position, is_target_reached: false}
        }
        pub fn get_target_position(&self) -> &Vec3 {
            &self.target_position
        }
    }
    #[derive(Debug)]
    pub enum Order {
        Ai(AIUnit), // effect is instant
        Move(Awaitable<Mover>), // wait for reaching target.
    }
    #[derive(Debug)]
    pub enum Awaitable<T> {
        Queued(T),
        Awaiting(T),
    }
    #[derive(Default, Debug)]
    pub struct Orders {
        orders: Vec<Order>,
        pub override_order: Option<Order>,
    }
    impl Orders {
        pub fn add_order(&mut self, new_order: Order) {
            self.orders.push(new_order);
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
    }    // OK means order was fully executed, Err means order is still ongoing.
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
    pub fn order_system_debug_init(mut commands: Commands,
        mut q_orders: Query<(Entity, &Transform, &Orders, Option<&DebugOrderMove>)>) {
            for (entity, transform, _, debug_marker) in &mut q_orders.iter() {
                if debug_marker.is_none() {
                    let graphic_entity = commands.spawn((transform.clone(), DebugOrderMoveGraphic {
                        entity_to_debug: entity,
                    })).current_entity().unwrap();
                    commands.insert_one(entity, DebugOrderMove{graphic: graphic_entity});
                }
            }
    }
    pub fn order_system_debug(mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut q_debug: Query<(Entity, &DebugOrderMoveGraphic)>,
        q_exists: Query<(Entity, &Transform, &DebugOrderMove)>,
        q_orders: Query<Mutated<Orders>>) {
        
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

            let red = materials.add(Color::rgb(0.8, 0.0, 0.0).into());
            let position = transform.translation();
            let first_point = (position.x(), position.y()).into();
            
            let mut waypoints = 
                if let Some(Order::Move(Awaitable::Awaiting(mover))) = &orders.override_order {
                    vec![first_point, (mover.target_position.x(), mover.target_position.y()).into()]
                }
                else {
                    vec![first_point]
                };
            orders.orders.iter().for_each(|o| {
                if let Order::Move(Awaitable::Awaiting(mover)) = o {
                    waypoints.push((mover.target_position.x(), mover.target_position.y()).into());
            }});
            let line = primitive(
                red,
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

    pub fn order_system_debug_change(
        mut q_orders: Query<(Entity, Mutated<Orders>)>) {
        
        for (entity, orders) in &mut q_orders.iter() {
            dbg!(entity, &*orders);
        }
    }
}
