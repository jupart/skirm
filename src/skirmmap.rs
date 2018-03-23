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

    pub fn has_line_of_sight(&self, to: &MapPoint, map: &SkirmMap) -> bool {
        let tiles_to_check = map.get_tiles_between(self, to);
        let mut has_sight = true;
        for tile in tiles_to_check {
            if !map.has_ground_at(&tile) {
                has_sight = false;
            }
        }
        has_sight
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

    pub fn has_ground_at(&self, point: &MapPoint) -> bool {
        match self.map.get(point) {
            Some(tile) => {
                if tile.tile_type == TileType::Ground {
                    true
                } else {
                    false
                }
            },
            None => false,
        }
    }

    pub fn get_tiles_between(&self, p1: &MapPoint, p2: &MapPoint) -> Vec<MapPoint> {
        let x1 = p1.x as f32;
        let x2 = p2.x as f32;
        let y1 = p1.y as f32;
        let y2 = p2.y as f32;
        let mut tiles = Vec::new();

        // Build our tiles from the equation of the line passing
        // through `self` and `to`

        // Special case - slope is infinite
        if x2 == x1 {
            let direction = (y2 - y1).signum();
            let num_of_y_tiles = ((y2 - y1) / TILE_HEIGHT as f32).abs() as i32;
            for i in 0..(num_of_y_tiles + 1) {
                let y = direction * (i * TILE_HEIGHT) as f32 + y1;
                tiles.push(MapPoint::new(x1 as i32, y as i32));
            }

        // Regular equation of a line
        } else {
            let slope = (y2 - y1) / (x2 - x1);
            let b = y2 - slope * x2;

            // Should we use `x =` form or `y =`? If delta-x is greater, we use
            // `y=` and vice versa
            if ((x2 - x1) / TILE_WIDTH as f32).abs() >= ((y2 - y1) / TILE_HEIGHT as f32).abs() {
                let direction = (x2 - x1).signum();
                let num_of_x_tiles = ((x2 - x1) / TILE_WIDTH as f32).abs() as i32;
                tiles.push(*p1);
                for i in 1..(num_of_x_tiles + 1) {
                    let x = direction * (i * TILE_WIDTH) as f32 + x1;
                    let y = slope * x + b;
                    tiles.push(MapPoint::new(x as i32, y as i32));
                }
            } else {
                let direction = (y2 - y1).signum();
                let num_of_y_tiles = ((y2 - y1) / TILE_HEIGHT as f32).abs() as i32;
                tiles.push(*p1);
                for i in 1..(num_of_y_tiles + 1) {
                    let y = direction * (i * TILE_HEIGHT) as f32 + y1;
                    let x = (y - b) / slope;
                    tiles.push(MapPoint::new(x as i32, y as i32));
                }
            }
        }
        tiles
    }

    pub fn nearest_tile(&self, x: i32, y: i32) -> MapPoint {
        let rounded_x = (x / TILE_WIDTH) * TILE_WIDTH;
        let rounded_y = (y / TILE_HEIGHT) * TILE_HEIGHT;
        MapPoint { x: rounded_x, y: rounded_y}
    }
}
