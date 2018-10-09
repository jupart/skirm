use specs::{Fetch, System, WriteStorage};

use crate::components::*;
use crate::asset_storage::AssetStorage;

pub struct SoundSys;
impl<'a> System<'a> for SoundSys {
    type SystemData = (
        Fetch<'a, AssetStorage>,
        WriteStorage<'a, SoundComp>,
    );

    fn run(&mut self, _data: Self::SystemData) {
        info!("<- SoundSys");
        info!("-> SoundSys");
    }
}

