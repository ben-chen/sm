use sdl2::keyboard::Scancode;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

use std::collections::HashSet;

use crate::{
    Direction, Input, InputBuffer, MovementStats, PhysicsData, Player1, PlayerState, PlayerStatus,
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
            physics_data.h_acceleration = 0;
            player_state.animation_counter += 1;
            match &inputs[0] {
                h if h.is_empty()
                    || player_state.status == PlayerStatus::Attacking
                    || h.contains(&Input::Attack) =>
                {
                    physics_data.h_acceleration = match player_state.status {
                        PlayerStatus::Jumping => match physics_data.h_speed {
                            h_speed if h_speed > movement_stats.air_max_speed as i32 => {
                                -((movement_stats.air_acceleration as i32).min(h_speed))
                            }
                            h_speed if h_speed < -(movement_stats.air_max_speed as i32) => {
                                (movement_stats.air_acceleration as i32).min(-h_speed)
                            }
                            _ => 0,
                        },
                        _ => match physics_data.h_speed {
                            h_speed if h_speed > 0 => {
                                -(movement_stats.friction.min(physics_data.h_speed as u32) as i32)
                            }
                            h_speed if h_speed < 0 => {
                                movement_stats.friction.min(-physics_data.h_speed as u32) as i32
                            }
                            _ => 0,
                        },
                    };
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
                        physics_data.v_speed = if inputs[1].contains(&Input::Crouch)
                            || inputs[2].contains(&Input::Crouch)
                            || inputs[3].contains(&Input::Crouch)
                        {
                            -(movement_stats.superjump_power as i32)
                        } else {
                            -(movement_stats.jump_power as i32)
                        };
                        physics_data.v_acceleration = movement_stats.gravity as i32;
                    }
                    _ => (),
                },
                h if h.contains(&Input::Move(Direction::Left)) => {
                    physics_data.h_acceleration = match player_state.status {
                        PlayerStatus::Jumping => {
                            if physics_data.h_speed > -(movement_stats.air_max_speed as i32) {
                                -(movement_stats.air_acceleration as i32)
                            } else {
                                0
                            }
                        }
                        _ => {
                            if physics_data.h_speed > -(movement_stats.max_speed as i32) {
                                -(movement_stats.acceleration as i32)
                            } else {
                                0
                            }
                        }
                    };
                }
                h if h.contains(&Input::Move(Direction::Right)) => {
                    physics_data.h_acceleration = match player_state.status {
                        PlayerStatus::Jumping => {
                            if physics_data.h_speed < movement_stats.air_max_speed as i32 {
                                movement_stats.air_acceleration as i32
                            } else {
                                0
                            }
                        }
                        _ => {
                            if physics_data.h_speed < movement_stats.max_speed as i32 {
                                movement_stats.acceleration as i32
                            } else {
                                0
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
            physics_data.h_speed = physics_data.h_speed.clamp(
                -(movement_stats.max_speed as i32),
                movement_stats.max_speed as i32,
            );

            // Update player state if grounded
            if ![PlayerStatus::Jumping, PlayerStatus::Attacking].contains(&player_state.status) {
                if physics_data.h_speed == 0 && physics_data.v_speed == 0 {
                    player_state.status = PlayerStatus::Idle;
                } else if physics_data.h_speed != 0 {
                    player_state.status = PlayerStatus::Running;
                }
            }

            // Check if attack finished
            if player_state.status == PlayerStatus::Attacking
                && player_state.animation_counter > crate::ATTACK_TOTAL_FRAMES
            {
                player_state.status = PlayerStatus::Idle;
            }

            match physics_data.h_speed {
                i32::MIN..=-1 => player_state.facing = Direction::Left,
                1..=i32::MAX => player_state.facing = Direction::Right,
                0 => (),
            }

            if physics_data.position.y() > 0 {
                physics_data.v_speed = 0;
                physics_data.v_acceleration = 0;
                physics_data.position.y = 0;
                if player_state.status == PlayerStatus::Jumping {
                    player_state.status = if physics_data.h_speed == 0 {
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
