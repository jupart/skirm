use specs::{Fetch, System, WriteStorage};

use crate::{components::*, input::SkirmerInput};

// An Input System that verifies and creates an entity's current action
pub struct PlanSys;

impl PlanSys {
    fn handle_input(&self) {
    }
}

impl<'a> System<'a> for PlanSys {
    type SystemData = (
        Fetch<'a, SkirmerInput>,
        WriteStorage<'a, StateComp>,
    );

    fn run(&mut self, (input, mut act): Self::SystemData) {
        info!("<- PlanSys");

        let act_comp = act.get_mut(input.ent).unwrap();

        let mv = &mut act_comp.move_action;
        if input.up != mv.up
            || input.down != mv.down
            || input.left != mv.left
            || input.right != mv.right
        {
            mv.dirty = true;
        }

        mv.up = input.up;
        mv.down = input.down;
        mv.left = input.left;
        mv.right = input.right;

        info!("-> PlanSys");
    }
}

