use specs::{VecStorage, World};

use std::collections::HashMap;
use std::time::Duration;

use crate::{
    item::{Weapon, Item},
    rendering::RenderType,
    map::{SkirmMap, MapPoint},
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TurnPhase {
    Start,
    FirstAction,
    SecondAction,
    BonusAction,
    Finish,
}

#[derive(Component)]
pub struct TurnComp {
    pub current_turn: bool,
    pub has_moved: u8,
    pub phase: TurnPhase,
}

impl TurnComp {
    pub fn default() -> Self {
        Self { current_turn: false, has_moved: 0, phase: TurnPhase::FirstAction }
    }

    pub fn increment(&mut self) {
        self.phase = match self.phase {
            TurnPhase::Start => TurnPhase::FirstAction,
            TurnPhase::FirstAction => TurnPhase::SecondAction,
            TurnPhase::SecondAction => TurnPhase::BonusAction,
            TurnPhase::BonusAction => TurnPhase::Finish,
            TurnPhase::Finish => TurnPhase::FirstAction,
        };
    }

    pub fn try_update_move(&mut self, mtp: &MoveToPoint, max_move: u8) -> Result<(), ()> {
        let number_of_moves = mtp.point_stack.len();
        let to_move = self.has_moved + number_of_moves as u8;
        if to_move <= max_move {
            self.has_moved = to_move;
            Ok(())
        } else {
            Err(())
        }
    }
}

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
            move_per_turn: 7,
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
    world.register::<TurnComp>();
}
