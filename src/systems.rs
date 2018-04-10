// use std::time::Duration;

use ggez::{graphics, Context};
use specs::{Entities, Fetch, FetchMut, System, ReadStorage, WriteStorage, Join};

use asset_storage::AssetStorage;
use components::*;
use resources::DeltaTime;
use input::{PlayerInput, PendingCommand};
use rendering::{RenderType, WHITE};
use skirmmap::{SkirmMap, MapPoint};

pub struct PositionSys;
impl<'a> System<'a> for PositionSys {
    type SystemData = (
        WriteStorage<'a, PositionComp>,
    );

    fn run(&mut self, _data: Self::SystemData) {

    }
}

// An Input System that verifies and creates an entity's `current_action`
pub struct PlayerInputSys;
impl<'a> System<'a> for PlayerInputSys {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, SkirmMap>,
        FetchMut<'a, PlayerInput>,
        WriteStorage<'a, ActionComp>,
        ReadStorage<'a, PositionComp>,
        ReadStorage<'a, EquipmentComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, skirmmap, mut input, mut action_comp, position_comp, equipment_comp) = data;

        if input.pending_command.is_none() || input.command_point.is_none() {
            return;
        }

        let player_ent = entities.join().nth(input.id as usize).unwrap();
        let p = position_comp.get(player_ent).unwrap();
        let e = equipment_comp.get(player_ent).unwrap();
        let a = action_comp.get_mut(player_ent).unwrap();

        let pos = MapPoint::round_from_pixel_coord(p.x as i32, p.y as i32);
        let to = input.command_point.map(|(x, y)| MapPoint::round_from_pixel_coord(x, y)).unwrap();
        match input.pending_command.unwrap() {
            PendingCommand::Move => {
                match MoveToPoint::new(pos, to, &*skirmmap) {
                    Ok(move_to_point) => {
                        a.current_action = Action::MoveTo(move_to_point);
                    },
                    Err(()) => {
                        a.current_action = Action::Idle;
                    },
                }
            },
            PendingCommand::Attack => {
                if skirmmap.has_line_of_sight(&pos, &to) {
                    a.current_action = Action::AttackAt(to);
                } else {
                    a.current_action = Action::Idle;
                }
            }
        }
        input.pending_command = None;
        input.command_point = None;
    }
}

// Performs entities' `current_action`s
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
        ReadStorage<'a, StatsComp>,
        WriteStorage<'a, ActionComp>,
        WriteStorage<'a, PositionComp>,
        ReadStorage<'a, EquipmentComp>,
        Fetch<'a, SkirmMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, _stats, mut action_comp, mut position_comp, equipment, map) = data;
        let dt = time.delta.as_secs() as f32 + time.delta.subsec_nanos() as f32 * 1e-9;

        for (a, p, e) in (&mut action_comp, &mut position_comp, &equipment).join() {
            let mut change_to = None;

            match a.current_action {
                Action::MoveTo(ref mut move_to_point) => {
                    let (x, y) = {
                        let points_iter = move_to_point.point_stack.get_mut(0).unwrap();
                        points_iter.as_pixel_coord_tuple()
                    };
                    let speed = 50.0;

                    if self.position_close_to(p.x, x)
                    && self.position_close_to(p.y, y) {
                        p.x = x;
                        p.y = y;
                        move_to_point.point_stack.remove(0);
                        if move_to_point.point_stack.is_empty() {
                            change_to = Some(Action::Idle);
                        }
                    } else {
                        let vec = (p.x - x, p.y - y);
                        let mag = (vec.0.powf(2.0) + vec.1.powf(2.0)).sqrt();
                        let unit = (vec.0 / mag, vec.1 / mag);
                        let move_vec = (unit.0 * speed * dt, unit.1 * speed * dt);
                        p.x -= move_vec.0;
                        p.y -= move_vec.1;
                    }
                },
                Action::AttackAt(point) => {
                    // let intended_damage = e.weapon.attack(&point);
                    if map.has_occupant(&point) {
                        println!("Attacked");
                    }
                    // draw attack
                    // play attack sound
                    change_to = Some(Action::Idle);
                }
                Action::Idle => (),
            }

            if change_to.is_some() {
                a.current_action = change_to.unwrap();
            }
        }
    }
}

pub struct RenderSys<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderSys<'c> {
    pub fn new(ctx: &'c mut Context) -> Self {
        Self { ctx }
    }

    fn draw_image(&mut self, id: &'static str, pos: (f32, f32), assets: &AssetStorage) {
        if let Some(image) = assets.images.get(id) {
            let point = graphics::Point2::new(pos.0, pos.1);
            graphics::set_color(self.ctx, WHITE).unwrap();
            graphics::draw(self.ctx, image, point, 0.0).unwrap();
        } else {
            // TODO: Log that we didn't find the image with id
        }
    }

    fn draw_glyph(&mut self, id: &'static str, pos: (f32, f32), assets: &AssetStorage) {
        let point = graphics::Point2::new(pos.0, pos.1);
        let glyph = graphics::Text::new(self.ctx, id, &assets.font).unwrap();
        graphics::set_color(self.ctx, WHITE).unwrap();
        graphics::draw(self.ctx, &glyph, point, 0.0).unwrap();
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
        let (assets, map, render_comp, position_comp) = data;

        // Draw map
        for (point, tile) in &map.map {
            // if not in viewport, continue

            self.draw_glyph(tile.glyph, point.as_pixel_coord_tuple(), &assets);
        }

        // Draw entities
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

    fn run(&mut self, _data: Self::SystemData) {

    }
}
