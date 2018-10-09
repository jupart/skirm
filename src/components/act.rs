use specs::VecStorage;

use std::time::Duration;

use crate::map::{SkirmMap, MapPoint};

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
pub struct ActComp {
    pub current_action: Action,
}

impl ActComp {
    pub fn new() -> Self {
        Self { current_action: Action::Idle }
    }
}


