use ggez::{graphics, Context};
use ggez::graphics::{Color, Rect, DrawMode};
use ggez::mouse;

use asset_storage::AssetStorage;
use input::{PendingCommand, PlayerInput};
use skirmmap::{MapPoint, SkirmMap, TILE_WIDTH, TILE_HEIGHT};

pub enum GuiEvent {

}

pub struct Gui;

impl Gui {
    fn draw_line_to_mouse(player_pos: &MapPoint, mode: PendingCommand, map: &SkirmMap, ctx: &mut Context) {
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

    pub fn draw_elements(
        player_pos: &MapPoint,
        input: &PlayerInput,
        assets: &AssetStorage,
        map: &SkirmMap,
        ctx: &mut Context
    ) -> Option<GuiEvent> {
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

            Gui::draw_line_to_mouse(player_pos, input.pending_command.unwrap(), map, ctx);
        }
        None
    }

    pub fn handle_event(event: Option<GuiEvent>) {
        if event.is_some() {
            match event {
                _ => ()
            }
        }
    }
}
