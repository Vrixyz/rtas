use bevy::prelude::Entity;

pub struct UnitSize(pub f32);
#[derive(PartialEq, Eq, Hash)]
/// Useful for client to know which sprite to use
pub enum RenderSprite {
    Ogre,
    Goblin,
    Bandit,
}

pub struct OffensiveStats {
    pub power: f32,
}

pub struct MeleeAbility {
    pub range: f32,
    // additional range to account for units movement
    pub motion_buffer_range: f32,
    pub time_to_strike: f32,
    pub cooldown: f32,
}

// TODO: use a mod to encapsulate state and structures, so the naming and and their scope is cleaner.
pub enum MeleeAbilityState {
    Ready,
    WillAttack(MeleeAbilityStateWillAttack),
    AttackCooldown(MeleeAbilityStateCooldown),
    MotionBufferExceeded,
}
impl MeleeAbilityState {
    pub fn interrupt(&mut self) {
        match self {
            MeleeAbilityState::Ready => {}
            MeleeAbilityState::MotionBufferExceeded => {}
            MeleeAbilityState::WillAttack(_) => {
                *self = MeleeAbilityState::Ready;
            }
            MeleeAbilityState::AttackCooldown(_) => {}
        }
    }
}

pub struct MeleeAbilityStateWillAttack {
    pub start_time: f32,
    pub target_entity: Entity,
}
pub struct MeleeAbilityStateCooldown {
    pub start_time: f32,
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
    pub amount: Vec<f32>,
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
    pub chase_when_target_too_far: bool,
}
