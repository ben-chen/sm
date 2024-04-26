use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use specs::prelude::World;
use specs::{Builder, DispatcherBuilder, WorldExt};
use std::time::Duration;

use sm::{
    Direction, Input, MovementStats, PhysicsData, Player1, PlayerState, PlayerStatus, Sprite,
};

mod animator;
mod keyboard_input;
mod physics;
mod renderer;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let window = video_subsystem
        .window("SM", 1000, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let idle_path = std::path::Path::new("/Users/benchen/workspace/sm/assets/Samurai/Idle.png");
    let running_path = std::path::Path::new("/Users/benchen/workspace/sm/assets/Samurai/Run.png");
    let blocking_path =
        std::path::Path::new("/Users/benchen/workspace/sm/assets/Samurai/Block.png");
    let jumping_path = std::path::Path::new("/Users/benchen/workspace/sm/assets/Samurai/Jump.png");
    // let hitstun_path = std::path::Path::new("/Users/benchen/workspace/sm/assets/Samurai/Hurt.png");
    // let blockstun_path = std::path::Path::new("/Users/benchen/workspace/sm/assets/Samurai/Block.png");

    let texture_creator = canvas.texture_creator();
    let idle_texture = texture_creator.load_texture(idle_path)?;
    let running_texture = texture_creator.load_texture(running_path)?;
    let blocking_texture = texture_creator.load_texture(blocking_path)?;
    let jumping_texture = texture_creator.load_texture(jumping_path)?;
    let textures = [idle_texture, running_texture, blocking_texture, jumping_texture];

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard_input::Keyboard, "Keyboard", &[])
        .with(physics::Physics, "Physics", &["Keyboard"])
        .with(animator::Animator, "Animator", &["Keyboard"])
        .build();

    let mut world = World::new();
    world.insert(Player1);
    world.insert(Input::Stop);
    dispatcher.setup(&mut world);

    world
        .create_entity()
        .with(Player1)
        .with(PhysicsData {
            position: Point::new(0, 0),
            h_speed: 0,
            v_speed: 0,
            h_acceleration: 0,
            v_acceleration: 0,
        })
        .with(Sprite {
            spritesheet: 0,
            current: Rect::new(0, 0, 128, 128),
            wrap: 1024,
            flip: false,
            counter: 0,
            animation_rate: 5,
        })
        .with(MovementStats {
            max_speed: 20,
            acceleration: 3,
            friction: 1,
            gravity: 2,
            jump_power: 20,
            air_acceleration: 1,
            air_max_speed: 10,
        })
        .with(PlayerState {
            status: PlayerStatus::Idle,
            facing: Direction::Right,
        })
        .build();

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'mainloop: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => (),
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        let mut input = Input::Stop;

        // Left and right movement with SOCD = neutral
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            input = Input::Move(Direction::Left);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            input = if input == Input::Move(Direction::Left) {
                Input::Stop
            } else {
                Input::Move(Direction::Right)
            };
        }

        if keyboard_state.is_scancode_pressed(Scancode::Space) {
            input = Input::Jump;
        }
        *world.write_resource() = input;

        // Update state
        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world);
        world.maintain();

        // Render
        let color = Color::RGB(i, 50, 255 - i);
        renderer::render(&mut canvas, color, &textures, world.system_data())?;

        // Time
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
