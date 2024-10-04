mod constants;
mod game;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;

use crate::game::Game;
use crate::constants::SCREEN_HEIGHT;
use crate::constants::SCREEN_WIDTH;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let window = video_subsystem.window("CONCRETE ANNIHILATOR", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut game = Game {
        started: true,
        paused: false,
        drawn: Vec::new(),
        textured: Vec::new()
    };

    game.load_content(&ttf_context, &texture_creator);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        canvas = game.display_content(canvas);

        ::std::thread::sleep(Duration::from_millis(16));
    }
}
