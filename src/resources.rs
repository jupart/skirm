use specs::Index;

use std::time::Duration;

pub struct DeltaTime {
    pub delta: Duration,
}

pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub to_move: bool,
    pub move_x: i32,
    pub move_y: i32,
    pub id: Index,
}

impl PlayerInput {
    pub fn new(id: Index) -> PlayerInput {
        PlayerInput {
            up: false,
            down: false,
            left: false,
            right: false,
            to_move: false,
            move_x: 0,
            move_y: 0,
            id,
        }
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.to_move = true;
        self.move_x = x;
        self.move_y = y;
    }
}
