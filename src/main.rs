// #![allow(dead_code)]

extern crate ggez;
extern crate specs;
extern crate serde;
extern crate ron;
extern crate ascii;
extern crate pathfinding;
extern crate line_drawing;
extern crate env_logger;
extern crate ncollide2d;
extern crate nalgebra;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate log;

use ggez::{conf, ContextBuilder, GameResult};
use ggez::event;

mod asset_storage;
mod systems;
mod components;
mod game;
mod resources;
mod skirmer;
mod item;
mod map;
mod input;
mod gui;
mod visual_effects;
mod camera;

use crate::game::Game;

type SkirmResult<T = ()> = GameResult<T>;
type CollisionWorld = ncollide2d::world::CollisionWorld<f32, specs::Entity>;
type CollisionObject = ncollide2d::world::CollisionObject<f32, specs::Entity>;
type Point2 = nalgebra::Point2<f32>;
type Vector2 = nalgebra::Vector2<f32>;

fn main() -> SkirmResult {
    env_logger::Builder::from_default_env().default_format_timestamp(false).init();
    enable_backtrace();

    info!("Creating Context");
    let mut ctx = ContextBuilder::new("aok", "jupart")
        .window_setup(conf::WindowSetup::default().title("aok"))
        .window_mode(conf::WindowMode::default().dimensions(250, 450))
        .add_resource_path("resources")
        .build()
        .expect("Something went wrong building the game's context!");

    info!("Creating Game");
    let mut game = Game::new(&mut ctx).expect("We screwed up creating the initial game state!");

    info!("Starting main loop");
    event::run(&mut ctx, &mut game)
}

fn enable_backtrace() {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
}
