use specs::{Fetch, System, ReadStorage, Join};
use ggez::{graphics, Context};

use crate::{
    asset_storage::AssetStorage,
    components::*,
    rendering::{RenderType, WHITE},
    map::SkirmMap,
};

pub struct RenderSys<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderSys<'c> {
    pub fn new(ctx: &'c mut Context) -> Self {
        Self { ctx }
    }

    fn draw_image(&mut self, id: String, pos: (f32, f32), assets: &AssetStorage) {
        if let Some(image) = assets.images.get(&id) {
            let point = graphics::Point2::new(pos.0, pos.1);
            graphics::set_color(self.ctx, WHITE).unwrap();
            graphics::draw(self.ctx, image, point, 0.0).unwrap();
        } else {
            // TODO: Log that we didn't find the image with id
        }
    }
}

impl<'a, 'c> System<'a> for RenderSys<'c> {
    type SystemData = (
        Fetch<'a, AssetStorage>,
        Fetch<'a, SkirmMap>,
        ReadStorage<'a, RenderComp>,
        ReadStorage<'a, PositionComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        info!("<- RenderSys");
        let (assets, map, render_comp, position_comp) = data;

        // Draw map
        for (point, tile) in &map.map {
            // if not in viewport, continue
            if tile.tile_type.is_some() {
                let image_id = assets.tiles.get(tile.tile_type.as_ref().unwrap()).unwrap();
                self.draw_image(image_id.to_string(), point.as_float_coord_tuple(), &assets);
            }
        }

        // Draw entities
        for (r, p) in (&render_comp, &position_comp).join() {
            match r.render_type {
                RenderType::Image { id } => {
                    self.draw_image(id.to_string(), (p.x, p.y), &assets);
                },
            }
        }
        info!("-> RenderSys");
    }
}

