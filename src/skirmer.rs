use specs::{Entity, World};

use item::{Item, ItemFactory};
use components::*;
use rendering::RenderType;

pub enum SkirmerType {
    Fighter,
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
        skirmer: SkirmerType,
        item_factory: &ItemFactory,
        world: &mut World
    ) -> Entity {
        let equipment = self.get_skirmer_items(skirmer, item_factory);
        let ent = world
            .create_entity()
            .with(PositionComp::new(x, y))
            .with(RenderComp { render_type: RenderType::Glyph { id: "@" } })
            .with(StatsComp::default())
            .with(ActionComp::new())
            .with(EquipmentComp::new(equipment))
            .build();
        ent
    }

    fn get_skirmer_items(&self, skirmer: SkirmerType, factory: &ItemFactory) -> Vec<Item> {
        match skirmer {
            SkirmerType::Fighter => {
                let weapon = factory.get_weapon(".22 Rifle");
                vec![weapon]
            }
        }
    }
}
