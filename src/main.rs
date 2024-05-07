use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use specs::prelude::World;
use specs::{Builder, DispatcherBuilder, WorldExt};

use sm::{
    CollisionData, CollisionMask, CollisionStatus, Direction, Fi32, MovementStats, PhysicsData,
    Player1, PlayerState, PlayerStatus, PointFi32, Sprite,
};

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let window = video_subsystem
        .window("SM", 1000, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let font = ttf_context.load_font(sm::FONT_PATH, 14)?;
    let texture_creator = canvas.texture_creator();

    let texture_paths = [
        sm::IDLE_PATH,
        sm::RUNNING_PATH,
        sm::BLOCKING_PATH,
        sm::JUMPING_PATH,
        sm::HITSTUN_PATH,
        sm::BLOCKSTUN_PATH,
        sm::ATTACKING_PATH,
        sm::FIGHTER_DEAD_PATH,
    ];
    let textures = texture_paths.map(|path| {
        texture_creator
            .load_texture(path)
            .unwrap_or_else(|_| panic!("Failed to load texture: {}", path))
    });

    let mut dispatcher = DispatcherBuilder::new()
        .with(sm::keyboard_input::Keyboard, "Keyboard", &[])
        .with(sm::collider::Collider, "Collider", &["Keyboard"])
        .with(sm::physics::Physics, "Physics", &["Collider"])
        .with(
            sm::player_animator::PlayerAnimator,
            "PlayerAnimator",
            &["Physics"],
        )
        .with(sm::animator::Animator, "Animator", &["PlayerAnimator"])
        .build();

    let mut world = World::new();
    world.insert(sm::InputBuffer::new());
    world.insert(sm::Framerate(1));
    dispatcher.setup(&mut world);

    world
        .create_entity()
        .with(Player1)
        .with(PhysicsData {
            position: PointFi32::new(0, 0),
            speed: PointFi32::new(0, 0),
            acceleration: PointFi32::new(0, 0),
        })
        .with(Sprite {
            spritesheet: 0,
            current: Rect::new(0, 0, 128, 128),
            wrap: 1024,
            flip: false,
            counter: 0,
            animation_rate: 5,
            glow: false,
        })
        .with(CollisionData {
            mask: CollisionMask::Circle(PointFi32::new(0, 0), Fi32::from_num(36.0)),
            status: CollisionStatus(false),
            repel_vector: PointFi32::new(0, 0),
            repel_speed: Fi32::from_num(3.0),
        })
        .with(MovementStats {
            max_speed: Fi32::from_num(17),
            acceleration: Fi32::from_num(2.5),
            friction: Fi32::from_num(1.2),
            gravity: Fi32::from_num(1.8),
            jump_power: Fi32::from_num(22),
            superjump_power: Fi32::from_num(30),
            air_acceleration: Fi32::from_num(1),
            air_max_speed: Fi32::from_num(10),
        })
        .with(PlayerState {
            status: PlayerStatus::Idle,
            facing: Direction::Right,
            animation_counter: 0,
        })
        .build();

    world
        .create_entity()
        .with(PhysicsData {
            position: PointFi32::new(200, 0),
            speed: PointFi32::new(0, 0),
            acceleration: PointFi32::new(0, 0),
        })
        .with(Sprite {
            spritesheet: 7,
            current: Rect::new(0, 0, 128, 128),
            wrap: 384,
            flip: false,
            counter: 0,
            animation_rate: 60,
            glow: false,
        })
        .with(CollisionData {
            mask: CollisionMask::Circle(PointFi32::new(0, 0), Fi32::from_num(36.0)),
            status: CollisionStatus(false),
            repel_vector: PointFi32::new(0, 0),
            repel_speed: Fi32::ZERO,
        })
        .build();

    world
        .create_entity()
        .with(PhysicsData {
            position: PointFi32::new(360, 0),
            speed: PointFi32::new(0, 0),
            acceleration: PointFi32::new(0, 0),
        })
        .with(Sprite {
            spritesheet: 7,
            current: Rect::new(0, 0, 128, 128),
            wrap: 384,
            flip: true,
            counter: 0,
            animation_rate: 60,
            glow: false,
        })
        .with(CollisionData {
            mask: CollisionMask::Circle(PointFi32::new(0, 0), Fi32::from_num(36.0)),
            status: CollisionStatus(false),
            repel_vector: PointFi32::new(0, 0),
            repel_speed: Fi32::ZERO,
        })
        .build();

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_time_accumulator = std::time::Duration::new(0, 0);
    let mut prev_time = std::time::Instant::now();

    let mut frame_count = 0u32;
    let mut fps_timer = std::time::Instant::now();
    'mainloop: loop {
        frame_time_accumulator += prev_time.elapsed();

        prev_time = std::time::Instant::now();
        while frame_time_accumulator >= sm::FRAME_TIME {
            // Handle events
            event_pump.pump_events();
            let keyboard_state = event_pump.keyboard_state();
            let input = sm::keyboard_input::get_input(&keyboard_state);
            if input.contains(&sm::Input::Quit) {
                break 'mainloop;
            }

            let mut input_buffer = world.write_resource::<sm::InputBuffer>().clone();
            // dbg!(&input);
            input_buffer.push(input);

            // Update state
            dispatcher.dispatch(&world);
            world.maintain();
            frame_time_accumulator -= sm::FRAME_TIME;
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
            world.write_resource::<sm::Framerate>().set(frame_count);
            frame_count = 0;
            fps_timer = std::time::Instant::now();
        }
    }

    Ok(())
}
