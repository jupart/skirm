use specs::{VecStorage, World};

use std::collections::HashMap;
use std::time::Duration;

use item::{Weapon, Item};
use rendering::RenderType;
use map::{SkirmMap, MapPoint};

#[derive(Component)]
pub struct EquipmentComp {
    pub weapon: Weapon,
    pub items: Vec<Item>,
}

impl EquipmentComp {
    pub fn new(weapon: Weapon, items: Vec<Item>) -> Self {
        Self { weapon, items }
    }
}

#[derive(Component)]
pub struct StatsComp {
    pub health: u8,
    pub max_health: u8,
    pub strength: u8,
    pub aim: u8,
    pub move_per_turn: u8,
}

impl StatsComp {
    pub fn default() -> Self {
        Self {
            health: 100,
            max_health: 100,
            strength: 5,
            aim: 5,
            move_per_turn: 5,
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
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
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
    ) -> Result<Self, ()> {
        match map.pathfind(&current_pos, &move_to) {
            Some(points) => {
                Ok(Self {
                    move_time: Duration::new(0, 0),
                    point_stack: points,
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
    pub fn new() -> Self {
        Self { current_action: Action::Idle }
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
