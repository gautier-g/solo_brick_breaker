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



fn main() {

    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width: i32 = 600;
    let height: i32 = 600;

    let window: Window = video_subsystem
        .window("Mon Jeu", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    const N : usize = 10;

    let mut rects = [Rect::new(1,1,90,90);N];
    let mut vitesses = [Point::new(0,0);N];

    let mut rng = rand::thread_rng();

    
    for i in 0..N {
        rects[i].reposition(Point::new(rng.gen_range(1..(width-100)),rng.gen_range(1..(height-100))));
        vitesses[i] = Point::new(rng.gen_range(-1..=1),rng.gen_range(-1..=1));
    }

    let _image_context = image::init(image::InitFlag::PNG);

    // Charger l'image
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
        canvas.set_draw_color(Color::RGB(255, 0, 0));
            
        for i in 0..rects.len() {
            let top_left_corner = rects[i].top_left();
            let bottom_right_corner = rects[i].bottom_right();

            if top_left_corner.x == 0 ||bottom_right_corner.x == width {
                vitesses[i].x = -vitesses[i].x;
            }
            if top_left_corner.y == 0 || bottom_right_corner.y == height{
                vitesses[i].y = -vitesses[i].y;
            }

            rects[i].set_x(rects[i].x + vitesses[i].x); 
            rects[i].set_y(rects[i].y + vitesses[i].y); 

            canvas.copy(&texture, None,rects[i] ).unwrap(); // Affiche l'image
    

        }
        
        canvas.present();
        std::thread::sleep(Duration::from_millis(3));
    }
    
}

