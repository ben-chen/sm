use specs::{Join, ReadStorage, System, WriteStorage};

use crate::{CollisionMask, CollisionStatus, PhysicsData, Sprite};

pub struct Collider;

impl<'a> System<'a> for Collider {
    type SystemData = (
        WriteStorage<'a, PhysicsData>,
        ReadStorage<'a, CollisionMask>,
        WriteStorage<'a, CollisionStatus>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let collision_masks = data.1.join().map(|cm| cm).collect::<Vec<_>>();
        let positions = data.0.join().map(|pd| pd.position).collect::<Vec<_>>();
        let mut collision_statuses = (&mut data.2).join().map(|cs| cs).collect::<Vec<_>>();
        let sprites = (&mut data.3).join().map(|s| s).collect::<Vec<_>>();

        for cs in collision_statuses.iter_mut() {
            cs.0 = false;
        }
        for i in 0..collision_masks.len() {
            for j in i + 1..collision_masks.len() {
                let colliding =
                    collision_masks[i].check(positions[i], collision_masks[j], positions[j]);
                collision_statuses[i].0 |= colliding;
                collision_statuses[j].0 |= colliding;
            }
        }
        for (i, sprite) in sprites.into_iter().enumerate() {
            sprite.glow = collision_statuses[i].0;
        }
    }
}
