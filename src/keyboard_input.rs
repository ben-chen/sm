use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

use sm::{Direction, Input, MovementStats, PhysicsData, Player1, PlayerState, PlayerStatus};

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadStorage<'a, Player1>,
        ReadExpect<'a, Input>,
        WriteStorage<'a, PhysicsData>,
        ReadStorage<'a, MovementStats>,
        WriteStorage<'a, PlayerState>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        for (physics_data, movement_stats, player_state) in
            (&mut data.2, &data.3, &mut data.4).join()
        {
            match &*data.1 {
                Input::Jump => match player_state.status {
                    PlayerStatus::Idle | PlayerStatus::Running => {
                        player_state.status = PlayerStatus::Jumping;
                        physics_data.v_speed = -(movement_stats.jump_power as i32);
                        physics_data.v_acceleration = movement_stats.gravity as i32;
                    }
                    _ => (),
                },
                Input::Move(Direction::Left) => {
                    physics_data.h_acceleration =
                        if player_state.status != PlayerStatus::Jumping {
                            if physics_data.h_speed > -(movement_stats.max_speed as i32) {
                                -(movement_stats.acceleration as i32)
                            } else {
                                0
                            }
                        } else {
                            if physics_data.h_speed > -(movement_stats.air_max_speed as i32) {
                                -(movement_stats.air_acceleration as i32)
                            } else {
                                0
                            }
                        };
                }
                Input::Move(Direction::Right) => {
                    physics_data.h_acceleration =
                        if player_state.status != PlayerStatus::Jumping {
                            if physics_data.h_speed < movement_stats.max_speed as i32 {
                                movement_stats.acceleration as i32
                            } else {
                                0
                            }
                        } else {
                            if physics_data.h_speed < movement_stats.air_max_speed as i32 {
                                movement_stats.air_acceleration as i32
                            } else {
                                0
                            }
                        };
                }
                Input::Stop => {
                    physics_data.h_acceleration = match physics_data.h_speed {
                        i32::MIN..=-1 => {
                            movement_stats.friction.min(-physics_data.h_speed as u32) as i32
                        }
                        1..=i32::MAX => {
                            -(movement_stats.friction.min(physics_data.h_speed as u32) as i32)
                        }
                        0 => 0,
                    };
                }
            };

            physics_data.h_speed = physics_data.h_speed.clamp(
                -(movement_stats.max_speed as i32),
                movement_stats.max_speed as i32,
            );

            if player_state.status != PlayerStatus::Jumping {
                if physics_data.h_speed == 0 && physics_data.v_speed == 0 {
                    player_state.status = PlayerStatus::Idle;
                } else if physics_data.h_speed != 0 {
                    player_state.status = PlayerStatus::Running;
                }
            }

            if physics_data.h_speed > 0 {
                player_state.facing = Direction::Right;
            } else if physics_data.h_speed < 0 {
                player_state.facing = Direction::Left;
            }

            if physics_data.position.y() > 0 {
                physics_data.v_speed = 0;
                physics_data.v_acceleration = 0;
                physics_data.position.y = 0;
                if player_state.status == PlayerStatus::Jumping {
                    player_state.status = PlayerStatus::Idle;
                }
            }
        }
    }
}
