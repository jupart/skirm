use specs::{Fetch, FetchMut, System, WriteStorage, ReadStorage, Join};
use nalgebra::Translation;
use ggez::graphics;

use crate::{
    CollisionWorld,
    Vector2,
    Point2,
    resources::DeltaTime,
    components::*
};

pub struct PhysicsSys;
impl<'a> System<'a> for PhysicsSys{
    type SystemData = (
        WriteStorage<'a, PositionComp>,
        WriteStorage<'a, PhysicsComp>,
        WriteStorage<'a, CollideComp>,
        ReadStorage<'a, StateComp>,
        FetchMut<'a, CollisionWorld>,
        Fetch<'a, DeltaTime>,
    );

    fn run(&mut self, (mut pos, mut physics, mut collide, state, mut world, time): Self::SystemData) {
        let dt = time.as_dt();
        for (pos, p, s, c) in (&mut pos, &mut physics, &state, &mut collide).join() {
            // Apply gravity
            if !s.is_on_ground() {
                p.acceleration += Vector2::new(0.0, 10.0 * dt);
            }

            // Update pos
            p.velocity += p.acceleration;
            p.acceleration = Vector2::new(0.0, 0.0);
            let new_position = {
                let obj = world.collision_object(c.handle).unwrap();
                let mut pos = obj.position().clone();
                pos.append_translation_mut(&Translation::from_vector(p.velocity));
                pos
            };
            world.set_position(c.handle, new_position);

            let position = ggez_collision_object_pos(&world, c);
            pos.x = position.x;
            pos.y = position.y;
        }
    }
}

fn ggez_collision_object_pos(world: &CollisionWorld, collider: &CollideComp) -> graphics::Point2 {
    let point = collision_object_pos(world, collider);
    graphics::Point2::new(point.x, point.y)
}

fn collision_object_pos(world: &CollisionWorld, collider: &CollideComp) -> Point2 {
    let collision_object = world.collision_object(collider.handle).unwrap();
    let isometry = collision_object.position();
    let new_pos = Point2::new(isometry.translation.vector.x, isometry.translation.vector.y);
    new_pos
}
