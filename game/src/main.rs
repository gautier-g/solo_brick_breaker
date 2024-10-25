mod game;
mod utils;

use game::Wave;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::image::{self, LoadTexture};
use sdl2::video::Window;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::str::FromStr;
use crate::utils::{WINDOW_WIDTH, WINDOW_HEIGHT, Angle};
use crate::game::Game;
use sdl2::event::Event;
use sdl2::Sdl;
use std::time::Duration;

fn main() {
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let _image_context = image::init(image::InitFlag::PNG);

    let window: Window = video_subsystem
        .window("Brick Breaker", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut game = Game {
        started: false,
        paused: false,
        drawn: Vec::new(),
        textured: Vec::new(),
        bricks: Vec::new(),
        angle: Angle::new(),
        balls: Vec::new(),
        index: Vec::new(),
        round: false,
        balls_in_round: 0,
        game_is_loaded: false,
        game_is_lost: false,
        wave: Wave::new(1, false, &ttf_context, &texture_creator)
    };

    let frequency = 44_100;
    let format = AUDIO_S16LSB;
    let channels = DEFAULT_CHANNELS;
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
    sdl2::mixer::allocate_channels(10);

    let home_music_chunk = sdl2::mixer::Chunk::from_file(Path::new("retro-game-arcade-236133.mp3")).unwrap();
    let background_ig_music_chunk = sdl2::mixer::Chunk::from_file(Path::new("background-music.mp3")).unwrap();
    sdl2::mixer::Channel(0).set_volume(60);
    sdl2::mixer::Channel(1).set_volume(30);
    sdl2::mixer::Channel(0).play(&home_music_chunk, 2).unwrap();

    game.load_content(&ttf_context, &texture_creator);

    let ball_texture = texture_creator
        .load_texture(Path::new("white-circle.png"))
        .unwrap();


    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame: i32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    game.act_drawn(x, y, &home_music_chunk, &background_ig_music_chunk);
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Escape => break 'running,
                        Keycode::Left => game.angle.incr(),
                        Keycode::Right => game.angle.decr(),
                        Keycode::Return | Keycode::Space => if !game.round {game.round = true},
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if !game.game_is_loaded {
            game.load_bricks(&ttf_context, &texture_creator, String::from_str("levels/test.txt").unwrap());
        }

        if !game.paused {
            game.update_balls_state(frame, &ttf_context, &texture_creator);
        }

        match (game.started, game.paused, game.game_is_lost) {
            (_, _, true) => {
                canvas = game.display_loss(canvas);
            }
            (true, false, _) => {
                canvas = game.display_balls_and_bricks(canvas, &ball_texture, frame);
            },
            (true, true, _) => {
                canvas = game.display_pause(canvas);
            },
            _ => {
                canvas = game.display_menu(canvas);
            }
        }
        frame = frame + 1;

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }
}