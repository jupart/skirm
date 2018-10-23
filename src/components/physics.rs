use specs::VecStorage;
use ncollide2d::world::CollisionObjectHandle;

use crate::{Vector2};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhysicsType {
    Moveable,
    Constant,
}

#[derive(Clone, Debug, Component)]
#[component(VecStorage)]
pub struct PhysicsComp {
    pub physics_type: PhysicsType,
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

impl PhysicsComp {
    pub fn new(physics_type: PhysicsType, vel: Vector2) -> Self {
        Self {
            physics_type,
            velocity: vel,
            acceleration: nalgebra::zero(),
        }
    }
}

#[derive(Clone, Debug, Component)]
#[component(VecStorage)]
pub struct CollideComp {
    pub handle: CollisionObjectHandle,
}
