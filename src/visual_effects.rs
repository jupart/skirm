use ggez::Context;
use ggez::graphics;
use ggez::graphics::{MeshBuilder, Point2};
use line_drawing;

use crate::map::{MapPoint, pixel_distance};

const DISTANCE_PER_DRAW: u8 = 10;

pub struct GunshotEffects {
    pub effects: Vec<GunshotEffect>,
}

pub struct GunshotEffect {
    pub lines_to_draw: Vec<(Point2, Point2)>,
}

impl GunshotEffect {
    pub fn new(p1: MapPoint, p2: MapPoint) -> Self {
        let full_distance = pixel_distance(p1.as_pixel_coord_tuple(), p2.as_pixel_coord_tuple());
        let draws = full_distance / DISTANCE_PER_DRAW as u16;
        let mut points: Vec<(i32, i32)> = line_drawing::Bresenham::new(p1.center(), p2.center()).collect();
        let mut points_left = points.to_vec();

        let mut lines_to_draw = Vec::new();
        for _i in 0..draws {
            let first_point = points_left[0].clone();

            for (i, point) in points_left.iter().enumerate() {
                let distance = pixel_distance(first_point, *point);
                if distance > DISTANCE_PER_DRAW as u16 {
                    lines_to_draw.push((Point2::new(first_point.0 as f32, first_point.1 as f32), Point2::new(point.0 as f32, point.1 as f32)));
                    points = points_left[i..].to_vec();
                    break;
                }
            }

            points_left = points.to_vec();
        }

        lines_to_draw.reverse();
        GunshotEffect { lines_to_draw }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mut mb = MeshBuilder::new();
        let line = self.lines_to_draw.pop().unwrap();
        let points = [line.0, line.1];
        let mesh = mb.line(&points, 1.0).build(ctx).unwrap();
        graphics::draw_ex(ctx, &mesh, Default::default()).unwrap();
    }

    pub fn finished(&self) -> bool {
        self.lines_to_draw.is_empty()
    }
}
