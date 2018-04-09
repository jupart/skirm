use specs::{Index, World};

use item::{Weapon, Item, ItemFactory};
use components::*;
use rendering::RenderType;
use skirmmap::{SkirmMap, MapPoint, MapError};

pub enum SkirmerType {
    Fighter,
    Sniper,
}

pub struct SkirmerFactory;

impl SkirmerFactory {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn create_skirmer(
        &self,
        x: f32,
        y: f32,
        skirmer: &SkirmerType,
        item_factory: &ItemFactory,
        map: &mut SkirmMap,
        world: &mut World
    ) -> Result<Index, MapError> {
        let (weapon, items) = self.get_skirmer_items(skirmer, item_factory);

        let ent = world.create_entity()
            .with(PositionComp::new(x, y))
            .with(RenderComp { render_type: RenderType::Glyph { id: "@" } })
            .with(StatsComp::default())
            .with(ActionComp::new())
            .with(EquipmentComp::new(weapon, items))
            .build();

        map.add_occupant(ent, MapPoint::new(x as i32, y as i32)).map(|()| ent.id())
    }

    fn get_skirmer_items(&self, skirmer: &SkirmerType, factory: &ItemFactory) -> (Weapon, Vec<Item>) {
        match skirmer {
            SkirmerType::Fighter => {
                let weapon = factory.get_weapon(".22 Rifle");
                (weapon, vec![])
            },
            SkirmerType::Sniper => {
                let weapon = factory.get_weapon(".22 Rifle");
                (weapon, vec![])
            }
        }
    }
}
