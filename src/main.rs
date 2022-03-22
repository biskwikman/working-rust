use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use std::time::Duration;
use sdl2::keyboard::Scancode;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

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
    direction: Direction,
    current_frame: i32,
}

// Returns the row of the spreadsheet corresponding to the given direction
fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
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

        // render frame based on key press
        let (frame_width, frame_height) = player.sprite.size();
        let current_frame = Rect::new(
            player.sprite.x() + frame_width as i32 * player.current_frame,
            player.sprite.y() + frame_height as i32 * direction_spritesheet_row(player.direction), 
            frame_width,
            frame_height,
        );
        let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
        // create a rectangle with the same height and width as our sprite rectangle
        let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);

        // apply sprite(empty rectangle with size and location corresponding to a single sprite)
        // to spritesheet (texture), to cut out first sprite and send to the screen rectangle
        canvas.copy(texture, current_frame, screen_rect)?;

        canvas.present();
        Ok(())
}

fn update_player(player: &mut Player) {
    use self::Direction::*;
    match player.direction {
        Left => {
            player.position = player.position.offset(-player.speed, 0);
        },
        Right => {
            player.position = player.position.offset(player.speed, 0);
        },
        Up => {
            player.position = player.position.offset(0, -player.speed);
        },
        Down => {
            player.position = player.position.offset(0, player.speed);
        },
    }
    // Only continue to animate if player is moving
    if player.speed != 0 {
        player.current_frame = (player.current_frame +1) % 3;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("working", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build()
        .expect("could not make canvas");

    // load spritesheet
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        speed: 0,
        direction: Direction::Right,
        current_frame: 0,
    };

    let mut event_pump = sdl_context.event_pump()?;

    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _=> {}
            }
        }

        if event_pump.keyboard_state().is_scancode_pressed(Scancode::W) {
            player.speed = PLAYER_MOVEMENT_SPEED;
            player.direction = Direction::Up;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::S) {
            player.speed = PLAYER_MOVEMENT_SPEED;
            player.direction = Direction::Down;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::A) {
            player.speed = PLAYER_MOVEMENT_SPEED;
            player.direction = Direction::Left;
        }
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
            player.speed = PLAYER_MOVEMENT_SPEED;
            player.direction = Direction::Right;
        }
        if !event_pump.keyboard_state().is_scancode_pressed(Scancode::S) 
            & !event_pump.keyboard_state().is_scancode_pressed(Scancode::W)
            & !event_pump.keyboard_state().is_scancode_pressed(Scancode::A)
            & !event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
            player.speed = 0;
        }

        // Update
        i = (i + 1) % 255;
        update_player(&mut player);

        //Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}
