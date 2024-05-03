use specs::{Join, System, WriteStorage};

use crate::Sprite;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = WriteStorage<'a, Sprite>;

    fn run(&mut self, mut data: Self::SystemData) {
        for sprite in (&mut data).join() {
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
