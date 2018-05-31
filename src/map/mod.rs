use std::fs::{File, read_dir};
use std::path::Path;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use specs::Entity;
use ggez::{GameResult};
use ascii::{ToAsciiChar, AsciiChar};
use pathfinding::dijkstra;
use line_drawing;

mod point;
mod tile;
pub use self::point::MapPoint;
pub use self::tile::{Tile, TileType};

pub const TILE_WIDTH: i32 = 8;
pub const TILE_HEIGHT: i32 = 14;

#[derive(PartialEq, Eq, Debug)]
pub enum MapError {
    Occupied,
    PointDoesNotExist,
}

pub struct SkirmMap {
    pub map: HashMap<MapPoint, Tile>,
}

impl SkirmMap {
    pub fn has_ground_at(&self, point: &MapPoint) -> bool {
        match self.map.get(point) {
            Some(tile) => {
                tile.tile_type == TileType::Ground
            },
            None => false,
        }
    }

    pub fn get_tiles_between(&self, p1: &MapPoint, p2: &MapPoint) -> Vec<MapPoint> {
        let points = line_drawing::Bresenham3d::new(p1.as_tuple(), p2.as_tuple());
        let mut vec = Vec::new();
        for (x, y, z) in points {
            vec.push(MapPoint::new(x, y, z));
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
    (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f32).sqrt() as u16
}

pub fn pixel_distance(p1: (i32, i32), p2: (i32, i32)) -> u16 {
    (((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)) as f32).sqrt() as u16
}

/// Takes a directory, loops over 0.txt, 1.txt, etc. in that directory to build map
pub fn load_map<P>(path: P) -> GameResult<SkirmMap>
    where P: AsRef<Path> + Debug,
{
    let full_map = HashMap::new();
    let file_iter = read_dir(&path).unwrap();
    for file in file_iter {
        let path = file.unwrap().path();
        let level = path.to_str().unwrap().split(".").next().unwrap().parse::<i32>().unwrap();
        let level_map = read_level_file(path, level)?;

        for (point, tile) in level_map {
            full_map.insert(point, tile);
        }
    }

    Ok(SkirmMap { map: full_map })
}

fn read_level_file<P>(path: P, z_level: i32) -> GameResult<HashMap<MapPoint, Tile>>
    where P: AsRef<Path> + Debug,
{
    let map_file = File::open(path)?;
    let buffer = BufReader::new(map_file);

    let mut map = HashMap::new();
    for (j, line) in buffer.lines().enumerate() {
        for (i, c) in line.unwrap().chars().enumerate() {
            if c.to_ascii_char().unwrap() == AsciiChar::Hash {
                map.insert(MapPoint::new(i as i32, j as i32, z_level), Tile::new(TileType::Wall));
            } else {
                map.insert(MapPoint::new(i as i32, j as i32, z_level), Tile::new(TileType::Ground));
            }
        }
    }

    Ok(map)
}
