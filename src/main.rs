mod utils;

use sdl2::image::{self, LoadTexture};
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::time::Duration;
use crate::utils::{WINDOW_WIDTH, WINDOW_HEIGHT, BALL_SIZE, N, VITESSE, Angle,Ball};

fn main() {
    let mut angle = Angle::new();

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem
        .window("Brick Breaker", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut balls = Vec::new();
    let mut index: Vec<usize> = Vec::new();

    let _image_context = image::init(image::InitFlag::PNG);

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture(Path::new("white-circle.png"))
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut round : bool = false;
    let mut frame = 0;
    let mut balls_in_round: i32 = 0;

    let rects = [Rect::new(100,100,100,100),Rect::new(500,300,100,100)];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
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
        if round && balls_in_round < N && frame % VITESSE== 0 {
            balls.push(Ball::new((WINDOW_WIDTH - (BALL_SIZE as i32)) as f32 / 2.0, (WINDOW_HEIGHT - (BALL_SIZE as i32)) as f32,
             angle.cos() as f32, -angle.sin() as f32));
            balls_in_round+=1;
            if balls_in_round == N-1 {balls_in_round+=1;};
        }        

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Deplacement des balles
        for i in 0..balls.len() {
            if balls[i].collision(&rects) == -1 {
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
    
        // Affichage de debut de round
        if !round {
            canvas.draw_line(
                (WINDOW_WIDTH / 2, WINDOW_HEIGHT),
                (
                    (WINDOW_WIDTH as f64 / 2.0 + 200.0 * angle.cos()) as i32,
                    (WINDOW_HEIGHT as f64 - 200.0 * angle.sin()) as i32,
                ),
            ).unwrap();
            canvas.fill_rects(&rects).unwrap();
            canvas.present();
        }

        // Affichage de round : deplacement des balles toutes les ieme frames pour simuler de la vitesse
        if frame % VITESSE == 0 && round {
            for i in 0..balls.len() {
                canvas.copy(&texture, None, balls[i].rect()).unwrap();
            }
            canvas.fill_rects(&rects).unwrap();
            canvas.present();
            std::thread::sleep(Duration::from_millis(15));
        }
        
        frame = (frame + 1)%VITESSE;
        
    }
}
