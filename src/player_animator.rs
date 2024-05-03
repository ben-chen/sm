use specs::{Join, ReadStorage, System, WriteStorage};

use crate::{PhysicsData, Sprite, PlayerState, PlayerStatus, Direction};

pub struct PlayerAnimator;

impl<'a> System<'a> for PlayerAnimator {
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
                PlayerStatus::Attacking => 768,
            };
            sprite.animation_rate = match player_state.status {
                PlayerStatus::Idle => 5,
                PlayerStatus::Running => if physics_data.h_speed.abs() > 6 { 1 } else { 2 },
                PlayerStatus::Blocking => 5,
                PlayerStatus::Jumping => 1,
                PlayerStatus::Hitstun => 3,
                PlayerStatus::Blockstun => 2,
                PlayerStatus::Attacking => 3,
            };

            sprite.flip = match player_state.facing {
                Direction::Left => true,
                Direction::Right => false,
            };

            let old_spritesheet = sprite.spritesheet;
            sprite.spritesheet = player_state.status.into();
            if old_spritesheet != sprite.spritesheet {
                sprite.current.set_x(0);
            }
        }
    }
}
