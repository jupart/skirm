use specs::Entity;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TileType {
    Wall,
    Ground,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub occupant: Option<Entity>,
    pub tile_type: Option<TileType>,
    pub glyph: char,
}

impl Tile {
    pub fn new(tile_type: Option<TileType>) -> Self {
        Self { occupant: None, tile_type, glyph: '#' }
    }

    pub fn has_occupant(&self) -> bool {
        self.occupant.is_some()
    }
}

