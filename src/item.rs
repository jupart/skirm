use ggez::GameResult;
use serde_yaml;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
}

#[derive(Clone, Deserialize)]
pub struct Weapon {
    weapon_type: String,
    description: String,
    damage: u8,
    accuracy: u8,
    range: u8,
    sound: String,
}

impl Weapon {
    pub fn attack() {

    }
}

pub struct Armor {

}

pub struct ItemFactory {
    weapons: HashMap<String, Weapon>,
}

impl ItemFactory {
    pub fn new() -> GameResult<Self> {

        // Weapons - open the file, read it into a buffer, deserialize with serde
        let mut weapon_file = File::open("./resources/weapons.yml")?;
        let mut buffer = String::new();
        weapon_file.read_to_string(&mut buffer)?;
        let weapons: HashMap<String, Weapon> = match serde_yaml::from_str(&buffer.as_str()) {
            Ok(result) => result,

            // TODO In the future we could have some builtin weapons that don't
            // require .yml definition and use them here.
            Err(e) => panic!("Error reading weapon.yml, format is corrupt. {:?}", e),
        };

        Ok(ItemFactory { weapons })
    }

    pub fn get_weapon(&self, name: &'static str) -> Item {
        match self.weapons.get(name) {
            Some(weapon) => Item::Weapon(weapon.clone()),
            _ => panic!("Error getting weapon named {}", name)
        }
    }
}
