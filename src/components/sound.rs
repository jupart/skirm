use specs::VecStorage;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SoundType {
    Move,
}

#[derive(Component)]
#[component(VecStorage)]
pub struct SoundComp {
    pub sound_map: HashMap<SoundType, (&'static str, bool)>,
}

impl SoundComp {
    pub fn new(sounds: HashMap<SoundType, (&'static str, bool)>) -> SoundComp {
        SoundComp { sound_map: sounds }
    }
}
