use std::{collections::VecDeque, sync::Arc, time::Duration};

use sdl2::{
    event::Event,
    image::{self, InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

const PLAYER_MOVEMENT_SPEED: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovementCommand {
    GoUp,
    GoDown,
    GoLeft,
    GoRight,
    HaltUp,
    HaltDown,
    HaltRight,
    HaltLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    current_velocity: Velocity,
    direction: Direction,
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(
        screen_position,
        player.sprite.width(),
        player.sprite.height(),
    );
    canvas.copy(texture, player.sprite, screen_rect)?;

    canvas.present();

    Ok(())
}

// Update player a fixed amount based on their speed.
// WARNING: Calling this function too often or at a variable
// rate will cause the player's speed to be unpredictable!
fn update_player(player: &mut Player, commands: &VecDeque<MovementCommand>) {
    for command in commands.iter() {
        match command {
            MovementCommand::Left => {
                player.direction = Direction::Left;
                player.current_velocity = Velocity { x: -player.speed, y: 0 };
            },
            MovementCommand::Right => {
                player.direction = Direction::Right;
                player.current_velocity = Velocity { x: player.speed, y: 0 };
            },
            MovementCommand::Up => {
                player.direction = Direction::Up;
                player.current_velocity = Velocity { x: 0, y: -player.speed };
            },
            MovementCommand::Down => {
                player.direction = Direction::Down;
                player.current_velocity = Velocity { x: 0, y: player.speed };
            },
            MovementCommand::Halt => player.current_velocity = Velocity { x: 0, y: 0 },
        }
    }

    player.position = player
        .position
        .offset(player.current_velocity.x, player.current_velocity.y);
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem.");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas.");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: PLAYER_MOVEMENT_SPEED,
        current_velocity: Velocity { x: 0, y: 0 },
        direction: Direction::Right,
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i: i32 = 0;
    'running: loop {
        // Handle events
        let mut movement_commands: VecDeque<MovementCommand> = VecDeque::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    movement_commands.push_back(MovementCommand::GoLeft);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    movement_commands.push_back(MovementCommand::GoRight);
                },
                Event::KeyDown { keycode: Some(Keycode::UP), .. } => {
                    movement_commands.push_back(MovementCommand::GoUp);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    movement_commands.push_back(MovementCommand::GoDown);
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_commands.push_back(MovementCommand::HaltLeft)
                }
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_commands.push_back(MovementCommand::HaltRight)
                }
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_commands.push_back(MovementCommand::HaltUp)
                }
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_commands.push_back(MovementCommand::HaltDown)
                }
                _ => {},
            }
        }

        // Update
        i = (i + 1) & 255;
        update_player(&mut player, &movement_commands);

        // Render
        render(
            &mut canvas,
            Color::RGB(i as u8, 64, 255 - i as u8),
            &texture,
            &player,
        )?;

        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
