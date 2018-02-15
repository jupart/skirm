use std::collections::HashMap;

use specs::{Index, World};

use item::Item;
use components::*;
use rendering::RenderType;

pub enum BodyPart {
    LeftEye,
    RightEye,
    Head,
    Neck,
    Torso,
    LeftArm,
    RightArm,
    LeftHip,
    RightHip,
    LeftLeg,
    RightLeg,
    LeftFoot,
    RightFoot,
}

pub struct SkirmerFactory;

impl SkirmerFactory {
    pub fn new() -> SkirmerFactory {
        SkirmerFactory{}
    }

    pub fn create_skirmer(
        &self,
        x: f32,
        y: f32,
        // equipment: HashMap<BodyPart, Vec<Item>>,
        world: &mut World
    ) -> Index {
        let id = world
            .create_entity()
            .with(PositionComp::new(x, y))
            .with(RenderComp { render_type: RenderType::Glyph { id: "@" } })
            .with(ActionComp::new())
            .build()
            .id();
        id
    }
}
