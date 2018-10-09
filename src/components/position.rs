use specs::VecStorage;

#[derive(Component)]
#[component(VecStorage)]
pub struct PositionComp {
    pub x: f32,
    pub y: f32,
}

impl PositionComp {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
