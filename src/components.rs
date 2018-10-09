use specs::World;

mod act;
mod equipment;
mod position;
mod render;
mod sound;
mod stats;
mod turn;

pub use self::{
    act::{ActComp, MoveToPoint, Action},
    equipment::EquipmentComp,
    position::PositionComp,
    render::RenderComp,
    sound::{SoundType, SoundComp},
    stats::StatsComp,
    turn::{TurnPhase, TurnComp},
};

pub fn register_components(world: &mut World) {
    world.register::<PositionComp>();
    world.register::<RenderComp>();
    world.register::<SoundComp>();
    world.register::<ActComp>();
    world.register::<StatsComp>();
    world.register::<EquipmentComp>();
    world.register::<TurnComp>();
}
