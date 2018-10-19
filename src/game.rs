use ggez::{timer, event, graphics, Context};
use ggez::event::{Keycode, Mod, MouseButton};
use ggez::graphics::{Rect};
use specs::{World, Dispatcher, DispatcherBuilder, RunNow, Entity};

use std::collections::HashMap;
use std::time::Duration;

use crate::{
    Point2, CollisionWorld,
    asset_storage::AssetStorage,
    camera::Camera,
    components::*,
    systems::*,
    resources::DeltaTime,
    input::SkirmerInput,
    item::ItemFactory,
    skirmer::{SkirmerFactory, SkirmerType::Fighter},
    map::{MapPoint, SkirmMap},
    // gui::{Gui},
    visual_effects::{GunshotEffect, GunshotEffects},
};

use crate::SkirmResult;

pub const PLAYER_COLLISION_GROUP: usize = 1;

pub struct Game<'a, 'b> {
    world: World,
    skirmers: Vec<Entity>,
    p1_ent: Entity,
    // pub gui: Gui,
    dispatcher: Dispatcher<'a, 'b>,
    has_focus: bool,
    paused: bool,
    camera: Camera,
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
        asset_storage.load_animations()?;
        asset_storage.load_sounds(ctx)?;

        let mut ent1_sounds = HashMap::new();
        ent1_sounds.insert(SoundType::Move, ("sine", true));

        info!("Create collision world");
        let collide_world: CollisionWorld = CollisionWorld::new(0.02);
        world.add_resource(collide_world);

        info!("Create entities");
        let p1_ent = skirmer_factory.create_skirmer(2, 2, &Fighter, &item_factory, &mut map, &mut world).unwrap();
        let skirmers = vec![p1_ent];

        let gunshot_effects: Vec<GunshotEffect> = Vec::new();

        info!("Create camera");
        let camera = Camera::new(250, 450);

        info!("Add specs shared resources");
        world.add_resource(asset_storage);
        world.add_resource(DeltaTime { delta: Duration::new(0, 0) });
        world.add_resource(SkirmerInput::new(p1_ent));
        world.add_resource(map);
        world.add_resource(GunshotEffects { effects: gunshot_effects });

        info!("Build system dispatcher");
        let dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .add(PlanSys, "plan", &[])
            .add(StateSys, "act", &["plan"])
            .add(StatsSys, "stats", &["act"])
            .add(SoundSys, "sound", &["act"])
            .add(AnimSys, "anim", &["act"])
            .add(PhysicsSys, "physics", &[])
            .build();

        // info!("Build gui");
        // let gui = Gui::new(ctx);

        graphics::set_background_color(ctx, BLACK);

        Ok(Self {
            world,
            skirmers,
            p1_ent,
            // gui,
            dispatcher,
            has_focus: true,
            paused: false,
            camera,
        })
    }

    fn draw_effects(&self, ctx: &mut Context, gun_effects: &mut Vec<GunshotEffect>) {
        for effect in &mut *gun_effects {
            effect.draw(ctx);
        }
        gun_effects.retain(|effect| !effect.finished());
    }

    fn update_game(&mut self, ctx: &mut Context) {
        self.print_fps_to_info(ctx);

        // Update delta time for frame
        let dt = &timer::get_delta(ctx);
        self.world.write_resource::<DeltaTime>().delta = *dt;

        self.update_camera(ctx);

        info!("  <- Dispatch the specs systems");
        self.dispatcher.dispatch(&self.world.res);
        info!("  -> Dispatch the specs systems");

        // Perform specs maintenance, removing entities, etc.
        self.world.maintain();
    }

    fn update_camera(&mut self, ctx: &mut Context) {
        let time = self.world.read_resource::<DeltaTime>();
        let pos_components = self.world.read::<PositionComp>();
        let input = self.world.read_resource::<SkirmerInput>();
        let player_pos = pos_components.get(input.ent).unwrap();

        self.camera.focus = Some(Point2::new(player_pos.x, player_pos.y));
        self.camera.update_center(time.as_dt());
    }

    fn print_fps_to_info(&self, ctx: &mut Context) {
        info!("FPS: {}", timer::get_fps(ctx));
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
            let mut rs = RenderSys::new(ctx, &self.camera);
            rs.run_now(&self.world.res);
        }

        // Effects rendering
        // self.draw_effects(ctx, &mut gun_effects.effects);

        // Gui rendering
        // self.gui.draw(&pos, &input, &assets, &*map, ctx);

        graphics::present(ctx);

        timer::yield_now();
        info!("-> Draw Game");
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut input = self.world.write_resource::<SkirmerInput>();

        match keycode {
            Keycode::W => input.up = true,
            Keycode::S => input.down = true,
            Keycode::A => input.left = true,
            Keycode::D => input.right = true,
            _ => ()
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut input = self.world.write_resource::<SkirmerInput>();

        match keycode {
            Keycode::W => input.up = false,
            Keycode::S => input.down = false,
            Keycode::A => input.left = false,
            Keycode::D => input.right = false,
            _ => ()
        }
    }

    fn focus_event(&mut self, _ctx: &mut Context, has_focus: bool) {
        self.has_focus = has_focus;
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        // let mut input = self.world.write_resource::<SkirmerInput>();

        // if self.gui.handle_click(Point2::new(x as f32, y as f32)) {
        //     return
        // }

        // match button {
        //     MouseButton::Left => input.select_point(x, y),
        //     MouseButton::Right => (),
        //     MouseButton::Middle => (),
        //     _ => (),
        // }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
        // if self.gui.handle_release(Point2::new(x as f32, y as f32)) {
        //     return
        // }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        let rect = Rect::new(0.0, 0.0, width as f32, height as f32);
        graphics::set_screen_coordinates(ctx, rect).unwrap();
        self.camera.update_screen(width as f32, height as f32);

        // self.gui.window_resized(width, height);
    }

    // fn mouse_motion_event(&mut self, _state: MouseState, _x: i32, _y: i32, _xrel: i32, _yrel: i32) { ... }
    // fn mouse_wheel_event(&mut self, _x: i32, _y: i32) { ... }
    // fn controller_button_down_event(&mut self, _btn: Button, _instance_id: i32) { ... }
    // fn controller_button_up_event(&mut self, _btn: Button, _instance_id: i32) { ... }
    // fn quit_event(&mut self) -> bool { ... }
}
