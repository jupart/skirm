use specs::{Entity, World};

use crate::{
    item::{Weapon, Item, ItemFactory},
    components::*,
    rendering::RenderType,
    map::{SkirmMap, MapPoint, MapError},
};

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
        tile_x: i32,
        tile_y: i32,
        skirmer: &SkirmerType,
        item_factory: &ItemFactory,
        map: &mut SkirmMap,
        world: &mut World
    ) -> Result<Entity, MapError> {
        let (weapon, items) = self.get_skirmer_items(skirmer, item_factory);

        let tile_point = MapPoint::new(tile_x, tile_y);
        let (x, y) = tile_point.as_float_coord_tuple();
        let ent = world.create_entity()
            .with(PositionComp::new(x, y))
            .with(RenderComp { render_type: RenderType::Image { id: "green_box" } })
            .with(StatsComp::default())
            .with(ActComp::new())
            .with(EquipmentComp::new(weapon, items))
            .build();

        map.add_occupant(ent, tile_point).map(|()| ent)
    }

    fn get_skirmer_items(&self, skirmer: &SkirmerType, factory: &ItemFactory) -> (Weapon, Vec<Item>) {
        match skirmer {
            &SkirmerType::Fighter => {
                let weapon = factory.get_weapon(".22 Rifle");
                (weapon, vec![])
            },
            &SkirmerType::Sniper => {
                let weapon = factory.get_weapon(".22 Rifle");
                (weapon, vec![])
            }
        }
    }
}
