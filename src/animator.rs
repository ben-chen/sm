use specs::{Join, ReadStorage, System, WriteStorage};

use sm::{PhysicsData, Sprite, PlayerState, PlayerStatus, Direction};

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (WriteStorage<'a, Sprite>, ReadStorage<'a, PhysicsData>, WriteStorage<'a, PlayerState>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (sprite, physics_data, player_state) in (&mut data.0, &data.1, &mut data.2).join() {
            sprite.wrap = match player_state.status {
                PlayerStatus::Idle => 768,
                PlayerStatus::Running => 1024,
                PlayerStatus::Blocking => 256,
                PlayerStatus::Jumping => 1536,
                PlayerStatus::Hitstun => 256,
                PlayerStatus::Blockstun => 256,
            };
            sprite.animation_rate = match player_state.status {
                PlayerStatus::Idle => 5,
                PlayerStatus::Running => 1,
                PlayerStatus::Blocking => 5,
                PlayerStatus::Jumping => 1,
                PlayerStatus::Hitstun => 3,
                PlayerStatus::Blockstun => 2,
            };

            sprite.flip = match player_state.facing {
                Direction::Left => true,
                Direction::Right => false,
            };

            sprite.spritesheet = player_state.status.into();

            sprite.counter += 1;
            if sprite.counter > sprite.animation_rate {
                sprite.current.set_x(
                    ((sprite.current.x() as u32 + sprite.current.width()) % sprite.wrap) as i32,
                );
                sprite.counter = 0;
            }
            sprite.current.x %= sprite.wrap as i32;
        }
    }
}
