use ggez::{graphics, Context};
use specs::{Entity, Entities, Fetch, FetchMut, System, ReadStorage, WriteStorage, Join};

use crate::{
    asset_storage::AssetStorage,
    components::*,
    resources::DeltaTime,
    input::{SkirmerInput, PendingCommand},
    rendering::{RenderType, WHITE},
    map::{SkirmMap, MapPoint, tile_distance},
    visual_effects::{GunshotEffect, GunshotEffects},
    item::{Weapon},
};

pub struct StatsSys;
impl<'a> System<'a> for StatsSys {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, StatsComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        info!("<- StatsSys");
        let (entities, stats) = data;
        for ent in entities.join() {
            let ent_stat = stats.get(ent).unwrap();
            if ent_stat.health == 0 {
                match entities.delete(ent) {
                    Err(_e) => (),
                    _ => (),
                }
            }
        }
        info!("-> StatsSys");
    }
}

pub struct PositionSys;
impl<'a> System<'a> for PositionSys {
    type SystemData = (
        WriteStorage<'a, PositionComp>,
    );

    fn run(&mut self, _data: Self::SystemData) {
        info!("<- PositionSys");
        info!("-> PositionSys");
    }
}

// An Input System that verifies and creates an entity's current_action
pub struct SkirmerInputSys;

impl<'a> System<'a> for SkirmerInputSys {
    type SystemData = (
        Fetch<'a, SkirmMap>,
        FetchMut<'a, SkirmerInput>,
        WriteStorage<'a, ActionComp>,
        ReadStorage<'a, PositionComp>,
        ReadStorage<'a, StatsComp>,
        WriteStorage<'a, TurnComp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        info!("<- SkirmerInputSys");
        let (skirmmap, mut input, mut action, position, stats, mut turn) = data;

        let turn = turn.get_mut(input.ent).unwrap();

        if input.pending_command.is_none()
            || input.command_point.is_none()
            || turn.phase == TurnPhase::Start
            || turn.phase == TurnPhase::Finish
        {
            info!("-> SkirmerInputSys");
            return;
        }

        let stats = stats.get(input.ent).unwrap();
        let p = position.get(input.ent).unwrap();
        let a = action.get_mut(input.ent).unwrap();

        let pos = MapPoint::from_pixel_coord(p.x as i32, p.y as i32);
        let to = input.command_point.map(|(x, y)| MapPoint::from_pixel_coord(x, y)).unwrap();
        match input.pending_command.unwrap() {
            PendingCommand::Move => {
                match MoveToPoint::new(pos, to, &*skirmmap) {
                    Ok(move_to_point) => {
                        if turn.try_update_move(&move_to_point, stats.move_per_turn).is_ok() {
                            turn.increment();
                            a.current_action = Action::MoveTo(move_to_point);
                        } else {
                            a.current_action = Action::Idle;
                        }
                    },
                    Err(()) => {
                        a.current_action = Action::Idle;
                    },
                }
            },
            PendingCommand::Attack => {
                if skirmmap.has_line_of_sight(&pos, &to) {
                    a.current_action = Action::AttackAt(to);
                    turn.increment();
                } else {
                    a.current_action = Action::Idle;
                }
            }
        }
        input.pending_command = None;
        input.command_point = None;
        info!("-> SkirmerInputSys");
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

    fn handle_move(&self, mtp: &mut MoveToPoint, pos: &mut PositionComp, dt: f32) -> Option<Action> {
        let mut change_to = None;

        let (x, y) = {
            let points_iter = mtp.point_stack.get_mut(0).unwrap();
            points_iter.as_float_coord_tuple()
        };
        let speed = 50.0;

        if self.position_close_to(pos.x, x)
        && self.position_close_to(pos.y, y) {
            pos.x = x;
            pos.y = y;
            mtp.point_stack.remove(0);
            if mtp.point_stack.is_empty() {
                change_to = Some(Action::Idle);
            }
        } else {
            let vec = (pos.x - x, pos.y - y);
            let mag = (vec.0.powf(2.0) + vec.1.powf(2.0)).sqrt();
            let unit = (vec.0 / mag, vec.1 / mag);
            let move_vec = (unit.0 * speed * dt, unit.1 * speed * dt);
            pos.x -= move_vec.0;
            pos.y -= move_vec.1;
        }

        change_to
    }

    fn handle_attack(&self, from: &MapPoint, to: &MapPoint, equipment: &EquipmentComp, map: &SkirmMap, effects: &mut GunshotEffects, stats: &mut WriteStorage<StatsComp>) -> Option<Action> {
        if map.has_occupant(to) {
            effects.effects.push(GunshotEffect::new(from.clone(), to.clone()));

            // play attack sound
            self.apply_damage(map.get_occupant(to).unwrap(), &equipment.weapon, tile_distance(from, *to), stats);
        }
        Some(Action::Idle)
    }

    fn apply_damage(&self, ent: Entity, _item: &Weapon, _distance: u16, stats: &mut WriteStorage<StatsComp>) {
        let ent_stats = stats.get_mut(ent).unwrap();
        if ent_stats.health < 50 {
            ent_stats.health = 0;
        } else {
            ent_stats.health -= 50;
        }
    }
}

impl<'a> System<'a> for ActionSys {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        WriteStorage<'a, StatsComp>,
        WriteStorage<'a, ActionComp>,
        WriteStorage<'a, PositionComp>,
        ReadStorage<'a, EquipmentComp>,
        Fetch<'a, SkirmMap>,
        FetchMut<'a, GunshotEffects>,
    );

    fn run(&mut self, data: Self::SystemData) {
        info!("<- ActionSys");
        let (time, mut stats, mut action_comp, mut position_comp, equipment, map, mut gun_effects) = data;
        let dt = time.delta.as_secs() as f32 + time.delta.subsec_nanos() as f32 * 1e-9;

        for (a, p, e) in (&mut action_comp, &mut position_comp, &equipment).join() {
            let change_to = match a.current_action {
                Action::MoveTo(ref mut move_to_point) => self.handle_move(move_to_point, p, dt),
                Action::AttackAt(point) => {
                    self.handle_attack(&MapPoint::from_pixel_coord(p.x as i32, p.y as i32), &point, e, &map, &mut gun_effects, &mut stats)
                },
                Action::Idle => None,
            };

            if change_to.is_some() {
                a.current_action = change_to.unwrap();
            }
        }
        info!("-> ActionSys");
    }
}

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

pub struct SoundSys;
impl<'a> System<'a> for SoundSys {
    type SystemData = (
        Fetch<'a, AssetStorage>,
        WriteStorage<'a, SoundComp>,
    );

    fn run(&mut self, _data: Self::SystemData) {
        info!("<- SoundSys");
        info!("-> SoundSys");
    }
}
