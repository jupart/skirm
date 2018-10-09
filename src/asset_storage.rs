use std::fs::DirEntry;

use ggez::{audio, Context};
use ggez::graphics::{Image, Font, Text};
use std::collections::HashMap;

use crate::SkirmResult;
use crate::map::tile::TileType;

pub struct AssetStorage {
    pub images: HashMap<String, Image>,
    pub sounds: HashMap<String, audio::Source>,
    pub tiles: HashMap<TileType, String>,
    pub font: Font,
    pub glyphs: HashMap<char, Text>,
}

impl AssetStorage {
    pub fn new(ctx: &mut Context) -> SkirmResult<Self> {
        let images = HashMap::new();
        let sounds = HashMap::new();

        let mut tiles = HashMap::new();
        tiles.insert(TileType::Ground, "blue_box".to_string());

        let font = Font::new(ctx, "/fonts/FiraMono-Regular.ttf", 11)?;
        let mut glyphs = HashMap::new();

        // Leading space is intentional
        for c in " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()[]{}".chars() {
            glyphs.insert(c, Text::new(ctx, &c.to_string(), &font).unwrap());
        }

        Ok(Self {
            images,
            sounds,
            tiles,
            font,
            glyphs,
        })
    }

    pub fn load_sounds(&mut self, _ctx: &mut Context) -> SkirmResult {
        // use std::fs;
        // let dirs = fs::read_dir("./resources/sounds")?;

        // for directory in dirs {
            // let (name, ggez_path_str) = self.get_resource(&directory.unwrap());
            // let sound = audio::Source::new(ctx, ggez_path_str)?;
            // self.sounds.insert(name, sound);
        // }

        Ok(())
    }

    fn get_resource(&self, dir: &DirEntry) -> (String, String) {
        // What a mess.. TODO figure out how to fix this
        let path_str = String::from(dir.path().to_str().unwrap());
        let ggez_path_str = String::from(path_str.split("./resources").nth(1).unwrap());
        let ext_name = path_str.split('/').nth(3).unwrap();
        let name = String::from(ext_name.split('.').nth(0).unwrap());
        (name, ggez_path_str)
    }

    pub fn load_images(&mut self, ctx: &mut Context) -> SkirmResult {
        use std::fs;
        let dirs = fs::read_dir("./resources/images")?;

        for path in dirs {
            let (name, ggez_path_str) = self.get_resource(&path.unwrap());
            self.images.insert(name, Image::new(ctx, ggez_path_str).unwrap());
        }

        Ok(())
    }

    pub fn play(&self, sound_name: &'static str) {
        let sound = &self.sounds[sound_name];
        sound.play().unwrap();
    }
}
