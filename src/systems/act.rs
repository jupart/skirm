use specs::{Entity, Fetch, FetchMut, System, ReadStorage, WriteStorage, Join};

use crate::{
    components::*,
    resources::DeltaTime,
    map::{SkirmMap, MapPoint, tile_distance},
    visual_effects::{GunshotEffect, GunshotEffects},
    item::{Weapon},
};

// Performs entities' `current_action`s
pub struct ActSys;
impl ActSys {
    fn position_close_to(&self, x1: f32, x2: f32) -> bool {
        let fluff = 1.0;
        (x2 - fluff <= x1)
            && (x1 <= x2 + fluff)
    }

    fn handle_move(&self, mtp: &mut MoveToPoint, pos: &mut PositionComp, dt: f32) -> Option<Action> {
        let mut change_to = None;

        let (x, y) = {
            let points_iter = mtp.point_stack.get_mut(0).unwrap();
            points_iter.as_float_coord_tuple()
        };
        let speed = 50.0;

        if self.position_close_to(pos.x, x)
        && self.position_close_to(pos.y, y) {
            pos.x = x;
            pos.y = y;
            mtp.point_stack.remove(0);
            if mtp.point_stack.is_empty() {
                change_to = Some(Action::Idle);
            }
        } else {
            let vec = (pos.x - x, pos.y - y);
            let mag = (vec.0.powf(2.0) + vec.1.powf(2.0)).sqrt();
            let unit = (vec.0 / mag, vec.1 / mag);
            let move_vec = (unit.0 * speed * dt, unit.1 * speed * dt);
            pos.x -= move_vec.0;
            pos.y -= move_vec.1;
        }

        change_to
    }

    fn handle_attack(&self, from: &MapPoint, to: &MapPoint, equipment: &EquipmentComp, map: &SkirmMap, effects: &mut GunshotEffects, stats: &mut WriteStorage<StatsComp>) -> Option<Action> {
        if map.has_occupant(to) {
            effects.effects.push(GunshotEffect::new(from.clone(), to.clone()));

            // play attack sound
            self.apply_damage(map.get_occupant(to).unwrap(), &equipment.weapon, tile_distance(from, *to), stats);
        }
        Some(Action::Idle)
    }

    fn apply_damage(&self, ent: Entity, _item: &Weapon, _distance: u16, stats: &mut WriteStorage<StatsComp>) {
        let ent_stats = stats.get_mut(ent).unwrap();
        if ent_stats.health < 50 {
            ent_stats.health = 0;
        } else {
            ent_stats.health -= 50;
        }
    }
}

impl<'a> System<'a> for ActSys {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        WriteStorage<'a, StatsComp>,
        WriteStorage<'a, ActComp>,
        WriteStorage<'a, PositionComp>,
        ReadStorage<'a, EquipmentComp>,
        Fetch<'a, SkirmMap>,
        FetchMut<'a, GunshotEffects>,
    );

    fn run(&mut self, data: Self::SystemData) {
        info!("<- ActSys");
        let (time, mut stats, mut action_comp, mut position_comp, equipment, map, mut gun_effects) = data;
        let dt = time.delta.as_secs() as f32 + time.delta.subsec_nanos() as f32 * 1e-9;

        for (a, p, e) in (&mut action_comp, &mut position_comp, &equipment).join() {
            let change_to = match a.current_action {
                Action::MoveTo(ref mut move_to_point) => self.handle_move(move_to_point, p, dt),
                Action::AttackAt(point) => {
                    self.handle_attack(&MapPoint::from_pixel_coord(p.x as i32, p.y as i32), &point, e, &map, &mut gun_effects, &mut stats)
                },
                Action::Idle => None,
            };

            if change_to.is_some() {
                a.current_action = change_to.unwrap();
            }
        }
        info!("-> ActSys");
    }
}


