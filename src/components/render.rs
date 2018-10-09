use specs::VecStorage;

use crate::rendering::RenderType;

#[derive(Component)]
#[component(VecStorage)]
pub struct RenderComp {
    pub render_type: RenderType,
}
