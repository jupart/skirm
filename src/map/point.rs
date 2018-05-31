use ggez::graphics::Point2;

use map::{SkirmMap, TILE_WIDTH, TILE_HEIGHT};
use map::tile::TileType;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct MapPoint {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl MapPoint {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn as_float_coord_tuple(&self) -> (f32, f32, f32) {
        ((self.x * TILE_WIDTH) as f32, (self.y * TILE_HEIGHT) as f32, self.z as f32)
    }

    pub fn from_pixel_coord(x: i32, y: i32, z: i32) -> Self {
        let tile_index_x = x / TILE_WIDTH;
        let tile_index_y = y / TILE_HEIGHT;
        MapPoint::new(tile_index_x, tile_index_y, z)
    }

    fn offset(&self, x: i32, y: i32, z: i32) -> Self {
        MapPoint { x: self.x + x, y: self.y + y, z: self.z + z }
    }

    pub fn neighbors(&self, map: &SkirmMap) -> Vec<(Self, usize)> {
        let mut neighbors = Vec::new();
        let points_to_check = vec![
            (-1, -1, 0), (0, -1, 0), (1, -1, 0),
            (-1, 0, 0),              (1, 0, 0),
            (-1, 1, 0),  (0, 1, 0),  (1, 1, 0)
        ];
        let mut i = -1;
        for to_check in points_to_check {
            i += 1;
            let next_point = self.offset(to_check.0, to_check.1, 0);
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

    pub fn as_tuple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    pub fn center_as_point2(&self) -> Point2 {
        let pixel = self.as_float_coord_tuple();
        let half_x = TILE_WIDTH / 2;
        let half_y = TILE_HEIGHT / 2;
        Point2::new(pixel.0 + (half_x as f32), pixel.1 + (half_y as f32), pixel.2)
    }

    pub fn as_pixel_coord_tuple(&self) -> (i32, i32, i32) {
        (self.x * TILE_WIDTH, self.y * TILE_HEIGHT, self.z)
    }

    pub fn center(&self) -> (i32, i32) {
        (self.x * TILE_WIDTH + TILE_WIDTH / 2, self.y * TILE_HEIGHT + TILE_HEIGHT / 2, self.z)
    }
}
