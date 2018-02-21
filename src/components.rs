use specs::{VecStorage, World};
use pathfinding::dijkstra;

use std::collections::HashMap;
use std::time::Duration;

use rendering::RenderType;
use skirmmap::{SkirmMap, MapPoint};

#[derive(Component)]
pub struct StatsComp {

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
                println!("{:?}", points.0);
                Ok(MoveToPoint {
                    move_time: Duration::new(0, 0),
                    point_stack: points.0,
                })
            },
            None => {
                println!("Can't move to {:?}", move_to);
                Err(())
            },
        }
    }
}

pub enum Action {
    MoveTo(MoveToPoint),
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
}
