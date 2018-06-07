use ggez::{timer, event, graphics, Context};
use ggez::event::{Keycode, Mod, MouseButton};
use ggez::graphics::{Point2, Rect};
use specs::{World, Dispatcher, DispatcherBuilder, RunNow, Entity};

use std::collections::HashMap;
use std::time::Duration;

use asset_storage::AssetStorage;
use components::*;
use systems::*;
use rendering::BLACK;
use resources::DeltaTime;
use input::{SkirmerInput, PendingCommand};
use item::ItemFactory;
use skirmer::{SkirmerFactory, SkirmerType::Fighter, SkirmerType::Sniper};
use map::{MapPoint, SkirmMap};
use gui::{Gui};
use visual_effects::{GunshotEffect, GunshotEffects};

use SkirmResult;

pub struct Game<'a, 'b> {
    pub world: World,
    pub skirmers: Vec<Entity>,
    pub p1_ent: Entity,
    pub gui: Gui,
    pub dispatcher: Dispatcher<'a, 'b>,
    pub has_focus: bool,
    pub paused: bool,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> SkirmResult<Self> {
        let mut world = World::new();
        register_components(&mut world);

        info!("Build storage and skirmer/item factories");
        let mut asset_storage = AssetStorage::new(ctx)?;
        let item_factory = ItemFactory::new()?;
        let skirmer_factory = SkirmerFactory::new();
        let mut map = SkirmMap::load("./resources/maps/test.skirm_map")?;

        asset_storage.load_images(ctx)?;
        asset_storage.load_sounds(ctx)?;

        let mut ent1_sounds = HashMap::new();
        ent1_sounds.insert(SoundType::Move, ("sine", true));

        info!("Create entities");
        let p1_ent = skirmer_factory.create_skirmer(8, 1, &Fighter, &item_factory, &mut map, &mut world).unwrap();
        let npc_ent = skirmer_factory.create_skirmer(8, 4, &Sniper, &item_factory, &mut map, &mut world).unwrap();
        let skirmers = vec![p1_ent, npc_ent];

        let gunshot_effects: Vec<GunshotEffect> = Vec::new();

        info!("Add specs shared resources");
        world.add_resource(asset_storage);
        world.add_resource(DeltaTime { delta: Duration::new(0, 0) });
        world.add_resource(SkirmerInput::new(p1_ent));
        world.add_resource(map);
        world.add_resource(GunshotEffects { effects: gunshot_effects });

        info!("Build system dispatcher");
        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .add(ActionSys, "action", &[])
            .add(SkirmerInputSys, "player_input", &[])
            .add(PositionSys, "position", &[])
            .add(StatsSys, "stats", &[])
            .add(SoundSys, "sound", &[])
            .build();

        info!("Build gui");
        let gui = Gui::new(ctx);

        graphics::set_background_color(ctx, BLACK);

        Ok(Self {
            world,
            skirmers,
            p1_ent,
            gui,
            dispatcher,
            has_focus: true,
            paused: false,
        })
    }

    fn draw_effects(&self, ctx: &mut Context, gun_effects: &mut Vec<GunshotEffect>) {
        for effect in &mut *gun_effects {
            effect.draw(ctx);
        }
        gun_effects.retain(|effect| !effect.finished());
    }

    fn update_game(&mut self, ctx: &mut Context) {
        // self.print_fps(ctx);

        self.update_current_skirmer_turn();

        // Update delta time for frame
        let dt = &timer::get_delta(ctx);
        self.world.write_resource::<DeltaTime>().delta = *dt;

        info!("  <- Dispatch the specs systems");
        self.dispatcher.dispatch(&self.world.res);
        info!("  -> Dispatch the specs systems");

        // Perform specs maintenance, removing entities, etc.
        self.world.maintain();
    }

    fn print_fps(&self, ctx: &mut Context) {
        if timer::get_ticks(ctx) % 50 == 0 {
            println!("FPS: {}", timer::get_fps(ctx));
        }
    }

    fn update_current_skirmer_turn(&mut self) {
        let mut turn_comps = self.world.write::<TurnComp>();
        let mut input = self.world.write_resource::<SkirmerInput>();
        let mut start_new_turn = false;

        {
            let active_turn_comp = turn_comps.get_mut(input.ent).unwrap();

            if active_turn_comp.phase == TurnPhase::Finish {
                start_new_turn = true;
                active_turn_comp.phase = TurnPhase::FirstAction;

                // Increment skirmer turn
                if input.ent == *self.skirmers.last().unwrap() {
                    input.ent = self.skirmers[0];
                } else {
                    let active_idx = self.skirmers.binary_search(&input.ent).unwrap() + 1;
                    input.ent = self.skirmers[active_idx];
                }
            }
        }

        if start_new_turn {
            let new_active_turn_comp = turn_comps.get_mut(input.ent).unwrap();
            new_active_turn_comp.phase = TurnPhase::FirstAction;
        }
    }
}

impl<'a, 'b> event::EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> SkirmResult {
        if self.has_focus && !self.paused {
            info!("<- Update Game");
            self.update_game(ctx);
            info!("-> Update Game");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> SkirmResult {
        info!("<- Draw Game");
        let input = self.world.read_resource::<SkirmerInput>();
        let assets = self.world.read_resource::<AssetStorage>();
        let map = self.world.read_resource::<SkirmMap>();
        let mut gun_effects = self.world.write_resource::<GunshotEffects>();

        let pos_components = self.world.read::<PositionComp>();
        let player_pos = pos_components.get(input.ent).unwrap();
        let pos = MapPoint::from_pixel_coord(player_pos.x as i32, player_pos.y as i32);

        graphics::clear(ctx);

        // Entity rendering via RenderSys
        {
            let mut rs = RenderSys::new(ctx);
            rs.run_now(&self.world.res);
        }

        // Effects rendering
        self.draw_effects(ctx, &mut gun_effects.effects);

        // Gui rendering
        self.gui.draw(&pos, &input, &assets, &*map, ctx);

        graphics::present(ctx);

        timer::yield_now();
        info!("-> Draw Game");
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<SkirmerInput>();

        if repeat {
            return;
        }

        match keycode {
            Keycode::A => input.set_pending_command(PendingCommand::Attack),
            Keycode::M => input.set_pending_command(PendingCommand::Move),
            Keycode::Escape => input.clear_pending_command(),
            _ => ()
        }
    }

    fn focus_event(&mut self, _ctx: &mut Context, has_focus: bool) {
        self.has_focus = has_focus;
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        let mut input = self.world.write_resource::<SkirmerInput>();

        if self.gui.handle_click(Point2::new(x as f32, y as f32)) {
            return
        }

        match button {
            MouseButton::Left => input.select_point(x, y),
            MouseButton::Right => (),
            MouseButton::Middle => (),
            _ => (),
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
        if self.gui.handle_release(Point2::new(x as f32, y as f32)) {
            return
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        let rect = Rect::new(0.0, 0.0, width as f32, height as f32);
        graphics::set_screen_coordinates(ctx, rect).unwrap();
        self.gui.window_resized(width, height);
    }

    // fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
    // fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) { }
    // fn mouse_motion_event(&mut self, _state: MouseState, _x: i32, _y: i32, _xrel: i32, _yrel: i32) { ... }
    // fn mouse_wheel_event(&mut self, _x: i32, _y: i32) { ... }
    // fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32) { ... }
    // fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) { ... }
    // fn quit_event(&mut self) -> bool { ... }
}
