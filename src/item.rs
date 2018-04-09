use ggez::GameResult;
use ron;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub enum Item {
}

#[derive(Clone, Deserialize)]
pub struct Weapon {
    pub weapon_type: String,
    pub description: String,
    pub damage: u8,
    pub accuracy: u8,
    pub range: u8,
    pub sound: String,
}

impl Weapon {
    pub fn attack(&self, distance: u8) -> u8 {
        self.damage
    }
}

pub struct ItemFactory {
    weapons: HashMap<String, Weapon>,
}

impl ItemFactory {
    pub fn new() -> GameResult<Self> {

        // Weapons - open the file, read it into a buffer, deserialize with serde
        let mut weapon_file = File::open("./resources/weapons.ron")?;
        let mut buffer = String::new();
        weapon_file.read_to_string(&mut buffer)?;
        let weapons: HashMap<String, Weapon> = match ron::de::from_str(buffer.as_str()) {
            Ok(result) => result,

            // TODO In the future we could have some builtin weapons that don't
            // require .ron definition and use them here.
            Err(e) => panic!("Error reading weapon.ron, format is corrupt. {:?}", e),
        };

        Ok(ItemFactory { weapons })
    }

    pub fn get_weapon(&self, name: &'static str) -> Weapon {
        match self.weapons.get(name) {
            Some(weapon) => weapon.clone(),
            _ => panic!("Error getting weapon named {}", name)
        }
    }
}
