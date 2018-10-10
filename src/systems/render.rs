use specs::{Fetch, System, ReadStorage, Join};
use ggez::{graphics, graphics::Point2, Context};

use crate::{
    asset_storage::AssetStorage,
    camera::Camera,
    components::*,
    rendering::{RenderType, WHITE},
    map::SkirmMap,
};

pub struct RenderSys<'b, 'c> {
    ctx: &'c mut Context,
    camera: &'b Camera,
}

impl<'b, 'c> RenderSys<'b, 'c> {
    pub fn new(ctx: &'c mut Context, camera: &'b Camera) -> Self {
        Self { ctx, camera }
    }

    fn draw_image(&mut self, id: String, pos: (f32, f32), assets: &AssetStorage) {
        match assets.images.get(&id) {
            Some(image) => {
                let cam = self.camera.get_world_center();
                let point = Point2::new(pos.0 - cam.x, pos.1 - cam.y);
                graphics::set_color(self.ctx, WHITE).unwrap();
                graphics::draw(self.ctx, image, point, 0.0).unwrap();
            },
            None => (),
        }
    }
}

impl<'a, 'b, 'c> System<'a> for RenderSys<'b, 'c> {
    type SystemData = (
        Fetch<'a, AssetStorage>,
        Fetch<'a, SkirmMap>,
        ReadStorage<'a, RenderComp>,
        ReadStorage<'a, PositionComp>,
    );

    fn run(&mut self, (assets, map, render_comp, position_comp): Self::SystemData) {
        info!("<- RenderSys");
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

