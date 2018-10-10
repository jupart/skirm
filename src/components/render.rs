use std::time::Duration;
use specs::VecStorage;
use ggez::graphics::Color;

pub const WHITE: Color = Color { r: 0.921, g: 0.859, b: 0.698, a: 1.0};
pub const BLACK: Color = Color { r: 0.157, g: 0.157, b: 0.157, a: 1.0 };

#[derive(Component)]
#[component(VecStorage)]
pub struct AnimComp {
    pub frames: Vec<&'static str>,
    pub frame_num: u32,
    pub delay: Duration,
    pub current_time: Duration,
}

impl AnimComp {
    pub fn increment_frame(&mut self) -> &'static str {
        self.frame_num += 1;
        if self.frame_num > self.frames.len() as u32 {
            self.frame_num = 0;
        }

        self.frames.get(self.frame_num as usize).unwrap()
    }
}

#[derive(Component)]
#[component(VecStorage)]
pub struct SpriteComp {
    pub id: &'static str,
}
