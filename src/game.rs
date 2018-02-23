use ggez::{timer, event, graphics, Context, GameResult};
use ggez::event::{Axis, Keycode, Mod, MouseButton};
use specs::{World, Dispatcher, DispatcherBuilder, RunNow, Index};

use std::collections::HashMap;
use std::time::Duration;

use asset_storage::AssetStorage;
use components::*;
use systems::*;
use resources::DeltaTime;
use input::{PlayerInput, PendingCommand};
// use rendering::RenderType;
use item::ItemFactory;
use skirmer::SkirmerFactory;
use skirmmap::SkirmMap;

pub struct Game<'a, 'b> {
    pub world: World,
    pub player_count: usize,
    pub player1_id: Index,
    pub player2_id: Index,
    pub player3_id: Index,
    pub player4_id: Index,
    pub dispatcher: Dispatcher<'a, 'b>,
    pub has_focus: bool,
    pub paused: bool,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> GameResult<Game<'a, 'b>> {
        let mut world = World::new();

        let pc = 0;

        register_components(&mut world);

        let mut asset_storage = AssetStorage::new(ctx)?;
        let mut item_factory = ItemFactory::new()?;
        let skirmer_factory = SkirmerFactory::new();

        asset_storage.load_images(ctx)?;
        asset_storage.load_sounds(ctx)?;

        let mut ent1_sounds = HashMap::new();
        ent1_sounds.insert(SoundType::Move, ("sine", true));

        // Create entities
        let player1_id = skirmer_factory.create_skirmer(64.0, 14.0, &mut world);

        // Add specs shared resources
        world.add_resource::<AssetStorage>(asset_storage);
        world.add_resource(DeltaTime { delta: Duration::new(0, 0) });
        world.add_resource(PlayerInput::new(player1_id));
        world.add_resource(SkirmMap::load("./resources/maps/test.skirm_map")?);

        // Dispatch systems
        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .add(ActionSys, "action", &[])
            .add(PlayerInputSys, "player_input", &[])
            .add(PositionSys, "position", &[])
            .add(SoundSys, "sound", &[])
            .build();

        Ok(Game {
            world,
            player_count: pc,
            player1_id,
            player2_id: 0,
            player3_id: 0,
            player4_id: 0,
            dispatcher,
            has_focus: true,
            paused: false,
        })
    }
}

impl<'a, 'b> event::EventHandler for Game<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.has_focus && !self.paused {
            let dt = &timer::get_delta(ctx);
            self.world.write_resource::<DeltaTime>().delta = *dt;
            self.dispatcher.dispatch(&self.world.res);
            self.world.maintain();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        {
            let mut rs = RenderSys::new(ctx);
            rs.run_now(&self.world.res);
        }
        graphics::present(ctx);

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInput>();

        if repeat {
            return;
        }

        match keycode {
            Keycode::A => input.set_pending_command(PendingCommand::Attack),
            Keycode::M => input.set_pending_command(PendingCommand::Move),
            _ => ()
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) { }

    fn controller_axis_event(&mut self, _ctx: &mut Context, _axis: Axis, _value: i16, _instance_id: i32) { }

    fn focus_event(&mut self, _ctx: &mut Context, has_focus: bool) {
        self.has_focus = has_focus;
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        let mut input = self.world.write_resource::<PlayerInput>();
        match button {
            MouseButton::Left => input.select_point(x, y),
            MouseButton::Right => (),
            MouseButton::Middle => (),
            _ => (),
        }
    }

    // fn mouse_button_up_event(&mut self, _button: MouseButton, _x: i32, _y: i32) { ... }
    // fn mouse_motion_event(&mut self, _state: MouseState, _x: i32, _y: i32, _xrel: i32, _yrel: i32) { ... }
    // fn mouse_wheel_event(&mut self, _x: i32, _y: i32) { ... }
    // fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32) { ... }
    // fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) { ... }
    // fn quit_event(&mut self) -> bool { ... }
    // fn resize_event(&mut self, _ctx: &mut Context, _width: u32, _height: u32) { ... }
}
