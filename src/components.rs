use specs::{VecStorage, World};
use pathfinding::dijkstra;

use std::collections::HashMap;
use std::time::Duration;

use item::Item;
use rendering::RenderType;
use skirmmap::{SkirmMap, MapPoint};

#[derive(Component)]
pub struct EquipmentComp {
    pub items: Vec<Item>,
}

impl EquipmentComp {
    pub fn new(items: Vec<Item>) -> EquipmentComp {
        EquipmentComp { items }
    }
}

#[derive(Component)]
pub struct StatsComp {
    pub health: u8,
    pub max_health: u8,
    pub strength: u8,
    pub aim: u8,
}

impl StatsComp {
    pub fn default() -> StatsComp {
        StatsComp {
            health: 100,
            max_health: 100,
            strength: 5,
            aim: 5,
        }
    }
}

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

#[derive(Eq, PartialEq)]
pub struct MoveToPoint {
    pub move_time: Duration,
    pub point_stack: Vec<MapPoint>,
}

impl MoveToPoint {
    pub fn new(
        current_pos: MapPoint,
        move_to: MapPoint,
        map: &SkirmMap
    ) -> Result<MoveToPoint, ()> {
        match dijkstra(&current_pos, |p| p.neighbors(map), |p| *p == move_to) {
            Some(points) => {
                Ok(MoveToPoint {
                    move_time: Duration::new(0, 0),
                    point_stack: points.0,
                })
            },
            None => Err(()),
        }
    }
}

pub enum Action {
    MoveTo(MoveToPoint),
    AttackAt(MapPoint),
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
    world.register::<StatsComp>();
    world.register::<EquipmentComp>();
}
