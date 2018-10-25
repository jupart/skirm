use specs::{Entity, Fetch, System, WriteStorage, Join};

use crate::{
    Vector2,
    components::*,
    resources::DeltaTime,
    map::SkirmMap,
    item::Weapon,
};

// Performs entities' `current_action`s
pub struct StateSys;
impl StateSys {
    fn _apply_damage(&self, ent: Entity, _item: &Weapon, _distance: u16, stats: &mut WriteStorage<StatsComp>) {
        let ent_stats = stats.get_mut(ent).unwrap();
        if ent_stats.health < 50 {
            ent_stats.health = 0;
        } else {
            ent_stats.health -= 50;
        }
    }
}

impl<'a> System<'a> for StateSys {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        WriteStorage<'a, StatsComp>,
        WriteStorage<'a, StateComp>,
        WriteStorage<'a, PositionComp>,
        WriteStorage<'a, AnimComp>,
        WriteStorage<'a, PhysicsComp>,
        Fetch<'a, SkirmMap>,
    );

    fn run(&mut self, (time, mut _stats, mut action, mut pos, mut anim, mut physics, _map): Self::SystemData) {
        info!("<- StateSys");
        let dt = time.as_dt();

        for (a, _p, n, y) in (&mut action, &mut pos, &mut anim, &mut physics).join() {
            if a.move_action.is_any_unhandled() {
            }

            if a.is_moving() {
                info!("Ent moving {:?}", a.move_action);
                let speed = 100.0 * dt;
                if a.move_action.up.state {
                    y.velocity = Vector2::new(0.0, -speed);
                }
                if a.move_action.down.state {
                    y.velocity = Vector2::new(0.0, speed);
                }
                if a.move_action.left.state {
                    y.velocity = Vector2::new(-speed, 0.0);
                }
                if a.move_action.right.state {
                    y.velocity = Vector2::new(speed, 0.0);
                }
            } else {
                if a.move_action.dirty {
                    n.change_id(String::from("idle"), true);
                    a.move_action.dirty = false;
                }
            }
            if a.attack_action.is_some() {
                let point = a.attack_action.unwrap();
                info!("Ent attacking {:?}", point);
            }
        }
        info!("-> StateSys");
    }
}


