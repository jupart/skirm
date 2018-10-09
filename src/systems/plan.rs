use specs::{Fetch, FetchMut, System, ReadStorage, WriteStorage};

use crate::{components::*, input::SkirmerInput};

// An Input System that verifies and creates an entity's current_action
pub struct PlanSys;

impl<'a> System<'a> for PlanSys {
    type SystemData = (
        Fetch<'a, SkirmerInput>,
        WriteStorage<'a, ActComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        info!("<- PlanSys");

        let (input, mut act) = data;
        let act_comp = act.get_mut(input.ent).unwrap();

        act_comp.move_action.up = input.up;
        act_comp.move_action.down = input.down;
        act_comp.move_action.left = input.left;
        act_comp.move_action.right = input.right;

        info!("-> PlanSys");
    }
}

