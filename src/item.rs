use ggez::GameResult;
use serde_yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
}

pub struct Weapon {

}

pub struct Armor {

}

#[derive(Deserialize)]
pub struct WeaponRecipe {
    weapon_type: String,
    description: String,
    damage: u8,
    accuracy: u8,
    range: u8,
}

pub struct ItemFactory {
    weapons: HashMap<String, WeaponRecipe>,
}

impl ItemFactory {
    pub fn new() -> GameResult<ItemFactory> {

        // Weapons - open the file, read it into a buffer, deserialize with serde
        let mut weapon_file = File::open("./resources/weapons.yml")?;
        let mut buffer = String::new();
        weapon_file.read_to_string(&mut buffer)?;
        let weapons: HashMap<String, WeaponRecipe> = match serde_yaml::from_str(&buffer.as_str()) {
            Ok(result) => result,

            // TODO In the future we could have some builtin weapons that don't
            // require .yml definition and use them here.
            Err(e) => panic!("Error reading weapon.yml, format is corrupt. {:?}", e),
        };

        Ok(ItemFactory { weapons })
    }

    pub fn get_weapon(&self, name: &'static str) -> &WeaponRecipe {
        match self.weapons.get(name) {
            Some(recipe) => &recipe,
            _ => panic!("Error getting weapon named {}", name)
        }
    }
}
