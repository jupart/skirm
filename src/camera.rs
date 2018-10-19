use crate::{Point2, Vector2};

const CAMERA_SLOP: f32 = 10.0;

#[derive(Debug)]
pub struct Camera {
    pub center: Point2,
    pub screen_size: Vector2,
    pub speed: f32,
    pub focus: Option<Point2>,
}

impl Camera {
    pub fn new(scrn_w: u32, scrn_h: u32) -> Self {
        let screen_size = Vector2::new(scrn_w as f32, scrn_h as f32);
        Self {
            center: Point2::new(0.0, 0.0),
            screen_size,
            speed: 100.0,
            focus: None,
        }
    }

    pub fn update_screen(&mut self, w: f32, h: f32) {
        self.screen_size = Vector2::new(w, h);
    }

    pub fn update_center(&mut self, dt: f32) {
        if self.focus.is_some() {
            let vec = self.focus.unwrap() - self.center;
            if vec.norm() > CAMERA_SLOP {
                let direction_vec = vec.normalize();
                let to_move = direction_vec * self.speed * dt;
                self.center += to_move;
            }
        }
    }

    pub fn get_world_center(&self) -> Point2 {
        let screen_x = self.screen_size.x;
        let screen_y = self.screen_size.y;
        let x = self.center.x;
        let y = self.center.y;
        Point2::new(x - screen_x / 2.0, y - screen_y / 2.0)
    }

    pub fn hard_focus(&mut self) {
        if self.focus.is_some() {
            self.center = self.focus.unwrap();
        }
    }
}
