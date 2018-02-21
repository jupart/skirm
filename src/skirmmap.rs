use std::fs::File;
use std::path::Path;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use ggez::GameResult;
use ascii::{ToAsciiChar, AsciiChar};

pub const TILE_WIDTH: i32 = 8;
pub const TILE_HEIGHT: i32 = 14;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct MapPoint {
    pub x: i32,
    pub y: i32,
}

impl MapPoint {
    pub fn new(x: i32, y: i32) -> MapPoint {
        // Round x and y down to TILE_WIDTH and TILE_HEIGHT
        let rounded_x = (x / TILE_WIDTH) * TILE_WIDTH;
        let rounded_y = (y / TILE_HEIGHT) * TILE_HEIGHT;
        MapPoint { x: rounded_x, y: rounded_y }
    }

    fn offset(&self, point: (i32, i32)) -> MapPoint {
        MapPoint { x: self.x + point.0, y: self.y + point.1 }
    }

    pub fn neighbors(&self, map: &SkirmMap) -> Vec<(MapPoint, usize)> {
        let mut neighbors = Vec::new();
        let points_to_check = vec![
            (-TILE_WIDTH, -TILE_HEIGHT), (0, -TILE_HEIGHT), (TILE_WIDTH, -TILE_HEIGHT),
            (-TILE_WIDTH, 0),                               (TILE_WIDTH, 0),
            (-TILE_WIDTH, TILE_HEIGHT),  (0, TILE_HEIGHT),  (TILE_WIDTH, TILE_WIDTH)
        ];
        let mut i = -1;
        for to_check in points_to_check {
            i += 1;
            let next_point = self.offset(to_check);
            let neighbor = match map.map.get(&next_point) {
                Some(tile) => {
                    tile
                },
                None => {
                    continue
                },
            };

            if neighbor.tile_type == TileType::Ground {
                let weight: usize;
                if i == 0 || i == 2 || i == 5 || i == 7 {
                    weight = 2;
                } else {
                    weight = 1;
                }
                neighbors.push((next_point, weight));
            }
        }
        neighbors
    }
}

#[derive(PartialEq, Eq)]
pub enum TileType {
    Wall,
    Ground,
}

pub struct Tile {
    pub tile_type: TileType,
    pub glyph: &'static str,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Tile {
        match tile_type {
            TileType::Wall => {
                Tile { tile_type, glyph: "#" }
            },
            TileType::Ground => {
                Tile { tile_type, glyph: "" }
            }
        }
    }
}

pub struct SkirmMap {
    pub map: HashMap<MapPoint, Tile>,
}

impl SkirmMap {
    pub fn load<P>(path: P) -> GameResult<SkirmMap>
        where P: AsRef<Path> + Debug,
    {
        let map_file = File::open(path)?;
        let buffer = BufReader::new(map_file);

        let mut map = HashMap::new();
        for (j, line) in buffer.lines().enumerate() {
            for (i, c) in line.unwrap().chars().enumerate() {
                if c.to_ascii_char()
                    .expect("Map character c is valid ascii") == AsciiChar::Hash
                {
                    let x = (i as i32) * TILE_WIDTH;
                    let y = (j as i32) * TILE_HEIGHT;
                    map.insert(MapPoint::new(x, y),
                               Tile::new(TileType::Wall));
                }
                else {
                    let x = (i as i32) * TILE_WIDTH;
                    let y = (j as i32) * TILE_HEIGHT;
                    map.insert(MapPoint::new(x, y),
                               Tile::new(TileType::Ground));
                }
            }
        }

        Ok(SkirmMap { map })
    }
}
