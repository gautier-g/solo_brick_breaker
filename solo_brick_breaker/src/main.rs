extern crate sdl2;

use nalgebra::Point2;
use sdl2::image::{self, LoadTexture};
use std::path::Path;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use std::f64::consts::PI;

struct Angle {
    x: f64,
}

impl Angle {
    fn new() -> Self {
        Angle { x: PI / 2.0 } // 90 degrés en radians
    }

    fn incr(&mut self) {
        if self.x <= 19.0*PI/20.0 {self.x += PI/100.0;}
    }

    fn decr(&mut self) {
        if self.x >= PI/20.0 {self.x -= PI/100.0;}
    }

    fn cos(&self) -> f64 {
        self.x.cos() 
    }

    fn sin(&self) -> f64 {
        self.x.sin()
    }
}

struct Ball {
    pos : Point2<f32>,
    vitesse: Point2<f32>
}

impl Ball {
    fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Ball {
            pos: Point2::new(x, y),
            vitesse: Point2::new(vx, vy),
        }
    }

    fn shift(&mut self) -> i32 {
        if self.pos.x <= 0.0 || self.pos.x >= (WINDOW_WIDTH as f32 - BALL_SIZE as f32) {
            self.vitesse.x = -self.vitesse.x;
        }
        if self.pos.y <= 0.0 {
            self.vitesse.y = -self.vitesse.y;
        }

        self.pos.x = self.pos.x + self.vitesse.x;
        self.pos.y = self.pos.y + self.vitesse.y;

        if self.pos.y >= WINDOW_WIDTH as f32 {-1}
        else {0}
    }

    fn rect(&self) -> Rect {
        Rect::new(self.pos.x as i32, self.pos.y as i32,BALL_SIZE,BALL_SIZE)
    }
}

const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 600;
const BALL_SIZE: u32 = 20;
const N: i32 = 10;
const VITESSE : f32= 15.0;

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

    let mut game : bool = false;

    let mut balls_in_game: i32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Escape => break 'running,
                        Keycode::Left => angle.incr(),
                        Keycode::Right => angle.decr(),
                        Keycode::Return | Keycode::Space => if !game {game = true;},
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if game && balls_in_game < N {
            balls.push(Ball::new((WINDOW_WIDTH - (BALL_SIZE as i32)) as f32 / 2.0, (WINDOW_HEIGHT - (BALL_SIZE as i32)) as f32, angle.cos() as f32 *VITESSE, -angle.sin() as f32 *VITESSE));
            std::thread::sleep(Duration::from_millis(50));
            balls_in_game+=1;
            if balls_in_game == N-1 {balls_in_game+=1;};
        }        

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Dessiner les balles
        for i in 0..balls.len() {
            if balls[i].shift() == -1 {
                index.push(i);  
            }
            canvas.copy(&texture, None, balls[i].rect()).unwrap();
        }

        for i in &index {
            balls.remove(*i);
        }
        index.clear();
        
        if balls.is_empty() { game = false; balls_in_game = 0;};

        if !game {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_line(
                (WINDOW_WIDTH / 2, WINDOW_HEIGHT),
                (
                    (WINDOW_WIDTH as f64 / 2.0 + 200.0 * angle.cos()) as i32,
                    (WINDOW_HEIGHT as f64 - 200.0 * angle.sin()) as i32,
                ),
            ).unwrap();
        }
        
        canvas.present();
        std::thread::sleep(Duration::from_millis(10));
    }
}
