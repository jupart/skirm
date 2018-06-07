use specs::Entity;

#[derive(Clone, Copy)]
pub enum PendingCommand {
    Move,
    Attack,
}

pub struct SkirmerInput {
    pub pending_command: Option<PendingCommand>,
    pub command_point: Option<(i32, i32)>,
    pub ent: Entity,
}

impl SkirmerInput {
    pub fn new(ent: Entity) -> Self {
        Self {
            pending_command: None,
            command_point: None,
            ent,
        }
    }

    pub fn select_point(&mut self, x: i32, y: i32) {
        self.command_point = Some((x, y));
    }

    pub fn set_pending_command(&mut self, cmd: PendingCommand) {
        self.pending_command = Some(cmd);
        self.command_point = None;
    }

    pub fn clear_pending_command(&mut self) {
        self.pending_command = None;
        self.command_point = None;
    }
}
