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
        let (time, mut stats, action_comp, mut position_comp, equipment, map, mut gun_effects) = data;
        let dt = time.as_dt();

        for (a, p, e) in (&action_comp, &mut position_comp, &equipment).join() {
            if a.move_action.is_some_direction() {
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
            }
            if a.attack_action.is_some() {
                let point = a.attack_action.unwrap();
                info!("Ent attacking {:?}", point);
            }
        }
        info!("-> ActSys");
    }
}


