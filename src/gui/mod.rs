mod widgets;

use ggez::{graphics, Context};
use ggez::graphics::{Color, Rect, DrawMode, Point2};
use ggez::mouse;

use rendering::{WHITE, BLACK};
use asset_storage::AssetStorage;
use input::{PendingCommand, PlayerInput};
use skirmmap::{MapPoint, SkirmMap, TILE_WIDTH, TILE_HEIGHT};
use self::widgets::{Widget, Button, PosHint};

fn simple_callback() {
    println!("clicked");
}

pub struct Gui {
    pub widgets: Vec<Box<Widget>>,
}

impl Gui {
    pub fn new(ctx: &Context) -> Self {
        let widgets = main_gui_state_widgets(ctx);
        Self { widgets }
    }

    fn draw_line_to_mouse(&self, player_pos: &MapPoint, mode: PendingCommand, map: &SkirmMap, ctx: &mut Context) {
        let mouse_pos = mouse::get_position(ctx).unwrap();
        let mouse_x = mouse_pos.x;
        let mouse_y = mouse_pos.y;
        let mouse_tile = map.nearest_tile(mouse_x as i32, mouse_y as i32);

        let tiles_to_highlight = map.get_tiles_between(player_pos, &mouse_tile, mode);
        if tiles_to_highlight.is_empty() {
            return
        }

        let white = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.05 };
        let true_white = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        let red = Color { r: 1.0, g: 0.0, b: 0.0, a: 0.05 };
        graphics::set_color(ctx, white).unwrap();

        for tile in &tiles_to_highlight {
            if !map.has_ground_at(tile) {
                graphics::set_color(ctx, red).unwrap();
            }
            let point = graphics::Point2::new(tile.x as f32, tile.y as f32);
            let rect = Rect::new(point.x, point.y, TILE_WIDTH as f32, TILE_HEIGHT as f32);
            graphics::rectangle(ctx, DrawMode::Fill, rect).unwrap();
        }

        // Draw last tile twice
        let tile = tiles_to_highlight.last().unwrap();
        let point = graphics::Point2::new(tile.x as f32, tile.y as f32);
        let rect = Rect::new(point.x, point.y, TILE_WIDTH as f32, TILE_HEIGHT as f32);
        graphics::rectangle(ctx, DrawMode::Fill, rect).unwrap();

        // Reset the color
        graphics::set_color(ctx, true_white).unwrap();
    }

    pub fn draw(&self, player_pos: &MapPoint, input: &PlayerInput, assets: &AssetStorage, map: &SkirmMap, ctx: &mut Context) {
        for widget in &self.widgets {
            widget.draw(assets, ctx);
        }

        if input.pending_command.is_some() {
            let x = 0.0;
            let y = (ctx.conf.window_mode.height - TILE_HEIGHT as u32) as f32;
            let point = graphics::Point2::new(x, y);
            let word;
            match input.pending_command.unwrap() {
                PendingCommand::Move => {
                    word = graphics::Text::new(ctx, "Move", &assets.font).unwrap();
                },
                PendingCommand::Attack => {
                    word = graphics::Text::new(ctx, "Attack", &assets.font).unwrap();
                },
            }
            graphics::draw(ctx, &word, point, 0.0).unwrap();

            self.draw_line_to_mouse(player_pos, input.pending_command.unwrap(), map, ctx);
        }
    }

    pub fn handle_click(&mut self, mouse_pos: Point2) -> bool {
        let mut captured_click = false;
        for widget in &mut self.widgets {
            if widget.handle_click(&mouse_pos) {
                captured_click = true;
            }
        }
        captured_click
    }

    pub fn handle_release(&mut self, mouse_pos: Point2) -> bool {
        let mut captured_click = false;
        for widget in &mut self.widgets {
            if widget.handle_release(&mouse_pos) {
                captured_click = true;
            }
        }
        captured_click
    }

    pub fn window_resized(&mut self, w: u32, h: u32) {
        for widget in &mut self.widgets {
            widget.update_location(w, h);
        }
    }

    pub fn add_widget(&mut self, widget: Box<Widget>) {
        self.widgets.push(widget);
    }
}

fn main_gui_state_widgets(ctx: &Context) -> Vec<Box<Widget>> {
    let mut widgets: Vec<Box<Widget>> = Vec::new();

    widgets.push(Box::new(Button::new(
        Point2::new(75.0, 20.0),
        WHITE,
        BLACK,
        String::from("Clickme"),
        Box::new(simple_callback),
        Some(PosHint { x: 0.0, y: 1.0 }),
        ctx,
    )));

    widgets.push(Box::new(Button::new(
        Point2::new(75.0, 20.0),
        WHITE,
        BLACK,
        String::from("Clickme2"),
        Box::new(simple_callback),
        Some(PosHint { x: 0.5, y: 1.0 }),
        ctx,
    )));

    widgets.push(Box::new(Button::new(
        Point2::new(75.0, 20.0),
        WHITE,
        BLACK,
        String::from("Clickme3"),
        Box::new(simple_callback),
        Some(PosHint { x: 1.0, y: 1.0 }),
        ctx,
    )));

    widgets
}
