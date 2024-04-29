use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use specs::prelude::World;
use specs::{Builder, DispatcherBuilder, WorldExt};

use sm::{Direction, MovementStats, PhysicsData, Player1, PlayerState, PlayerStatus, Sprite};

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(sm::FONT_PATH, 14)?;
    let window = video_subsystem
        .window("SM", 1000, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let texture_paths = [
        sm::IDLE_PATH,
        sm::RUNNING_PATH,
        sm::BLOCKING_PATH,
        sm::JUMPING_PATH,
        sm::HITSTUN_PATH,
        sm::BLOCKSTUN_PATH,
        sm::ATTACKING_PATH,
    ];
    let textures = texture_paths.map(|path| {
        texture_creator
            .load_texture(path)
            .unwrap_or_else(|_| panic!("Failed to load texture: {}", path))
    });

    let mut dispatcher = DispatcherBuilder::new()
        .with(sm::keyboard_input::Keyboard, "Keyboard", &[])
        .with(sm::physics::Physics, "Physics", &["Keyboard"])
        .with(sm::animator::Animator, "Animator", &["Keyboard"])
        .build();

    let mut world = World::new();
    world.insert(sm::InputBuffer::new());
    world.insert(sm::Framerate(1));
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
            jump_power: 22,
            superjump_power: 28,
            air_acceleration: 1,
            air_max_speed: 10,
        })
        .with(PlayerState {
            status: PlayerStatus::Idle,
            facing: Direction::Right,
            animation_counter: 0,
        })
        .build();

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut time_acculumator = std::time::Duration::new(0, 0);
    let mut prev_time = std::time::Instant::now();

    let mut frame_count = 0u32;
    let mut fps_timer = std::time::Instant::now();
    'mainloop: loop {
        time_acculumator += prev_time.elapsed();
        prev_time = std::time::Instant::now();
        while time_acculumator >= sm::FRAME_TIME {
            // Handle events
            event_pump.pump_events();
            let keyboard_state = event_pump.keyboard_state();
            let input = sm::keyboard_input::get_input(&keyboard_state);
            if input.contains(&sm::Input::Quit) {
                break 'mainloop;
            }

            let mut input_buffer = world.write_resource::<sm::InputBuffer>().clone();
            dbg!(&input);
            input_buffer.push(input);

            // Update state
            dispatcher.dispatch(&world);
            world.maintain();
            time_acculumator -= sm::FRAME_TIME;
        }

        // Render
        let color = Color::RGB(50, 50, 50);
        sm::renderer::render(
            &mut canvas,
            color,
            &texture_creator,
            &textures,
            &font,
            &world,
        )?;

        // Count frames
        frame_count += 1;
        if fps_timer.elapsed().as_secs() >= 1 {
            dbg!(frame_count);
            world.write_resource::<sm::Framerate>().set(frame_count);
            frame_count = 0;
            fps_timer = std::time::Instant::now();
        }
    }

    Ok(())
}
