extern crate sdl2;

use nalgebra::Point2;
use sdl2::{mixer::Chunk, pixels::Color, rect::Rect, render::TextureCreator, ttf::Sdl2TtfContext, video::WindowContext};
use std::f64::consts::PI;
use sdl2::render::Texture;

pub const WINDOW_WIDTH: u32 = 600;
pub const WINDOW_HEIGHT: u32 = 700;
pub const BRICK_SIZE: u32 = 30;
pub const LEVEL_PATH : &str = "levels/test.txt";

pub struct Angle (f64);

impl Angle {
    pub fn new() -> Self {
        Angle (PI / 2.0)
    }

    pub fn incr(&mut self) {
        if self.0 <= 19.0*PI/20.0 {self.0 += PI/200.0;}
    }

    pub fn decr(&mut self) {
        if self.0 >= PI/20.0 {self.0 -= PI/200.0;}
    }

    pub fn cos(&self) -> f64 {
        self.0.cos() 
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }
}

pub struct Ball {
    pos : Point2<f32>,
    vitesse: Point2<f32>
}

impl<'a> Ball {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Ball {
            pos: Point2::new(x, y),
            vitesse: Point2::new(vx, vy),
        }
    }

    pub fn collision(&mut self,ttf_context: &Sdl2TtfContext,texture_creator: &'a TextureCreator<WindowContext>, bricks: &mut Vec<Brick<'a>>, new_ball_chunk: &Chunk, damage: i32, ball_size: u32) -> i32 {
        if self.pos.y >= WINDOW_HEIGHT as f32 {
            return -1;
        }
    
        if self.pos.x + self.vitesse.x <= 105.0 || self.pos.x + self.vitesse.x >= (WINDOW_WIDTH as f32 - ball_size as f32 - 105.0) {
            self.vitesse.x = -self.vitesse.x;
            self.shift();
            return 0;
        }
    
        if self.pos.y + self.vitesse.y <= 80.0 {
            self.vitesse.y = -self.vitesse.y;
            self.shift();
            return 0;
        }
    
        let tmp = Rect::new(
            (self.pos.x + self.vitesse.x) as i32,
            (self.pos.y + self.vitesse.y) as i32,
            ball_size,
            ball_size,
        );
    
        for brick in bricks.iter_mut() {
            if tmp.has_intersection(brick.rect) {
                sdl2::mixer::Channel(3).play(new_ball_chunk, 0).unwrap();

                brick.life -= damage;
                brick.set_texture(ttf_context, texture_creator);
    
                let x_center_brick = brick.rect.center().x;

                if ((self.vitesse.x > 0.0 && (x_center_brick > self.pos.x as i32)) || (self.vitesse.x < 0.0 && (x_center_brick < self.pos.x as i32))) && (brick.rect.y <= (self.pos.y as i32 + (ball_size/2) as i32) && (self.pos.y as i32 - (ball_size/2) as i32) <= brick.rect.y + brick.rect.height() as i32) {
                    self.vitesse.x = -self.vitesse.x; 
                }
                else {
                    self.vitesse.y = -self.vitesse.y;
                }
    
                self.shift();
                return 0;
            }
        }
    
        self.shift();
        0
    }

    pub fn shift(&mut self){
        self.pos.x += self.vitesse.x;
        self.pos.y += self.vitesse.y;
    }

    pub fn rect(&self, ball_size: u32) -> Rect {
        Rect::new(self.pos.x as i32, self.pos.y as i32,ball_size,ball_size)
    }
}

pub struct Brick<'a> {
    pub rect : Rect,
    pub life : i32,
    pub texture : Texture<'a>,
    pub brick_type : String
}

impl<'a> PartialEq for Brick<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rect == other.rect
    }
}

impl<'a> Brick<'a> {
    pub fn new(i: i32, j: i32, life: i32, brick_type: String, ttf_context: &Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let font = ttf_context.load_font("fonts/Marlboro.ttf", 128).unwrap();

        let life_text = life.to_string();

        let brick_color = Color::RGB(30, 30,30);

        let brick_surface = font
            .render(&life_text)
            .blended(brick_color)
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&brick_surface)
            .unwrap();     

        Brick {
            rect: Rect::new(i * (BRICK_SIZE+2) as i32 + 109, j * (BRICK_SIZE+2) as i32 + 151, BRICK_SIZE, BRICK_SIZE),
            life: life,
            texture: texture,
            brick_type: brick_type
        }
    }

    pub fn set_texture(&mut self,ttf_context: &Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>) {
        let font = ttf_context.load_font("fonts/Marlboro.ttf", 128).unwrap();

        let life_text = self.life.to_string();

        let brick_color = Color::RGB(30, 30,30);
        
        let brick_surface = font
            .render(&life_text)
            .blended(brick_color)
            .unwrap();

        self.texture = texture_creator
            .create_texture_from_surface(&brick_surface)
            .unwrap();        
    }

    pub fn euclidian_distance(&self, brick: &Brick) -> i32 {
        let res1: f32 = (brick.rect.y - self.rect.y) as f32;
        let res2: f32 = (brick.rect.x - self.rect.x) as f32;
        let sum = (res1 as i32).pow(2) + (res2 as i32).pow(2);
        return (sum as f32).sqrt() as i32;
    }
}
