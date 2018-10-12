use specs::VecStorage;

use crate::map::MapPoint;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Move {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub dirty: bool,
}

impl Move {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            dirty: false,
        }
    }

    pub fn is_some_direction(&self) -> bool {
        self.up || self.down || self.left || self.right
    }
}

#[derive(Component)]
#[component(VecStorage)]
pub struct ActComp {
    pub move_action: Move,
    pub attack_action: Option<MapPoint>,
}

impl ActComp {
    pub fn new() -> Self {
        Self {
            move_action: Move::new(),
            attack_action: None,
        }
    }
}


