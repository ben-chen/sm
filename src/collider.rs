use specs::{Join, System, WriteStorage};

use crate::{CollisionData, Fi32, PhysicsData, PointFi32, Sprite};

pub struct Collider;

impl<'a> System<'a> for Collider {
    type SystemData = (
        WriteStorage<'a, PhysicsData>,
        WriteStorage<'a, CollisionData>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let mut collision_datas = (&mut data.1).join().collect::<Vec<_>>();
        let mut physics_datas = (&mut data.0).join().collect::<Vec<_>>();
        let sprites = (&mut data.2).join().collect::<Vec<_>>();

        for cd in &mut collision_datas {
            cd.status.0 = false;
            cd.repel_vector = PointFi32::new(0, 0);
        }
        for i in 0..collision_datas.len() {
            for j in i + 1..collision_datas.len() {
                let colliding = collision_datas[i].mask.check(
                    physics_datas[i].position,
                    &collision_datas[j].mask,
                    physics_datas[j].position,
                );
                collision_datas[i].status.0 |= colliding;
                collision_datas[j].status.0 |= colliding;
                if colliding {
                    collision_datas[i].repel_vector +=
                        (physics_datas[i].position - physics_datas[j].position).normalize();
                    collision_datas[j].repel_vector +=
                        (physics_datas[j].position - physics_datas[i].position).normalize();
                }
            }
        }
        for (i, sprite) in sprites.into_iter().enumerate() {
            sprite.glow = collision_datas[i].status.0;

            let speed_dot = physics_datas[i]
                .speed
                .dot(collision_datas[i].repel_vector)
                .min(Fi32::ZERO);
            physics_datas[i].speed -= collision_datas[i].repel_vector.normalize() * speed_dot;
            let acceleration_dot = physics_datas[i]
                .acceleration
                .dot(collision_datas[i].repel_vector)
                .min(Fi32::ZERO);
            physics_datas[i].acceleration -=
                collision_datas[i].repel_vector.normalize() * acceleration_dot;
        }
    }
}
