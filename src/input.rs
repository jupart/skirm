use specs::Entity;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Input {
    pub handled: bool,
    pub state: bool,
}

impl Input {
    pub fn new() -> Self {
        Self { handled: true, state: false }
    }

    pub fn set(&mut self, state: bool) {
        self.handled = false;
        self.state = state;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct InputState {
    pub up: Input,
    pub left: Input,
    pub right: Input,
    pub down: Input,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            up: Input::new(),
            left: Input::new(),
            right: Input::new(),
            down: Input::new(),
        }
    }

    pub fn is_any_unhandled(&self) -> bool {
        !self.up.handled || !self.down.handled || !self.left.handled || !self.right.handled
    }

    pub fn is_some_direction(&self) -> bool {
        self.up.state || self.down.state || self.left.state || self.right.state
    }
}

pub struct PlayerInputState {
    pub input: InputState,
    pub ent: Entity,
}

impl PlayerInputState {
    pub fn new(ent: Entity) -> Self {
        Self {
            input: InputState::new(),
            ent,
        }
    }
}
