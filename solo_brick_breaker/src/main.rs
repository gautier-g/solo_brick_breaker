extern crate sdl2;

use sdl2::image::{self, LoadTexture};
use std::path::Path;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::Sdl;
use rand::Rng;

struct Ball {
    rect : Rect,
    vitesse : Point
}

const WINDOW_WIDTH: i32 = 600;
const WINDOW_HEIGHT: i32 = 600;
const BALL_SIZE: u32 = 20;
const N : usize = 10;

impl Ball {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Ball {
            rect: Rect::new(x, y, BALL_SIZE, BALL_SIZE),
            vitesse: Point::new(vx, vy),
        }
    }
    
    fn shift(&mut self) {
        let top_left_corner = self.rect.top_left();
        let bottom_right_corner = self.rect.bottom_right();

        if top_left_corner.x <= 0 ||bottom_right_corner.x >= WINDOW_WIDTH {
            self.vitesse.x = -self.vitesse.x;
        }
        if top_left_corner.y <= 0 || bottom_right_corner.y >= WINDOW_HEIGHT{
            self.vitesse.y = -self.vitesse.y;
        }

        self.rect.set_x(self.rect.x + self.vitesse.x); 
        self.rect.set_y(self.rect.y + self.vitesse.y);
    }
}

fn main() {

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem
        .window("Brick Breaker", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    let mut rng = rand::thread_rng();
    let mut balls = Vec::new();
    
    let _image_context = image::init(image::InitFlag::PNG);

    for _ in 0..N {
        let tmp = Ball::new(290,579,rng.gen_range(-5..=5),rng.gen_range(-5..=5));
        balls.push(tmp);
    }

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture(Path::new("white-circle.png"))
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
            
        for ball in &mut balls {
            ball.shift();
            canvas.copy(&texture, None,ball.rect).unwrap();
        }
        
        canvas.present();
        std::thread::sleep(Duration::from_millis(10));
    }
}