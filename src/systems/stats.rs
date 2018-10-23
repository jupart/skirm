use specs::{Entities, System, WriteStorage, Join};

use crate::components::StatsComp;

pub struct StatsSys;
impl<'a> System<'a> for StatsSys {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, StatsComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        info!("<- StatsSys");
        // let (entities, stats) = data;
        // for ent in entities.join() {
        //     let ent_stat = stats.get(ent).unwrap();
        //     if ent_stat.health == 0 {
        //         match entities.delete(ent) {
        //             Err(_e) => (),
        //             _ => (),
        //         }
        //     }
        // }
        info!("-> StatsSys");
    }
}
