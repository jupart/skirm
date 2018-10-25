use specs::World;

mod state;
mod equipment;
mod physics;
mod position;
mod render;
mod sound;
mod stats;
// mod turn;

pub use self::{
    state::StateComp,
    equipment::EquipmentComp,
    position::PositionComp,
    render::{SpriteComp, AnimComp, WHITE, BLACK},
    sound::{SoundType, SoundComp},
    stats::StatsComp,
    physics::{PhysicsComp, CollideComp, PhysicsType},
    // turn::{TurnPhase, TurnComp},
};

pub fn register_components(world: &mut World) {
    world.register::<PhysicsComp>();
    world.register::<CollideComp>();
    world.register::<PositionComp>();
    world.register::<AnimComp>();
    world.register::<SpriteComp>();
    world.register::<SoundComp>();
    world.register::<StateComp>();
    world.register::<StatsComp>();
    world.register::<EquipmentComp>();
    // world.register::<TurnComp>();
}
