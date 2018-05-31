#![allow(dead_code)]

extern crate ggez;
extern crate specs;
extern crate serde;
extern crate ron;
extern crate ascii;
extern crate pathfinding;
extern crate line_drawing;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate specs_derive;

use ggez::{conf, ContextBuilder, GameResult};
use ggez::event;

mod asset_storage;
mod systems;
mod components;
mod game;
mod resources;
mod rendering;
mod skirmer;
mod item;
mod map;
mod input;
mod gui;
mod visual_effects;

use game::Game;

type SkirmResult<T = ()> = GameResult<T>;

fn main() -> SkirmResult {
    let mut ctx = ContextBuilder::new("skirm", "jupart")
        .window_setup(conf::WindowSetup::default().title("skirm"))
        .window_mode(conf::WindowMode::default().dimensions(250, 450))
        .add_resource_path("resources")
        .build()
        .expect("Something went wrong building the game's context!");

    let mut game = Game::new(&mut ctx).expect("We screwed up creating the initial game state!");
    event::run(&mut ctx, &mut game)
}
