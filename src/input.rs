use specs::Index;

use skirmmap::MapPoint;

#[derive(Clone, Copy)]
pub enum PendingCommand {
    Move,
    Attack,
}

pub struct PlayerInput {
    pub pending_command: Option<PendingCommand>,
    pub command_point: Option<MapPoint>,
    pub id: Index,
}

impl PlayerInput {
    pub fn new(id: Index) -> PlayerInput {
        PlayerInput {
            pending_command: None,
            command_point: None,
            id,
        }
    }

    pub fn select_point(&mut self, x: i32, y: i32) {
        self.command_point = Some(MapPoint::new(x, y));
    }

    pub fn set_pending_command(&mut self, cmd: PendingCommand) {
        self.pending_command = Some(cmd);
        self.command_point = None;
    }
}
