extern crate ggez;
extern crate specs;
extern crate serde;
extern crate serde_yaml;
extern crate ascii;
extern crate pathfinding;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate specs_derive;

use ggez::{conf, ContextBuilder};
use ggez::event::run;

mod asset_storage;
mod systems;
mod components;
mod game;
mod resources;
mod rendering;
mod skirmer;
mod item;
mod skirmmap;
mod input;
mod gui;

use game::Game;

fn main() {
    let cb = ContextBuilder::new("skirm", "jupart")
        .window_setup(conf::WindowSetup::default()
            .title("skirm")
        )
        .window_mode(conf::WindowMode::default()
            .dimensions(250, 450)
        );

    let ctx = &mut cb.build().unwrap();

    match Game::new(ctx) {
        Err(e) => {
            println!("Could not load!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered: {}", e);
            } else {
                println!("Exited cleanly.");
            }
        }
    }
}
