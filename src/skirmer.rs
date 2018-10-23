use ncollide2d::{
    shape::{Cuboid, ShapeHandle},
    world::{CollisionGroups, GeometricQueryType},
};

use nalgebra::Isometry2;
use specs::{Entity, World};

use crate::{
    Vector2, CollisionWorld,
    game::PLAYER_COLLISION_GROUP,
    item::{Weapon, Item, ItemFactory},
    components::*,
    map::{SkirmMap, MapPoint, MapError},
};

pub enum SkirmerType {
    Fighter,
    Sniper,
}

pub struct SkirmerFactory;

impl SkirmerFactory {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn create_skirmer(
        &self,
        tile_x: i32,
        tile_y: i32,
        skirmer: &SkirmerType,
        item_factory: &ItemFactory,
        map: &mut SkirmMap,
        world: &mut World
    ) -> Result<Entity, MapError> {
        // let (weapon, items) = self.get_skirmer_items(skirmer, item_factory);

        let tile_point = MapPoint::new(tile_x, tile_y);
        let (x, y) = tile_point.as_float_coord_tuple();

        let ent = world.create_entity()
            .with(PositionComp::new(x, y))
            .with(AnimComp::new(String::from("default"), true))
            .with(SpriteComp::new(String::from("green_box")))
            .with(StatsComp::default())
            .with(StateComp::new())
            .with(PhysicsComp::new(PhysicsType::Moveable, nalgebra::zero()))
            .build();

        // Player collision info
        let shape = Cuboid::new(Vector2::new(12.0, 12.0));
        let mut group = CollisionGroups::new();
        group.set_membership(&[PLAYER_COLLISION_GROUP]);
        group.set_blacklist(&[PLAYER_COLLISION_GROUP]);
        let query_type = GeometricQueryType::Contacts(0.0, 0.0);

        let collider = {
            let mut collide_world = world.write_resource::<CollisionWorld>();
            let player_handle = collide_world.add(
                Isometry2::new(Vector2::new(0.0, -6.0), nalgebra::zero()),
                ShapeHandle::new(shape.clone()),
                group,
                query_type,
                ent,
            );

            CollideComp {
                handle: player_handle,
            }
        };

        world.write::<CollideComp>().insert(ent, collider);
        map.add_occupant(ent, tile_point).map(|()| ent)
    }

    fn get_skirmer_items(&self, skirmer: &SkirmerType, factory: &ItemFactory) -> (Weapon, Vec<Item>) {
        match skirmer {
            &SkirmerType::Fighter => {
                let weapon = factory.get_weapon(".22 Rifle");
                (weapon, vec![])
            },
            &SkirmerType::Sniper => {
                let weapon = factory.get_weapon(".22 Rifle");
                (weapon, vec![])
            }
        }
    }
}
