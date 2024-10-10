
mod game;
mod utils;

use sdl2::image::{self, LoadTexture};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use std::path::Path;
use crate::utils::{WINDOW_WIDTH, WINDOW_HEIGHT, BALL_SIZE, N, VITESSE, Angle,Ball,Brick};
use std::time::Duration;
use crate::game::Game;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::Sdl;

fn main() {

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let window: Window = video_subsystem
        .window("Brick Breaker", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Ici de base
    let mut game = Game {
        started: true,
        paused: false,
        drawn: Vec::new(),
        textured: Vec::new()
    };
    let mut bricks_store = game.load_content(&ttf_context, &texture_creator,String::from("levels/test.txt"));
    let mut bricks = bricks_store.iter_mut().collect();
    //

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut angle = Angle::new();
    let mut balls = Vec::new();
    let mut index: Vec<usize> = Vec::new();

    let _image_context = image::init(image::InitFlag::PNG);

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture(Path::new("white-circle.png"))
        .unwrap();

    let mut round : bool = false;
    let mut frame = 0;
    let mut balls_in_round: i32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseButtonDown {x, y, .. } => game = game.act_drawn(x, y),
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Escape => break 'running,
                        Keycode::Left => angle.incr(),
                        Keycode::Right => angle.decr(),
                        Keycode::Return | Keycode::Space => if !round {round = true;frame = 0; },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
        // Lancement des balles au fur et a mesure
        if round && balls_in_round < N && frame % VITESSE == 0 {
            balls.push(Ball::new((WINDOW_WIDTH - (BALL_SIZE as u32)) as f32 / 2.0, (WINDOW_HEIGHT - (BALL_SIZE as u32)) as f32,
             angle.cos() as f32, -angle.sin() as f32));
            balls_in_round+=1;
            if balls_in_round == N-1 {balls_in_round+=1;};
        }        
        
        // Deplacement des balles
        for i in 0..balls.len() {
            if balls[i].collision(&mut bricks) == -1 {
                index.push(i);  
            }
        }
        
        // Les balles en dehors du jeu sont retirées
        for i in &index {
            balls.remove(*i);
        }
        index.clear();
     
        
        
        // Si il n'y a plus de balle le round fini
        if balls.is_empty() { round = false; balls_in_round = 0;};

        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Affichage de debut de round
        if !round {
            canvas.draw_line(
                (WINDOW_WIDTH as i32/ 2 , WINDOW_HEIGHT as i32),
                (
                    (WINDOW_WIDTH as f64 / 2.0 + 200.0 * angle.cos()) as i32,
                    (WINDOW_HEIGHT as f64 - 200.0 * angle.sin()) as i32,
                ),
            ).unwrap();
            
        }

        // Affichage de round : deplacement des balles toutes les ieme frames pour simuler de la vitesse
        if round {
            
            for i in 0..bricks.len() {
                if bricks[i].life <= 0 {
                    index.push(i);  
                }
            }

            // Les balles en dehors du jeu sont retirées
            for i in &index {
                bricks.remove(*i);
            }
            index.clear();


            for ball in &balls {
                canvas.copy(&texture, None, ball.rect()).unwrap();
            }

        }
        frame = (frame + 1)%VITESSE;

        
        match (game.started, game.paused) {
            (true,false) => canvas = game.display_game(canvas,&ttf_context, &texture_creator,&bricks),
            (true,true) => canvas = game.display_pause(canvas),
            _ => canvas = game.display_menu(canvas),
        }
        //draw_bricks(&mut canvas, &mut bricks);
        
        canvas.present();
    }
    
}

