use specs::Entity;

use item::Item;

#[derive(PartialEq, Eq)]
pub enum TileType {
    Wall,
    Ground,
}

pub struct Tile {
    pub occupant: Option<Entity>,
    pub ground_occupant: Option<Item>,
    pub tile_type: TileType,
    pub glyph: char,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        match tile_type {
            TileType::Wall => {
                Self { occupant: None, ground_occupant: None, tile_type, glyph: '#' }
            },
            TileType::Ground => {
                Self { occupant: None, ground_occupant: None, tile_type, glyph: ' ' }
            }
        }
    }

    pub fn has_occupant(&self) -> bool {
        self.occupant.is_some()
    }
}

