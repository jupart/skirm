// use specs::{Fetch, FetchMut, System, ReadStorage, WriteStorage};

// use crate::{
//     components::*,
//     input::{SkirmerInput, PendingCommand},
//     map::{SkirmMap, MapPoint},
// };

// // An Input System that verifies and creates an entity's current_action
// pub struct PlanSys;

// impl<'a> System<'a> for PlanSys {
//     type SystemData = (
//         Fetch<'a, SkirmMap>,
//         FetchMut<'a, SkirmerInput>,
//         WriteStorage<'a, ActComp>,
//         ReadStorage<'a, PositionComp>,
//         ReadStorage<'a, StatsComp>,
//         WriteStorage<'a, TurnComp>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         info!("<- PlanSys");
//         let (skirmmap, mut input, mut action, position, stats, mut turn) = data;

//         let turn = turn.get_mut(input.ent).unwrap();

//         if input.pending_command.is_none()
//             || input.command_point.is_none()
//             || turn.phase == TurnPhase::Start
//             || turn.phase == TurnPhase::Finish
//         {
//             info!("-> PlanSys");
//             return;
//         }

//         let stats = stats.get(input.ent).unwrap();
//         let p = position.get(input.ent).unwrap();
//         let a = action.get_mut(input.ent).unwrap();

//         let pos = MapPoint::from_pixel_coord(p.x as i32, p.y as i32);
//         let to = input.command_point.map(|(x, y)| MapPoint::from_pixel_coord(x, y)).unwrap();
//         match input.pending_command.unwrap() {
//             PendingCommand::Move => {
//                 match MoveToPoint::new(pos, to, &*skirmmap) {
//                     Ok(move_to_point) => {
//                         if turn.try_update_move(&move_to_point, stats.move_per_turn).is_ok() {
//                             turn.increment();
//                             a.current_action = Action::MoveTo(move_to_point);
//                         } else {
//                             a.current_action = Action::Idle;
//                         }
//                     },
//                     Err(()) => {
//                         a.current_action = Action::Idle;
//                     },
//                 }
//             },
//             PendingCommand::Attack => {
//                 if skirmmap.has_line_of_sight(&pos, &to) {
//                     a.current_action = Action::AttackAt(to);
//                     turn.increment();
//                 } else {
//                     a.current_action = Action::Idle;
//                 }
//             }
//         }
//         input.pending_command = None;
//         input.command_point = None;
//         info!("-> PlanSys");
//     }
// }

