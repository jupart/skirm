use specs::VecStorage;

use crate::{
    map::MapPoint,
    input::InputState,
};

#[derive(Component)]
#[component(VecStorage)]
pub struct StateComp {
    pub move_action: InputState,
    pub attack_action: Option<MapPoint>,
    pub on_ground: bool,
}

impl StateComp {
    pub fn new() -> Self {
        Self {
            move_action: InputState::new(),
            attack_action: None,
            on_ground: false,
        }
    }

    pub fn is_moving_vertical(&self) -> {
        self.move_action.up.state || self.move_action.down.state
    }

    pub fn is_moving_horizontal(&self) -> {
        self.move_action.left.state || self.move_action.right.state
    }

    pub fn is_moving(&self) -> bool {
        self.move_action.is_some_direction()
    }

    pub fn is_on_ground(&self) -> bool {
        self.on_ground
    }
}


