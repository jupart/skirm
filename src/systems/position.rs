use specs::{System, WriteStorage};

use crate::{
    components::PositionComp,
};

pub struct PositionSys;
impl<'a> System<'a> for PositionSys {
    type SystemData = (
        WriteStorage<'a, PositionComp>,
    );

    fn run(&mut self, _data: Self::SystemData) {
        info!("<- PositionSys");
        info!("-> PositionSys");
    }
}
