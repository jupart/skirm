use ggez::{graphics, Context};
use ggez::graphics::{Color, Point2, Rect, Text, DrawMode};

use asset_storage::AssetStorage;

pub enum PosHint {
    Left,
    Center,
    Right,
    Bottom,
    Top,
}

pub trait Widget {
    fn draw(&self, assets: &AssetStorage, ctx: &mut Context);
    fn handle_click(&mut self, mouse_pos: &Point2) -> bool;
    fn update_location(&mut self, window_w: u32, window_h: u32);
    fn get_pos_hint(&self) -> &Option<(PosHint, PosHint)>;
}

pub struct Button {
    pub pos: Point2,
    pub size: Point2,
    pub bg: Color,
    pub fg: Color,
    pub text: String,
    pub callback: Box<FnMut()>,
    pub pos_hint: Option<(PosHint, PosHint)>,
}

impl Widget for Button {
    fn draw(&self, assets: &AssetStorage, ctx: &mut Context) {
        // Rectangle button shape
        graphics::set_color(ctx, self.bg).unwrap();
        let rect = Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y);
        graphics::rectangle(ctx, DrawMode::Fill, rect).unwrap();

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

    // Returns `true` if clicked and calls its stored callback, else `false`
    fn handle_click(&mut self, mouse_pos: &Point2) -> bool {
        let l = self.pos.x;
        let r = l + self.size.x;
        let t = self.pos.y;
        let b = t + self.size.y;
        let x = mouse_pos.x;
        let y = mouse_pos.y;

        let is_in: bool;
        if x >= l && x <= r && y <= b && y >= t {
            is_in = true;
            (self.callback)();
        } else {
            is_in = false;
        }
        is_in
    }

    fn update_location(&mut self, window_w: u32, window_h: u32) {
        if self.get_pos_hint().is_some() {
            self.pos = match try_pos_from_hint(self.pos_hint.as_ref().unwrap(), &self.size, window_w, window_h) {
                Some(p) => p,
                None => self.pos,
            }
        }
    }

    fn get_pos_hint(&self) -> &Option<(PosHint, PosHint)> {
        &self.pos_hint
    }
}

impl Button {
    pub fn new(
        pos: Point2,
        size: Point2,
        bg: Color,
        fg: Color,
        text: String,
        callback: Box<FnMut()>,
        pos_hint: Option<(PosHint, PosHint)>,
        ctx: &Context
    ) -> Self {
        let mut hinted_pos = pos;
        if pos_hint.is_some() {
            let w = ctx.conf.window_mode.width;
            let h = ctx.conf.window_mode.height;
            hinted_pos = match try_pos_from_hint(&pos_hint.as_ref().unwrap(), &size, w, h) {
                Some(p) => p,
                None => pos,
            };
        } else {

        }

        Self { pos: hinted_pos, size, bg, fg, text, callback, pos_hint }
    }
}

pub fn try_pos_from_hint(hint: &(PosHint, PosHint), size: &Point2, window_w: u32, window_h: u32) -> Option<Point2> {
    let window_w = window_w as f32;
    let window_h = window_h as f32;
    let left = 0.0;
    let right = window_w - size.x;
    let top = 0.0;
    let bottom = window_h - size.y;
    let x_center = window_w / 2.0 - size.x / 2.0;
    let y_center = window_h / 2.0 - size.y / 2.0;

    let x = match hint.0 {
        PosHint::Left => Some(left),
        PosHint::Center => Some(x_center),
        PosHint::Right => Some(right),
        _ => None,
    };
    let y = match hint.1 {
        PosHint::Top => Some(top),
        PosHint::Center => Some(y_center),
        PosHint::Bottom => Some(bottom),
        _ => None,
    };

    let result;
    if x.is_none() || y.is_none() {
        result = None;
    } else {
        result = Some(Point2::new(x.unwrap(), y.unwrap()));
    }
    result
}
