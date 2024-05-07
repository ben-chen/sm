use sdl2::keyboard::Scancode;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

use std::collections::HashSet;

use crate::{
    Direction, Fi32, Input, InputBuffer, MovementStats, PhysicsData, Player1, PlayerState,
    PlayerStatus,
};

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadStorage<'a, Player1>,
        ReadExpect<'a, InputBuffer>,
        WriteStorage<'a, PhysicsData>,
        ReadStorage<'a, MovementStats>,
        WriteStorage<'a, PlayerState>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        for (physics_data, movement_stats, player_state) in
            (&mut data.2, &data.3, &mut data.4).join()
        {
            let inputs = data.1.get_all();
            physics_data.acceleration.x = Fi32::ZERO;
            player_state.animation_counter += 1;
            match &inputs[0] {
                h if h.is_empty()
                    || player_state.status == PlayerStatus::Attacking
                    || h.contains(&Input::Attack) =>
                {
                    physics_data.acceleration.x = match player_state.status {
                        PlayerStatus::Jumping => match physics_data.speed.x {
                            x_speed if x_speed > movement_stats.air_max_speed => {
                                -((movement_stats.air_acceleration).min(x_speed))
                            }
                            x_speed if x_speed < -(movement_stats.air_max_speed) => {
                                (movement_stats.air_acceleration).min(-x_speed)
                            }
                            _ => Fi32::ZERO,
                        },
                        _ => match physics_data.speed.x {
                            x_speed if x_speed.is_positive() => {
                                -(movement_stats.friction.min(physics_data.speed.x))
                            }
                            x_speed if x_speed.is_negative() => {
                                movement_stats.friction.min(-physics_data.speed.x)
                            }
                            _ => Fi32::ZERO,
                        },
                    };
                    dbg!(physics_data.speed.x);
                    if h.contains(&Input::Attack) {
                        if player_state.status != PlayerStatus::Attacking {
                            player_state.animation_counter = 0;
                        }
                        player_state.status = PlayerStatus::Attacking;
                    };
                }
                h if h.contains(&Input::Jump) => match player_state.status {
                    PlayerStatus::Idle | PlayerStatus::Running => {
                        player_state.status = PlayerStatus::Jumping;
                        physics_data.speed.y = if inputs[1].contains(&Input::Crouch)
                            || inputs[2].contains(&Input::Crouch)
                            || inputs[3].contains(&Input::Crouch)
                        {
                            -(movement_stats.superjump_power)
                        } else {
                            -(movement_stats.jump_power)
                        };
                    }
                    _ => (),
                },
                h if h.contains(&Input::Move(Direction::Left)) => {
                    physics_data.acceleration.x = match player_state.status {
                        PlayerStatus::Jumping => {
                            if physics_data.speed.x > -(movement_stats.air_max_speed) {
                                -(movement_stats.air_acceleration)
                            } else {
                                Fi32::ZERO
                            }
                        }
                        _ => {
                            if physics_data.speed.x > -movement_stats.max_speed {
                                -(movement_stats.acceleration)
                            } else {
                                Fi32::ZERO
                            }
                        }
                    };
                }
                h if h.contains(&Input::Move(Direction::Right)) => {
                    physics_data.acceleration.x = match player_state.status {
                        PlayerStatus::Jumping => {
                            if physics_data.speed.x < movement_stats.air_max_speed {
                                movement_stats.air_acceleration
                            } else {
                                Fi32::ZERO
                            }
                        }
                        _ => {
                            if physics_data.speed.x < movement_stats.max_speed {
                                movement_stats.acceleration
                            } else {
                                Fi32::ZERO
                            }
                        }
                    };
                }
                h if h.contains(&Input::Crouch) => {
                    // Crouch
                }
                _ => (),
            };

            // Clamp to max_speed
            physics_data.speed.x = physics_data
                .speed
                .x
                .clamp(-movement_stats.max_speed, movement_stats.max_speed);

            // Gravity
            if player_state.status == PlayerStatus::Jumping || physics_data.position.y < Fi32::ZERO
            {
                physics_data.acceleration.y = movement_stats.gravity;
            } else {
                physics_data.acceleration.y = Fi32::ZERO;
            }

            // Update player state if grounded
            if ![PlayerStatus::Jumping, PlayerStatus::Attacking].contains(&player_state.status) {
                if physics_data.speed.x.is_zero() && physics_data.speed.y.is_zero() {
                    player_state.status = PlayerStatus::Idle;
                } else if !physics_data.speed.x.is_zero() {
                    player_state.status = PlayerStatus::Running;
                }
            }

            // Check if attack finished
            if player_state.status == PlayerStatus::Attacking
                && player_state.animation_counter > crate::ATTACK_TOTAL_FRAMES
            {
                player_state.status = PlayerStatus::Idle;
            }

            if inputs[0].contains(&Input::Move(Direction::Left))
                && physics_data.speed.x.is_negative()
            {
                player_state.facing = Direction::Left;
            } else if inputs[0].contains(&Input::Move(Direction::Right))
                && physics_data.speed.x.is_positive()
            {
                player_state.facing = Direction::Right;
            }

            if (physics_data.position.y + physics_data.speed.y).is_positive() {
                physics_data.speed.y = Fi32::ZERO;
                physics_data.acceleration.y = Fi32::ZERO;
                physics_data.position.y = Fi32::ZERO;
                if player_state.status == PlayerStatus::Jumping {
                    player_state.status = if physics_data.speed.x == Fi32::ZERO {
                        PlayerStatus::Idle
                    } else {
                        PlayerStatus::Running
                    }
                }
            }

            // dbg!(player_state.status);
        }
    }
}

/// Controls: Map keyboard inputs to game inputs
pub fn get_input(keyboard_state: &sdl2::keyboard::KeyboardState) -> HashSet<Input> {
    let mut input: HashSet<Input> = keyboard_state
        .pressed_scancodes()
        .filter_map(|scancode| match scancode {
            Scancode::Left | Scancode::A => Some(Input::Move(Direction::Left)),
            Scancode::Right | Scancode::D => Some(Input::Move(Direction::Right)),
            Scancode::Down | Scancode::S => Some(Input::Crouch),
            Scancode::Up | Scancode::W | Scancode::Space => Some(Input::Jump),
            Scancode::U => Some(Input::Attack),
            Scancode::Escape | Scancode::X => Some(Input::Quit),
            _ => None,
        })
        .collect();

    //SOCD
    // Left + Right = Neutral
    if input.contains(&Input::Move(Direction::Left))
        && input.contains(&Input::Move(Direction::Right))
    {
        input.remove(&Input::Move(Direction::Left));
        input.remove(&Input::Move(Direction::Right));
    }
    // Down + Up = Up
    if input.contains(&Input::Crouch) && input.contains(&Input::Jump) {
        input.remove(&Input::Crouch);
    }

    input
}
