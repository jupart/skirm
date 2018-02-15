use specs::{VecStorage, World};

use std::collections::HashMap;

use rendering::RenderType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SoundType {
    Move,
}

#[derive(Component)]
#[component(VecStorage)]
pub struct SoundComp {
    pub sound_map: HashMap<SoundType, (&'static str, bool)>,
}

impl SoundComp {
    pub fn new(sounds: HashMap<SoundType, (&'static str, bool)>) -> SoundComp {
        SoundComp { sound_map: sounds }
    }
}

#[derive(Component)]
#[component(VecStorage)]
pub struct PositionComp {
    pub x: f32,
    pub y: f32,
}

impl PositionComp {
    pub fn new(x: f32, y: f32) -> PositionComp {
        PositionComp { x, y }
    }
}

pub enum Action {
    MoveTo {
        x: i32,
        y: i32,
    },
    Idle,
}

#[derive(Component)]
#[component(VecStorage)]
pub struct ActionComp {
    pub current_action: Action,
}

impl ActionComp {
    pub fn new() -> ActionComp {
        ActionComp { current_action: Action::Idle }
    }
}

#[derive(Component)]
#[component(VecStorage)]
pub struct RenderComp {
    pub render_type: RenderType,
}

pub fn register_components(world: &mut World) {
    world.register::<PositionComp>();
    world.register::<RenderComp>();
    world.register::<SoundComp>();
    world.register::<ActionComp>();
}
