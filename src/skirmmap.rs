use std::fs::File;
use std::path::Path;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use specs::Entity;
use ggez::{GameResult, GameError};
use ascii::{ToAsciiChar, AsciiChar};
use pathfinding::dijkstra;

use item::Item;

pub const TILE_WIDTH: i32 = 8;
pub const TILE_HEIGHT: i32 = 14;

#[derive(PartialEq, Eq, Debug)]
pub enum MapError {
    Occupied,
    PointDoesNotExist,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct MapPoint {
    pub x: i32,
    pub y: i32,
}

impl MapPoint {
    pub fn new(x: i32, y: i32) -> Self {
        // Round x and y down to TILE_WIDTH and TILE_HEIGHT
        let rounded_x = (x / TILE_WIDTH) * TILE_WIDTH;
        let rounded_y = (y / TILE_HEIGHT) * TILE_HEIGHT;
        Self { x: rounded_x, y: rounded_y }
    }

    fn offset(&self, point: (i32, i32)) -> Self {
        MapPoint { x: self.x + point.0, y: self.y + point.1 }
    }

    pub fn neighbors(&self, map: &SkirmMap) -> Vec<(Self, usize)> {
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
                let weight = if i == 0 || i == 2 || i == 5 || i == 7 {
                    2
                } else {
                    1
                };
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
    pub occupant: Option<Entity>,
    pub ground_occupant: Option<Item>,
    pub tile_type: TileType,
    pub glyph: &'static str,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        match tile_type {
            TileType::Wall => {
                Self { occupant: None, ground_occupant: None, tile_type, glyph: "#" }
            },
            TileType::Ground => {
                Self { occupant: None, ground_occupant: None, tile_type, glyph: "" }
            }
        }
    }

    pub fn has_occupant(&self) -> bool {
        self.occupant.is_some()
    }
}

pub struct SkirmMap {
    pub map: HashMap<MapPoint, Tile>,
}

impl SkirmMap {
    pub fn load<P>(path: P) -> GameResult<Self>
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

        Ok(Self { map })
    }

    pub fn has_ground_at(&self, point: &MapPoint) -> bool {
        match self.map.get(point) {
            Some(tile) => {
                tile.tile_type == TileType::Ground
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
        if p1.x == p2.x {
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

    pub fn pathfind(&self, p1: &MapPoint, p2: &MapPoint) -> Option<Vec<MapPoint>> {
        match dijkstra(p1, |p| p.neighbors(self), |p| *p == *p2) {
            Some(points) => Some(points.0),
            None => None,
        }
    }

    pub fn nearest_tile(&self, x: i32, y: i32) -> MapPoint {
        let rounded_x = (x / TILE_WIDTH) * TILE_WIDTH;
        let rounded_y = (y / TILE_HEIGHT) * TILE_HEIGHT;
        MapPoint { x: rounded_x, y: rounded_y}
    }

    pub fn has_line_of_sight(&self, p1: &MapPoint, p2: &MapPoint) -> bool {
        let tiles_to_check = self.get_tiles_between(p1, p2);
        let mut has_sight = true;
        for tile in tiles_to_check {
            if !self.has_ground_at(&tile) {
                has_sight = false;
            }
        }
        has_sight
    }

    pub fn add_occupant(&mut self, ent: Entity, point: MapPoint) -> Result<(), MapError> {
        let result;
        if self.has_occupant(&point) {
            result = Err(MapError::Occupied)
        } else {
            result = match self.map.get_mut(&point) {
                Some(tile) => {
                    tile.occupant = Some(ent);
                    Ok(())
                },
                None => Err(MapError::PointDoesNotExist)
            }
        }

        result
    }

    pub fn has_occupant(&self, point: &MapPoint) -> bool {
        self.map.get(point).is_some()
    }
}

pub fn tile_distance(p1: &MapPoint, p2: MapPoint) -> u16 {
    (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64).sqrt() as u16
}
