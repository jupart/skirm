use ggez::{graphics, Context};
use ggez::graphics::{Color, Point2, Rect, Text, DrawMode};

use asset_storage::AssetStorage;

pub struct PosHint {
    pub x: f32,
    pub y: f32,
}

pub trait Widget {
    fn draw(&self, assets: &AssetStorage, ctx: &mut Context);
    fn handle_click(&mut self, mouse_pos: &Point2) -> bool;
    fn handle_release(&mut self, mouse_pos: &Point2) -> bool;
    fn update_location(&mut self, window_w: u32, window_h: u32);
}

pub struct Button {
    pub pos: Point2,
    pub size: Point2,
    pub bg: Color,
    pub fg: Color,
    pub text: String,
    pub callback: Box<FnMut()>,
    pub pos_hint: Option<PosHint>,
    pub is_pressed: bool,
}

impl Widget for Button {
    fn draw(&self, assets: &AssetStorage, ctx: &mut Context) {
        // Rectangle button shape
        if self.is_pressed {
            graphics::set_color(ctx, self.bg).unwrap();
            let rect = Rect::new(self.pos.x + 1.0, self.pos.y + 1.0, self.size.x - 2.0, self.size.y - 2.0);
            graphics::rectangle(ctx, DrawMode::Fill, rect).unwrap();
        } else {
            graphics::set_color(ctx, self.bg).unwrap();
            let rect = Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y);
            graphics::rectangle(ctx, DrawMode::Fill, rect).unwrap();
        }

        // Text
        let h = assets.font.get_height() as f32;
        let w = assets.font.get_width(&self.text) as f32;
        let x = (self.pos.x + (self.size.x - w) / 2.0).floor();
        let y = (self.pos.y + (self.size.y - h) / 2.0).floor();
        graphics::set_color(ctx, self.fg).unwrap();
        let draw_point = Point2::new(x, y);
        let draw_text = Text::new(ctx, &self.text, &assets.font).unwrap();
        graphics::draw(ctx, &draw_text, draw_point, 0.0).unwrap();
    }

    fn handle_click(&mut self, mouse_pos: &Point2) -> bool {
        let left = self.pos.x;
        let right = left + self.size.x;
        let top = self.pos.y;
        let bottom = top + self.size.y;

        let is_in: bool;
        if mouse_pos.x >= left && mouse_pos.x <= right && mouse_pos.y <= bottom && mouse_pos.y >= top {
            is_in = true;
            self.is_pressed = true;
        } else {
            is_in = false;
        }
        is_in
    }

    fn handle_release(&mut self, mouse_pos: &Point2) -> bool {
        let left = self.pos.x;
        let right = left + self.size.x;
        let top = self.pos.y;
        let bottom = top + self.size.y;

        let is_in = mouse_pos.x >= left && mouse_pos.x <= right && mouse_pos.y <= bottom && mouse_pos.y >= top;
        if is_in && self.is_pressed {
            (self.callback)();
        }
        self.is_pressed = false;
        is_in
    }

    fn update_location(&mut self, window_w: u32, window_h: u32) {
        self.pos = pos_from_hint(self.pos_hint.as_ref().unwrap(), &self.size, window_w, window_h);
    }
}

impl Button {
    pub fn new(
        size: Point2,
        bg: Color,
        fg: Color,
        text: String,
        callback: Box<FnMut()>,
        pos_hint: Option<PosHint>,
        ctx: &Context
    ) -> Self {
        let w = ctx.conf.window_mode.width;
        let h = ctx.conf.window_mode.height;
        let hinted_pos = pos_from_hint(pos_hint.as_ref().unwrap(), &size, w, h);

        Self { pos: hinted_pos, size, bg, fg, text, callback, pos_hint, is_pressed: false }
    }
}

pub fn pos_from_hint(hint: &PosHint, size: &Point2, window_w: u32, window_h: u32) -> Point2 {
    let x = hint.x * (window_w as f32 - size.x);
    let y = hint.y * (window_h as f32 - size.y);

    Point2::new(x, y)
}
