use std::fs::File;
use std::path::Path;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use specs::Entity;
use ggez::{GameResult};
use ggez::graphics::Point2;
use ascii::{ToAsciiChar, AsciiChar};
use pathfinding::dijkstra;
use line_drawing;

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
        Self { x, y }
    }

    pub fn as_float_coord_tuple(&self) -> (f32, f32) {
        ((self.x * TILE_WIDTH) as f32, (self.y * TILE_HEIGHT) as f32)
    }

    pub fn from_pixel_coord(x: i32, y: i32) -> Self {
        let tile_index_x = x / TILE_WIDTH;
        let tile_index_y = y / TILE_HEIGHT;
        MapPoint::new(tile_index_x, tile_index_y)
    }

    fn offset(&self, x: i32, y: i32) -> Self {
        MapPoint { x: self.x + x, y: self.y + y }
    }

    pub fn neighbors(&self, map: &SkirmMap) -> Vec<(Self, usize)> {
        let mut neighbors = Vec::new();
        let points_to_check = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1, 0),           (1, 0),
            (-1, 1),  (0, 1),  (1, 1)
        ];
        let mut i = -1;
        for to_check in points_to_check {
            i += 1;
            let next_point = self.offset(to_check.0, to_check.1);
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

    pub fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn center_as_point2(&self) -> Point2 {
        let pixel = self.as_float_coord_tuple();
        let half_x = TILE_WIDTH / 2;
        let half_y = TILE_HEIGHT / 2;
        Point2::new(pixel.0 + (half_x as f32), pixel.1 + (half_y as f32))
    }

    pub fn as_pixel_coord_tuple(&self) -> (i32, i32) {
        (self.x * TILE_WIDTH, self.y * TILE_HEIGHT)
    }

    pub fn center(&self) -> (i32, i32) {
        (self.x * TILE_WIDTH + TILE_WIDTH / 2, self.y * TILE_HEIGHT + TILE_HEIGHT / 2)
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
                if c.to_ascii_char().unwrap() == AsciiChar::Hash {
                    map.insert(MapPoint::new(i as i32, j as i32), Tile::new(TileType::Wall));
                } else {
                    map.insert(MapPoint::new(i as i32, j as i32), Tile::new(TileType::Ground));
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
        let points = line_drawing::Bresenham::new(p1.as_tuple(), p2.as_tuple());
        let mut vec = Vec::new();
        for (x, y) in points {
            vec.push(MapPoint::new(x, y));
        }
        vec
    }

    pub fn pathfind(&self, p1: &MapPoint, p2: &MapPoint) -> Option<Vec<MapPoint>> {
        match dijkstra(p1, |p| p.neighbors(self), |p| *p == *p2) {
            Some(points) => Some(points.0),
            None => None,
        }
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
        let tile = self.map.get(point);
        if tile.is_some() {
            tile.unwrap().has_occupant()
        } else {
            false
        }
    }

    pub fn get_occupant(&self, point: &MapPoint) -> Option<Entity> {
        Some(self.map.get(point).unwrap().occupant.unwrap())
    }
}

pub fn tile_distance(p1: &MapPoint, p2: MapPoint) -> u16 {
    (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f64).sqrt() as u16
}

pub fn pixel_distance(p1: (i32, i32), p2: (i32, i32)) -> u16 {
    (((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)) as f64).sqrt() as u16
}
