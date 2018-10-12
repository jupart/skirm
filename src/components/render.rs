use std::time::Duration;
use specs::VecStorage;
use ggez::graphics::Color;

pub const WHITE: Color = Color { r: 0.921, g: 0.859, b: 0.698, a: 1.0};
pub const BLACK: Color = Color { r: 0.157, g: 0.157, b: 0.157, a: 1.0 };

#[derive(Component)]
#[component(VecStorage)]
pub struct AnimComp {
    pub id: String,
    pub frame_num: u32,
    pub delay: Duration,
    pub current_time: Duration,
    pub repeat: bool,
    pub dirty: bool,
}

impl AnimComp {
    pub fn new(id: String, repeat: bool) -> Self {
        Self {
            id,
            frame_num: 0,
            delay: Duration::from_millis(250),
            current_time: Duration::new(0, 0),
            repeat,
            dirty: true,
        }
    }

    pub fn reset_time(&mut self) {
        self.current_time = Duration::new(0, 0);
    }

    pub fn change_id(&mut self, id: String, repeat: bool) {
        info!("Ent changed animation id to {}", id);
        self.id = id;
        self.frame_num = 0;
        self.delay = Duration::from_millis(250);
        self.current_time = Duration::new(0, 0);
        self.repeat = repeat;
        self.dirty = true;
    }
}

#[derive(Component)]
#[component(VecStorage)]
pub struct SpriteComp {
    pub id: String,
}

impl SpriteComp {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
