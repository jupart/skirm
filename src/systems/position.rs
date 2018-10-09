use specs::{System, WriteStorage, Fetch};

use crate::{
    components::PositionComp,
    input::SkirmerInput,
    resources::DeltaTime,
};

pub struct PositionSys;
impl<'a> System<'a> for PositionSys {
    type SystemData = (
        WriteStorage<'a, PositionComp>,
        Fetch<'a, SkirmerInput>,
        Fetch<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        info!("<- PositionSys");
        let (mut pos, input, delta) = data;
        let dt = delta.as_dt();
        let position = pos.get_mut(input.ent).unwrap();

        let speed = 100.0;
        if input.up {
            position.y -= speed * dt;
        }
        if input.down {
            position.y += speed * dt;
        }
        if input.left {
            position.x -= speed * dt;
        }
        if input.right {
            position.x += speed * dt;
        }
        info!("-> PositionSys");
    }
}
