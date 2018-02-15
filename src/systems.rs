use ggez::{graphics, Context};
use specs::{Entities, Fetch, FetchMut, System, ReadStorage, WriteStorage, Join};

use asset_storage::AssetStorage;
use components::*;
use resources::{DeltaTime, PlayerInput};
use rendering::RenderType;

pub struct PositionSys;
impl<'a> System<'a> for PositionSys {
    type SystemData = (
        WriteStorage<'a, PositionComp>,
    );

    fn run(&mut self, data: Self::SystemData) {

    }
}

pub struct PlayerInputSys;
impl<'a> System<'a> for PlayerInputSys {
    type SystemData = (
        Entities<'a>,
        FetchMut<'a, PlayerInput>,
        WriteStorage<'a, ActionComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut input, mut action_comp) = data;
        for (e, a) in (&*entities, &mut action_comp).join() {
            if input.id == e.id() {
                if input.to_move {
                    a.current_action = Action::MoveTo { x: input.move_x, y: input.move_y };
                    input.to_move = false;
                }
            }
        }
    }
}

pub struct ActionSys;
impl ActionSys {
    fn position_close_to(&self, x1: f32, x2: f32) -> bool {
        let fluff = 1.0;
        (x2 - fluff <= x1)
            && (x1 <= x2 + fluff)
    }

}

impl<'a> System<'a> for ActionSys {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        WriteStorage<'a, ActionComp>,
        WriteStorage<'a, PositionComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, mut action_comp, mut position_comp) = data;
        let dt = time.delta.as_secs() as f32 + time.delta.subsec_nanos() as f32 * 1e-9;

        for (a, p) in (&mut action_comp, &mut position_comp).join() {
            match a.current_action {
                Action::MoveTo { x, y } => {
                    let speed = 50.0;

                    if self.position_close_to(p.x, x as f32)
                    && self.position_close_to(p.y, y as f32) {
                        p.x = x as f32;
                        p.y = y as f32;
                        a.current_action = Action::Idle;
                    } else {
                        let vec = (p.x - x as f32, p.y - y as f32);
                        let mag = (vec.0.powf(2.0) + vec.1.powf(2.0)).sqrt();
                        let unit = (vec.0 / mag, vec.1 / mag);
                        let move_vec = (unit.0 * speed * dt, unit.1 * speed * dt);
                        p.x -= move_vec.0;
                        p.y -= move_vec.1;
                    }
                }
                Action::Idle => (),
            }
        }
    }
}

pub struct RenderSys<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderSys<'c> {
    pub fn new(ctx: &'c mut Context) -> RenderSys<'c> {
        RenderSys { ctx }
    }

    fn draw_image(&mut self, id: &'static str, pos: (f32, f32), assets: &AssetStorage) {
        if let Some(image) = assets.images.get(id) {
            let point = graphics::Point2::new(pos.0, pos.1);
            graphics::draw(self.ctx, image, point, 0.0).unwrap();
        } else {
            // TODO: Log that we didn't find the image with id
        }
    }

    fn draw_glyph(&mut self, id: &'static str, pos: (f32, f32), assets: &AssetStorage) {
        let point = graphics::Point2::new(pos.0, pos.1);
        let glyph = graphics::Text::new(self.ctx, id, &assets.font).unwrap();
        graphics::draw(self.ctx, &glyph, point, 0.0).unwrap();
    }
}

impl<'a, 'c> System<'a> for RenderSys<'c> {
    type SystemData = (
        Fetch<'a, AssetStorage>,
        ReadStorage<'a, RenderComp>,
        ReadStorage<'a, PositionComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (assets, render_comp, position_comp) = data;

        for (r, p) in (&render_comp, &position_comp).join() {
            match r.render_type {
                RenderType::Image { id } => {
                    self.draw_image(id, (p.x, p.y), &assets);
                },
                RenderType::Glyph { id } => {
                    self.draw_glyph(id, (p.x, p.y), &assets);
                }
            }
        }
    }
}

pub struct SoundSys;
impl<'a> System<'a> for SoundSys {
    type SystemData = (
        Fetch<'a, AssetStorage>,
        WriteStorage<'a, SoundComp>,
    );

    fn run(&mut self, data: Self::SystemData) {

    }
}
