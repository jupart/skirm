use specs::{Fetch, System, WriteStorage};

use crate::{
    components::*,
    input::PlayerInputState,
};

// An Input System that verifies and creates an entity's current action
pub struct PlanSys;

impl PlanSys {
}

impl<'a> System<'a> for PlanSys {
    type SystemData = (
        Fetch<'a, PlayerInputState>,
        WriteStorage<'a, StateComp>,
    );

    fn run(&mut self, (player_input, mut act): Self::SystemData) {
        info!("<- PlanSys");

        let act_comp = act.get_mut(player_input.ent).unwrap();
        act_comp.move_action = player_input.input;

        info!("-> PlanSys");
    }
}

