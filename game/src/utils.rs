// Importation des librairies
extern crate sdl2;

use nalgebra::Point2;
use sdl2::rect::Rect;
use std::f64::consts::PI;

// Déclaration des constantes
pub const WINDOW_WIDTH: u32 = 600;
pub const WINDOW_HEIGHT: u32 = 700;
pub const BALL_SIZE: u32 = 10;
pub const BRICK_SIZE: u32 = 32;
pub const N: i32 = 10;
pub const VITESSE : i32= 15;

// Déclarations des structures
pub struct Angle (f64);

impl Angle {
    pub fn new() -> Self {
        Angle (PI / 2.0) // 90 degrés en radians
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

impl Ball {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Ball {
            pos: Point2::new(x, y),
            vitesse: Point2::new(vx, vy),
        }
    }

    pub fn collision(&mut self,bricks : &mut Vec<&mut Brick>) -> i32 {
        if self.pos.y >= WINDOW_HEIGHT as f32 {return -1}

        if self.pos.x + self.vitesse.x <= 0.0 || self.pos.x + self.vitesse.x >= (WINDOW_WIDTH as f32 - BALL_SIZE as f32) {
            self.vitesse.x = -self.vitesse.x;
            self.shift();
            return 0
        }

        if self.pos.y + self.vitesse.y <= 0.0 {
            self.vitesse.y = -self.vitesse.y;
            self.shift();
            return 0
        }

        let tmp = Rect::new((self.pos.x + self.vitesse.x) as i32,(self.pos.y + self.vitesse.y) as i32,BALL_SIZE,BALL_SIZE);
        for brick in bricks.iter_mut() {
            if tmp.has_intersection(brick.rect) {
                brick.life-=1;
                if brick.rect.x as f32 > self.pos.x || self.pos.x > brick.rect.x as f32 + brick.rect.width() as f32 {
                    self.vitesse.x = -self.vitesse.x; 
                }
                else {
                    self.vitesse.y = -self.vitesse.y;
                }
                self.shift();
                return 0
            }
        }
        self.shift();
        0
    }

    pub fn shift(&mut self){
        self.pos.x += self.vitesse.x;
        self.pos.y += self.vitesse.y;
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x as i32, self.pos.y as i32,BALL_SIZE,BALL_SIZE)
    }
}

pub struct Brick {
    pub rect : Rect,
    pub life : i32
}

impl Brick {
    pub fn new(i:i32,j:i32,life:i32) -> Self {
        Brick {rect:Rect::new(i*BRICK_SIZE as i32+ 108 ,j*BRICK_SIZE as i32+ 150 ,BRICK_SIZE,BRICK_SIZE),life}
    }
}