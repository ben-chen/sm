use sdl2::rect::{Point, Rect};
use specs::prelude::{Component, VecStorage};
use specs_derive::Component;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Left,
}

impl From<Direction> for bool {
    fn from(direction: Direction) -> bool {
        match direction {
            Direction::Left => true,
            Direction::Right => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlayerStatus {
    Idle,
    Running,
    Blocking,
    Jumping,
    Hitstun,
    Blockstun,
    Attacking,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Input {
    Move(Direction),
    Jump,
    Crouch,
    Attack,
    Quit,
}

#[derive(Component, Debug, Clone, Copy, Default)]
#[storage(VecStorage)]
pub struct Framerate(pub u32);

impl Framerate {
    pub fn new(fps: u32) -> Self {
        Framerate(fps)
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, fps: u32) {
        self.0 = fps;
    }
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
    pub superjump_power: u32,
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
    pub animation_counter: u32,
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
            PlayerStatus::Attacking => 6,
        }
    }
}

/// Fixed size ring buffer of length crate::COMMAND_BUFFER_SIZE
#[derive(Clone)]
pub struct InputBuffer {
    inner: Arc<Mutex<InputBufferInner>>,
}

struct InputBufferInner {
    buffer: [HashSet<Input>; crate::COMMAND_BUFFER_SIZE],
    oldest_index: usize,
}

impl InputBuffer {
    /// Create a new CommandBuffer with all inputs set to Input::Stop
    pub fn new() -> Self {
        InputBuffer {
            inner: Arc::new(Mutex::new(InputBufferInner {
                buffer: core::array::from_fn(|_| HashSet::new()),
                oldest_index: 0,
            })),
        }
    }

    /// Push a new input into the buffer, overwriting the oldest input
    pub fn push(&mut self, input: HashSet<Input>) {
        let mut inner = self.inner.lock().unwrap();
        let oldest_index = inner.oldest_index;
        inner.buffer[oldest_index] = input;
        inner.oldest_index = (inner.oldest_index + 1) % crate::COMMAND_BUFFER_SIZE;
    }

    /// Get the input at index i, where 0 is the most recent input and COMMAND_BUFFER_SIZE - 1 is the oldest input
    pub fn get(&self, i: usize) -> HashSet<Input> {
        let inner = self.inner.lock().unwrap();
        inner.buffer
            [(inner.oldest_index + crate::COMMAND_BUFFER_SIZE - i - 1) % crate::COMMAND_BUFFER_SIZE]
            .clone()
    }

    pub fn get_all(&self) -> [HashSet<Input>; crate::COMMAND_BUFFER_SIZE] {
        let inner = self.inner.lock().unwrap();
        let mut buffer = core::array::from_fn(|_| HashSet::new());
        for (i, item) in buffer.iter_mut().enumerate() {
            item.clone_from(
                &inner.buffer[(inner.oldest_index + crate::COMMAND_BUFFER_SIZE - i - 1)
                    % crate::COMMAND_BUFFER_SIZE],
            );
        }
        buffer
    }

    pub fn most_recent(&self) -> HashSet<Input> {
        self.get(0)
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self::new()
    }
}
