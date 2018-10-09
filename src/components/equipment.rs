use specs::VecStorage;

use crate::item::{Weapon, Item};

#[derive(Component)]
#[component(VecStorage)]
pub struct EquipmentComp {
    pub weapon: Weapon,
    pub items: Vec<Item>,
}

impl EquipmentComp {
    pub fn new(weapon: Weapon, items: Vec<Item>) -> Self {
        Self { weapon, items }
    }
}
