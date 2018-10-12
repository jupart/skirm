use specs::{Entity, Fetch, System, WriteStorage, Join};

use crate::{
    components::*,
    resources::DeltaTime,
    map::SkirmMap,
    item::Weapon,
};

// Performs entities' `current_action`s
pub struct ActSys;
impl ActSys {
    fn _apply_damage(&self, ent: Entity, _item: &Weapon, _distance: u16, stats: &mut WriteStorage<StatsComp>) {
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
        WriteStorage<'a, AnimComp>,
        Fetch<'a, SkirmMap>,
    );

    fn run(&mut self, (time, mut _stats, mut action, mut pos, mut anim, _map): Self::SystemData) {
        info!("<- ActSys");
        let dt = time.as_dt();

        for (a, p, n) in (&mut action, &mut pos, &mut anim).join() {
            if a.move_action.is_some_direction() {
                if a.move_action.dirty {
                    n.change_id(String::from("move"), true);
                    a.move_action.dirty = false;
                }

                let speed = 100.0;
                info!("Ent moving {:?}", a.move_action);
                if a.move_action.up {
                    p.y -= speed * dt;
                }
                if a.move_action.down {
                    p.y += speed * dt;
                }
                if a.move_action.left {
                    p.x -= speed * dt;
                }
                if a.move_action.right {
                    p.x += speed * dt;
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
        info!("-> ActSys");
    }
}


