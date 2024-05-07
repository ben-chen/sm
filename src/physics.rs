use specs::{Join, System, WriteStorage};

use crate::PhysicsData;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = WriteStorage<'a, PhysicsData>;

    fn run(&mut self, mut data: Self::SystemData) {
        for physics_data in (&mut data).join() {
            physics_data.speed.x += physics_data.acceleration.x;
            physics_data.speed.y += physics_data.acceleration.y;
            physics_data.position = physics_data.position.offset(physics_data.speed.x, physics_data.speed.y);
        }
    }
}
