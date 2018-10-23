use ncollide2d::{
    shape::{Cuboid, ShapeHandle},
    world::{CollisionGroups, GeometricQueryType},
};

use nalgebra::Isometry2;

use std::{
    fs::File,
    path::Path,
    fmt::Debug,
    io::{ BufRead, BufReader },
    collections::HashMap,
};

use specs::Entity;
use ascii::{ToAsciiChar, AsciiChar};
use pathfinding::dijkstra;
use line_drawing;

pub mod point;
pub mod tile;
pub use self::point::MapPoint;
pub use self::tile::{Tile, TileType};

use crate::{
    SkirmResult,
    game::TILE_COLLISION_GROUP,
    CollisionWorld,
    Vector2,
    components::*,
};

pub const TILE_WIDTH: i32 = 32;
pub const TILE_HEIGHT: i32 = 32;

#[derive(PartialEq, Eq, Debug)]
pub enum MapError {
    Occupied,
    PointDoesNotExist,
}

#[derive(Debug)]
pub struct SkirmMap {
    pub map: HashMap<MapPoint, Tile>,
}

impl SkirmMap {
    pub fn load<P>(path: P, world: &mut specs::World) -> SkirmResult<Self>
        where P: AsRef<Path> + Debug,
    {
        let map_file = File::open(path)?;
        let buffer = BufReader::new(map_file);

        let mut map = HashMap::new();
        for (j, line) in buffer.lines().enumerate() {
            for (i, c) in line.unwrap().chars().enumerate() {
                if c.to_ascii_char().unwrap() == AsciiChar::Hash {
                    create_map_entity(world, &mut map, i as i32, j as i32, Some(TileType::Ground));
                }
                else {
                    create_map_entity(world, &mut map, i as i32, j as i32, None);
                }
            }
        }

        Ok(Self { map })
    }

    pub fn has_ground_at(&self, point: &MapPoint) -> bool {
        match self.map.get(point) {
            Some(tile) => {
                tile.tile_type == Some(TileType::Ground)
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
    (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f32).sqrt() as u16
}

pub fn pixel_distance(p1: (i32, i32), p2: (i32, i32)) -> u16 {
    (((p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2)) as f32).sqrt() as u16
}

fn create_map_entity(world: &mut specs::World, map: &mut HashMap<MapPoint, Tile>, x: i32, y: i32, tile: Option<TileType>) {
    // Insert into the map for quick lookup
    let point = MapPoint::new(x, y);
    map.insert(point, Tile::new(tile));

    if tile.is_some() {
        let pixel = point.as_float_coord_tuple();
        let ent = world.create_entity()
            .with(PositionComp::new(pixel.0, pixel.1))
            .with(SpriteComp::new(String::from("green_box")))
            .build();

        // Collision info
        let shape = Cuboid::new(Vector2::new(12.0, 12.0));
        let mut group = CollisionGroups::new();
        group.set_membership(&[TILE_COLLISION_GROUP]);
        group.set_blacklist(&[TILE_COLLISION_GROUP]);
        let query_type = GeometricQueryType::Contacts(0.0, 0.0);

        let collider = {
            let mut collide_world = world.write_resource::<CollisionWorld>();
            let handle = collide_world.add(
                Isometry2::new(Vector2::new(0.0, -6.0), nalgebra::zero()),
                ShapeHandle::new(shape.clone()),
                group,
                query_type,
                ent,
            );

            CollideComp {
                handle,
            }
        };

        world.write::<CollideComp>().insert(ent, collider);
    }
}
