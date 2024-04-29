use specs::{Join, System, WriteStorage};

use crate::PhysicsData;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = WriteStorage<'a, PhysicsData>;

    fn run(&mut self, mut data: Self::SystemData) {
        for physics_data in (&mut data).join() {
            physics_data.h_speed += physics_data.h_acceleration;
            physics_data.v_speed += physics_data.v_acceleration;
            physics_data.position = physics_data.position.offset(physics_data.h_speed, physics_data.v_speed);
        }
    }
}
