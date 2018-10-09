use specs::Entity;

pub struct SkirmerInput {
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub ent: Entity,
}

impl SkirmerInput {
    pub fn new(ent: Entity) -> Self {
        Self {
            up: false,
            left: false,
            right: false,
            down: false,
            ent,
        }
    }
}
