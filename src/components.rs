use sdl2::rect::{Point, Rect};
use specs::prelude::{Component, VecStorage};
use specs_derive::Component;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
}

impl From<Direction> for bool {
    fn from(f: Direction) -> bool {
        match f {
            Direction::Left => true,
            Direction::Right => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus {
    Idle,
    Running,
    Blocking,
    Jumping,
    Hitstun,
    Blockstun,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    Move(Direction),
    Stop,
    Jump,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PhysicsData {
    pub position: Point,
    pub h_speed: i32,
    pub v_speed: i32,
    pub h_acceleration: i32,
    pub v_acceleration: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementStats {
    pub max_speed: u32,
    pub acceleration: u32,
    pub friction: u32,
    pub gravity: u32,
    pub jump_power: u32,
    pub air_acceleration: u32,
    pub air_max_speed: u32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize, // index into textures array
    pub current: Rect,
    pub wrap: u32,
    pub flip: bool,
    pub counter: u32,
    pub animation_rate: u32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player1;

#[derive(Component)]
#[storage(VecStorage)]
pub struct PlayerState {
    pub status: PlayerStatus,
    pub facing: Direction,
}

impl From<PlayerStatus> for usize {
    // Get textures index from status
    fn from(player_status: PlayerStatus) -> usize {
        match player_status {
            PlayerStatus::Idle => 0,
            PlayerStatus::Running => 1,
            PlayerStatus::Blocking => 2,
            PlayerStatus::Jumping => 3,
            PlayerStatus::Hitstun => 4,
            PlayerStatus::Blockstun => 5,
        }
    }
}
