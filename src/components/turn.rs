use specs::VecStorage;

use crate::components::act::MoveToPoint;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TurnPhase {
    Start,
    FirstAction,
    SecondAction,
    BonusAction,
    Finish,
}

#[derive(Component)]
#[component(VecStorage)]
pub struct TurnComp {
    pub current_turn: bool,
    pub has_moved: u8,
    pub phase: TurnPhase,
}

impl TurnComp {
    pub fn default() -> Self {
        Self { current_turn: false, has_moved: 0, phase: TurnPhase::FirstAction }
    }

    pub fn increment(&mut self) {
        self.phase = match self.phase {
            TurnPhase::Start => TurnPhase::FirstAction,
            TurnPhase::FirstAction => TurnPhase::SecondAction,
            TurnPhase::SecondAction => TurnPhase::BonusAction,
            TurnPhase::BonusAction => TurnPhase::Finish,
            TurnPhase::Finish => TurnPhase::FirstAction,
        };
    }

    pub fn try_update_move(&mut self, mtp: &MoveToPoint, max_move: u8) -> Result<(), ()> {
        let number_of_moves = mtp.point_stack.len();
        let to_move = self.has_moved + number_of_moves as u8;
        if to_move <= max_move {
            self.has_moved = to_move;
            Ok(())
        } else {
            Err(())
        }
    }
}
