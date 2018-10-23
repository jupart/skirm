use ggez::{timer, event, graphics, Context};
use ggez::event::{Keycode, Mod, MouseButton};
use ggez::graphics::{Rect};
use specs::{World, Dispatcher, DispatcherBuilder, RunNow, Entity};

use std::collections::HashMap;
use std::time::Duration;

use crate::{
    Point2, CollisionWorld, CollisionObject,
    asset_storage::AssetStorage,
    camera::Camera,
    components::*,
    systems::*,
    resources::DeltaTime,
    input::PlayerInputState,
    item::ItemFactory,
    skirmer::{SkirmerFactory, SkirmerType::Fighter},
    map::{MapPoint, SkirmMap},
    // gui::{Gui},
    visual_effects::{GunshotEffect, GunshotEffects},
};

use crate::SkirmResult;

pub const PLAYER_COLLISION_GROUP: usize = 1;
pub const TILE_COLLISION_GROUP: usize = 2;

pub struct Game<'a, 'b> {
    world: World,
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

        info!("Create collision world");
        let collide_world: CollisionWorld = CollisionWorld::new(0.02);
        world.add_resource(collide_world);

        let mut map = SkirmMap::load("./resources/maps/test.skirm_map", &mut world)?;

        asset_storage.load_images(ctx)?;
        asset_storage.load_animations()?;
        asset_storage.load_sounds(ctx)?;

        let mut ent1_sounds = HashMap::new();
        ent1_sounds.insert(SoundType::Move, ("sine", true));

        info!("Create entities");
        let p1_ent = skirmer_factory.create_skirmer(2, 2, &Fighter, &item_factory, &mut map, &mut world).unwrap();

        let gunshot_effects: Vec<GunshotEffect> = Vec::new();

        info!("Create camera");
        let camera = Camera::new(250, 450);

        info!("Add specs shared resources");
        world.add_resource(asset_storage);
        world.add_resource(DeltaTime { delta: Duration::new(0, 0) });
        world.add_resource(PlayerInputState::new(p1_ent));
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
            p1_ent,
            // gui,
            dispatcher,
            has_focus: true,
            paused: false,
            camera,
        })
    }

    fn _draw_effects(&self, ctx: &mut Context, gun_effects: &mut Vec<GunshotEffect>) {
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

        self.handle_collisions();
        self.update_camera(ctx);

        info!("<- Dispatch the specs systems");
        self.dispatcher.dispatch(&self.world.res);
        info!("-> Dispatch the specs systems");

        // Perform specs maintenance, removing entities, etc.
        self.world.maintain();
    }

    fn update_camera(&mut self, ctx: &mut Context) {
        let time = self.world.read_resource::<DeltaTime>();
        let pos_components = self.world.read::<PositionComp>();
        let player_input = self.world.read_resource::<PlayerInputState>();
        let player_pos = pos_components.get(player_input.ent).unwrap();

        self.camera.focus = Some(Point2::new(player_pos.x, player_pos.y));
        self.camera.update_center(time.as_dt());
    }

    fn print_fps_to_info(&self, ctx: &mut Context) {
        info!("FPS: {}", timer::get_fps(ctx));
    }

    fn handle_collisions(&self) {
        info!("<- Checking collisions");
        let mut col_world = self.world.write_resource::<CollisionWorld>();
        let mut state_comps = self.world.write::<StateComp>();
        col_world.update();

        // Save and reuse the same vec each run of the loop so we only allocate once.
        let contacts_list = &mut Vec::new();
        for e in col_world.contact_events() {
            contacts_list.clear();
            match e {
                ncollide2d::events::ContactEvent::Started(cobj_handle1, cobj_handle2) => {
                    if let Some(pair) = (&*col_world).contact_pair(*cobj_handle1, *cobj_handle2) {
                        println!("Starting collision between {:?} and {:?}", cobj_handle1, cobj_handle2);
                        pair.contacts(contacts_list);
                        let cobj1 = col_world.collision_object(*cobj_handle1).expect("Invalid collision object handle");
                        let cobj2 = col_world.collision_object(*cobj_handle2).expect("Invalid collision object handle");

                        let mut do_collision = |cobj1: &CollisionObject, cobj2: &CollisionObject| {
                            let entity = cobj1.data();
                            if cobj2.collision_groups().is_member_of(TILE_COLLISION_GROUP) {
                                let entity_state = state_comps.get_mut(*entity).expect("Are collision groups blacklisted correctly?");
                                entity_state.on_ground = true;
                            }
                        };
                        do_collision(cobj1, cobj2);
                        do_collision(cobj2, cobj1);
                    }
                }
                ncollide2d::events::ContactEvent::Stopped(cobj_handle1, cobj_handle2) => {
                    if let Some(pair) = (&*col_world).contact_pair(*cobj_handle1, *cobj_handle2) {
                        println!("Finishing collision between {:?} and {:?}", cobj_handle1, cobj_handle2);
                        pair.contacts(contacts_list);
                        let cobj1 = col_world.collision_object(*cobj_handle1).expect("Invalid collision object handle");
                        let cobj2 = col_world.collision_object(*cobj_handle2).expect("Invalid collision object handle");

                        // Get the entities out of the collision data
                        let mut do_collision = |cobj1: &CollisionObject, cobj2: &CollisionObject| {
                            let entity = cobj1.data();
                            if cobj2.collision_groups().is_member_of(TILE_COLLISION_GROUP) {
                                let entity_state = state_comps.get_mut(*entity).expect("Are collision groups blacklisted correctly?");
                                entity_state.on_ground = false;
                            }
                        };
                        do_collision(cobj1, cobj2);
                        do_collision(cobj2, cobj1);
                    }
                }
            }
        }
        info!("-> Checking collisions");
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
        let player_input = self.world.read_resource::<PlayerInputState>();
        let assets = self.world.read_resource::<AssetStorage>();
        let map = self.world.read_resource::<SkirmMap>();
        let mut gun_effects = self.world.write_resource::<GunshotEffects>();

        let pos_components = self.world.read::<PositionComp>();
        let player_pos = pos_components.get(player_input.ent).unwrap();
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
        let mut input = self.world.write_resource::<PlayerInputState>().input;

        match keycode {
            Keycode::W => input.up.set(true),
            Keycode::S => input.down.set(true),
            Keycode::A => input.left.set(true),
            Keycode::D => input.right.set(true),
            _ => ()
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut input = self.world.write_resource::<PlayerInputState>().input;

        match keycode {
            Keycode::W => input.up.set(false),
            Keycode::S => input.down.set(false),
            Keycode::A => input.left.set(false),
            Keycode::D => input.right.set(false),
            _ => ()
        }
    }

    fn focus_event(&mut self, _ctx: &mut Context, has_focus: bool) {
        self.has_focus = has_focus;
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        // let mut input = self.world.write_resource::<InputState>();

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
