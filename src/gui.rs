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
    fn draw_line_to_mouse(player_pos: &MapPoint, map: &SkirmMap, ctx: &mut Context) {
        let mouse_pos = mouse::get_position(ctx).unwrap();
        let mouse_x = mouse_pos.x;
        let mouse_y = mouse_pos.y;
        let mouse_tile = map.nearest_tile(&(mouse_x, mouse_y));

        let tiles_to_highlight = map.get_tiles_between(player_pos, &mouse_tile);

        let white = Color { r: 1.0, g: 1.0, b: 1.0, a: 0.1 };
        graphics::set_color(ctx, white).unwrap();

        for tile in tiles_to_highlight {
            let point = graphics::Point2::new(tile.x as f32, tile.y as f32);
            let rect = Rect::new(point.x, point.y, TILE_WIDTH as f32, TILE_HEIGHT as f32);
            graphics::rectangle(ctx, DrawMode::Fill, rect).unwrap();
        }
        graphics::set_color(ctx, Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }).unwrap();
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

            Gui::draw_line_to_mouse(player_pos, map, ctx);
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
