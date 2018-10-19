use specs::VecStorage;
use ncollide2d::world::CollisionObjectHandle;

use crate::{Vector2};

#[derive(Clone, Debug, Component)]
#[component(VecStorage)]
pub struct PhysicsComp {
    pub gravity: bool,
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

impl PhysicsComp {
    pub fn new(gravity: bool, vel: Vector2) -> Self {
        Self {
            gravity,
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
