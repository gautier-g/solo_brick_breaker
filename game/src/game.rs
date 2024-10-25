extern crate sdl2;
extern crate rand;

use ffmpeg_next::ffi::sqrtf;
use sdl2::mixer::Chunk;
use sdl2::surface::Surface;
use sdl2::sys::ttf::TTF_HINTING_NONE;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use crate::utils::{Angle, Ball, BALL_SIZE, BRICK_SIZE, N, WINDOW_HEIGHT, WINDOW_WIDTH};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};
use std::path::Path;

use crate::utils::Brick;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub(crate) struct DrawnContent {
    pub(crate) displayed_in_game: bool,
    pub(crate) displayed_in_pause: bool,
    pub(crate) displayed_at_loss: bool,
    pub(crate) name: Option<String>,
    pub(crate) rect: Rect,
    pub(crate) color: Color
}

pub(crate) struct TexturedContent<'a> {
    pub(crate) displayed_in_game: bool,
    pub(crate) displayed_in_pause: bool,
    pub(crate) displayed_at_loss: bool,
    pub(crate) name: Option<String>,
    pub(crate) texture: Texture<'a>,
    pub(crate) src: Option<Rect>,
    pub(crate) dst: Option<Rect>
}

pub(crate) struct Wave <'a> {
    pub(crate) wave_number: u32,
    pub(crate) is_ignited: bool,
    pub(crate) title_texture: Texture<'a>,
    pub(crate) no_title_texture: Texture<'a>,
}

impl<'a> Wave <'a> {
    pub fn new(wave_number: u32, is_ignited: bool, ttf_context: &Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let font = ttf_context.load_font(Path::new("fonts/Marlboro.ttf"), 128).unwrap();

        let title_surface = font
            .render(&format!("{}{}", "Wave n°", wave_number.to_string()))
            .blended(Color::RGBA(180, 120, 120, 255))
            .map_err(|e| e.to_string()).unwrap();

        let title_texture = texture_creator
            .create_texture_from_surface(&title_surface)
            .map_err(|e| e.to_string()).unwrap();

        let no_title_surface = font
            .render(&format!("{}{}", "Wave n°", wave_number.to_string()))
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let no_title_texture = texture_creator
            .create_texture_from_surface(&no_title_surface)
            .map_err(|e| e.to_string()).unwrap();

        Wave {
            wave_number: wave_number,
            is_ignited: is_ignited,
            title_texture: title_texture,
            no_title_texture: no_title_texture
        }
    }

    pub fn display(&self, mut canvas: Canvas<Window>, frame: i32) ->Canvas<Window> {
        if (frame % 60) <= 30 {
            canvas.copy(&(self.title_texture), None, Some(rect!(50, 20, 250, 50))).unwrap();
        }
        else {
            canvas.copy(&(self.no_title_texture), None, Some(rect!(50, 20, 250, 50))).unwrap();
        }
        
        canvas
    }
}

pub(crate) struct Game<'a> {
    pub(crate) started: bool,
    pub(crate) paused: bool,
    pub(crate) drawn: Vec<DrawnContent>,
    pub(crate) textured: Vec<TexturedContent<'a>>,
    pub(crate) bricks: Vec<Brick<'a>>,
    pub(crate) angle: Angle,
    pub(crate) balls: Vec<Ball>,
    pub(crate) index: Vec<usize>,
    pub(crate) round: bool,
    pub(crate) balls_in_round: i32,
    pub(crate) game_is_loaded: bool,
    pub(crate) game_is_lost: bool,
    pub(crate) wave: Wave<'a>
}

impl<'a> Game<'a> {

    pub fn load_bricks(&mut self,ttf_context: &Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>,level : String) {

        let font = ttf_context.load_font(Path::new("fonts/Marlboro.ttf"), 128).unwrap();

        let file = File::open(level).unwrap();
        let reader = io::BufReader::new(file);
    
        let mut bricks:Vec<Brick<'a>> = Vec::new();
        let mut j= 0;

        for line in reader.lines() {
            match line {
                Ok(content) => {                
                    let tmp : Vec<&str> =  content.split(" ").collect();
                    for i in 0..tmp.len() {
                        match tmp[i].parse::<u32>() {
                            Ok(0) => {},
                            Ok(nombre) => {
                                let brick_surface = font
                                    .render(tmp[i].to_string().as_str())
                                    .blended(Color::RGBA(0, 255, 0, 255))
                                    .map_err(|e| e.to_string()).unwrap();

                                let brick_texture = texture_creator
                                    .create_texture_from_surface(&brick_surface)
                                    .map_err(|e| e.to_string()).unwrap();

                                                             
                                bricks.push(Brick::new(i as i32,j as i32 , nombre as i32,brick_texture));
                            },
                            _ => {}, 
                        }
                    }
                }
                _ => {}, 
            }
            j = j + 1;
        }
        self.bricks = bricks;
        self.game_is_loaded = true;

    }

    pub (crate) fn load_content(&mut self, ttf_context: &Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>){
        let font = ttf_context.load_font(Path::new("fonts/Marlboro.ttf"), 128).unwrap();

        let surface = font
            .render("CONCRETE ANNIHILATOR")
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        
        let title_textured_content = TexturedContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("menu_title").unwrap()),
            texture: texture,
            src: None,
            dst: Some(rect!(WINDOW_WIDTH/2 - width/6, 50, width/3, height/3))
        };

        self.textured.push(title_textured_content);

        let subtitle_surface = font
            .render("Survive a maximum of waves!")
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string()).unwrap();

        let subtitle_texture = texture_creator
            .create_texture_from_surface(&subtitle_surface)
            .map_err(|e| e.to_string()).unwrap();

        let TextureQuery { width, height, .. } = subtitle_texture.query();
        
        let subtitle_textured_content = TexturedContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("menu_subtitle").unwrap()),
            texture: subtitle_texture,
            src: None,
            dst: Some(rect!(WINDOW_WIDTH/2 - width/6, 130, width/3, height/3))
        };

        self.textured.push(subtitle_textured_content);

        let start_button = DrawnContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("menu_start").unwrap()),
            rect: rect!(200, 200, 200, 100),
            color: Color::RGB(255, 255, 255)
        };

        let start_surface = font
            .render("Start")
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let start_texture = texture_creator
            .create_texture_from_surface(&start_surface)
            .map_err(|e| e.to_string()).unwrap();

        let start_textured_content = TexturedContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: None,
            texture: start_texture,
            src: None,
            dst: Some(rect!(225, 225, 150, 50))
        };

        let settings_button = DrawnContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("menu_settings").unwrap()),
            rect: rect!(200, 350, 200, 100),
            color: Color::RGB(255, 255, 255)
        };

        let settings_surface = font
            .render("Settings")
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let settings_texture = texture_creator
            .create_texture_from_surface(&settings_surface)
            .map_err(|e| e.to_string()).unwrap();

        let settings_textured_content = TexturedContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: None,
            texture: settings_texture,
            src: None,
            dst: Some(rect!(225, 375, 150, 50))
        };

        let credits_button = DrawnContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("menu_credits").unwrap()),
            rect: rect!(200, 500, 200, 100),
            color: Color::RGB(255, 255, 255)
        };

        let credits_surface = font
            .render("Credits")
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let credits_texture = texture_creator
            .create_texture_from_surface(&credits_surface)
            .map_err(|e| e.to_string()).unwrap();

        let credits_textured_content = TexturedContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: None,
            texture: credits_texture,
            src: None,
            dst: Some(rect!(225, 525, 150, 50))
        };

        let pause_button = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("pause_button").unwrap()),
            rect: rect!(420, 15, 150, 40),
            color: Color::RGB(255, 255, 255)
        };

        let pause_surface = font
            .render("Pause")
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let pause_texture = texture_creator
            .create_texture_from_surface(&pause_surface)
            .map_err(|e| e.to_string()).unwrap();

        let pause_textured_content = TexturedContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: None,
            texture: pause_texture,
            src: None,
            dst: Some(rect!(425, 20, 140, 40))
        };

        let resume_button = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: true,
            displayed_at_loss: false,
            name: Some(String::from_str("pause_resume").unwrap()),
            rect: rect!(200, 200, 200, 100),
            color: Color::RGB(255, 255, 255)
        };

        let resume_surface = font
            .render("Resume")
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let resume_texture = texture_creator
            .create_texture_from_surface(&resume_surface)
            .map_err(|e| e.to_string()).unwrap();

        let resume_textured_content = TexturedContent {
            displayed_in_game: true,
            displayed_in_pause: true,
            displayed_at_loss: false,
            name: None,
            texture: resume_texture,
            src: None,
            dst: Some(rect!(225, 225, 150, 50))
        };

        let giveup_button = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: true,
            displayed_at_loss: false,
            name: Some(String::from_str("pause_giveup").unwrap()),
            rect: rect!(200, 350, 200, 100),
            color: Color::RGB(255, 255, 255)
        };

        let giveup_surface = font
            .render("Give up")
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let giveup_texture = texture_creator
            .create_texture_from_surface(&giveup_surface)
            .map_err(|e| e.to_string()).unwrap();

        let giveup_textured_content = TexturedContent {
            displayed_in_game: true,
            displayed_in_pause: true,
            displayed_at_loss: false,
            name: None,
            texture: giveup_texture,
            src: None,
            dst: Some(rect!(225, 375, 150, 50))
        };

        self.textured.push(start_textured_content);
        self.textured.push(settings_textured_content);
        self.textured.push(credits_textured_content);
        self.textured.push(pause_textured_content);
        self.textured.push(resume_textured_content);
        self.textured.push(giveup_textured_content);
        self.drawn.push(start_button);
        self.drawn.push(settings_button);
        self.drawn.push(credits_button);
        self.drawn.push(pause_button);
        self.drawn.push(resume_button);
        self.drawn.push(giveup_button);

        let left_bar_outside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("left_bar").unwrap()),
            rect: rect!(100, 75, 5, 600),
            color: Color::RGB(50, 50, 255)
        };

        let right_bar_outside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("right_bar").unwrap()),
            rect: rect!(WINDOW_WIDTH-105, 75, 5, 600),
            color: Color::RGB(50, 50, 255)
        };
        
        let left_bar_inside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("left_bar").unwrap()),
            rect: rect!(101, 76, 3, 598),
            color: Color::RGB(0, 0, 0)
        };

        let right_bar_inside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("right_bar").unwrap()),
            rect: rect!(WINDOW_WIDTH-104, 76, 3, 598),
            color: Color::RGB(0, 0, 0)
        };

        let top_bar_outside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("top_bar").unwrap()),
            rect: rect!(100, 75, 400, 5),
            color: Color::RGB(50, 50, 255)
        };

        let top_bar_inside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("top_bar").unwrap()),
            rect: rect!(101, 76, 398, 3),
            color: Color::RGB(0, 0, 0)
        };

        self.drawn.push(left_bar_outside);
        self.drawn.push(right_bar_outside);
        self.drawn.push(top_bar_outside);   
        self.drawn.push(left_bar_inside);
        self.drawn.push(right_bar_inside);
        self.drawn.push(top_bar_inside);

        let limit_bar = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            displayed_at_loss: false,
            name: Some(String::from_str("limit_bar").unwrap()),
            rect: rect!(101, 585, 398, 3),
            color: Color::RGB(255, 0, 0)
        };

        self.drawn.push(limit_bar);

        let retry_button = DrawnContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: true,
            name: Some(String::from_str("retry_button").unwrap()),
            rect: rect!(200,  475, 200, 100),
            color: Color::RGB(255, 255, 255)
        };

        let retry_surface = font
            .render("Retry")
            .blended(Color::RGBA(0, 0, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

        let retry_texture = texture_creator
            .create_texture_from_surface(&retry_surface)
            .map_err(|e| e.to_string()).unwrap();

        let retry_textured_content = TexturedContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: true,
            name: None,
            texture: retry_texture,
            src: None,
            dst: Some(rect!(225, 500, 150, 50))
        };

        let loss_title_surface = font
            .render("You lose!")
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string()).unwrap();

        let loss_title_texture = texture_creator
            .create_texture_from_surface(&loss_title_surface)
            .map_err(|e| e.to_string()).unwrap();

        let loss_title_textured_content = TexturedContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: true,
            name: None,
            texture: loss_title_texture,
            src: None,
            dst: Some(rect!(225, 200, 150, 50))
        };

        let loss_subtitle_surface = font
            .render("Best score: ")
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string()).unwrap();

        let loss_subtitle_texture = texture_creator
            .create_texture_from_surface(&loss_subtitle_surface)
            .map_err(|e| e.to_string()).unwrap();

        let loss_subtitle_textured_content = TexturedContent {
            displayed_in_game: false,
            displayed_in_pause: false,
            displayed_at_loss: true,
            name: None,
            texture: loss_subtitle_texture,
            src: None,
            dst: Some(rect!(225, 300, 150, 50))
        };

        self.drawn.push(retry_button);
        self.textured.push(retry_textured_content);
        self.textured.push(loss_title_textured_content);
        self.textured.push(loss_subtitle_textured_content);

    }

    pub(crate) fn display_menu(&self, mut can: Canvas<Window>) -> Canvas<Window> {
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();
        for content in self.drawn.iter() {
            if !content.displayed_in_game && !content.displayed_at_loss {
                let _ = can.set_draw_color(content.color);
                let _ = can.fill_rect(content.rect);
            }
        }
        
        for content in self.textured.iter() {
            if !content.displayed_in_game && !content.displayed_at_loss {
                let _ = can.copy(&content.texture, content.src, content.dst);
            }
        }
        can
    }

    pub(crate) fn display_pause(&self, mut can: Canvas<Window>) -> Canvas<Window> {
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();

        for content in self.drawn.iter() {
            if content.displayed_in_pause && content.displayed_in_game {
                let _ = can.set_draw_color(content.color);
                let _ = can.fill_rect(content.rect);
            }
        }
        
        for content in self.textured.iter() {
            if content.displayed_in_pause && content.displayed_in_game {
                let _ = can.copy(&content.texture, content.src, content.dst);
            }
        }
        can
    }

    pub(crate) fn display_loss(&self, mut can: Canvas<Window>) -> Canvas<Window> {
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();

        for content in self.drawn.iter() {
            if content.displayed_at_loss {
                let _ = can.set_draw_color(content.color);
                let _ = can.fill_rect(content.rect);
            }
        }
        
        for content in self.textured.iter() {
            if content.displayed_at_loss {
                let _ = can.copy(&content.texture, content.src, content.dst);
            }
        }
        can
    }

    pub(crate) fn display_game(&self, mut can: Canvas<Window>, frame: i32) -> Canvas<Window> {
        can.set_draw_color(Color::RGB(0, 0, 0));
        can.clear();

        for content in self.drawn.iter() {
            if !content.displayed_in_pause && content.displayed_in_game {
                let _ = can.set_draw_color(content.color);
                let _ = can.fill_rect(content.rect);
            }
        }
        
        for content in self.textured.iter() {
            if !content.displayed_in_pause && content.displayed_in_game {
                let _ = can.copy(&content.texture, content.src, content.dst);
            }
        }

        for brick in self.bricks.iter() {
            can.set_draw_color(Color::RGB(255,255,255));
            let _ = can.fill_rect(brick.rect);
            let _ = can.copy(&brick.texture,None,brick.rect);
        }

        can = self.wave.display(can, frame);

        can
    }

    pub(crate) fn act_drawn(&mut self, x: i32, y: i32, home_music_chunk: &Chunk, background_ig_music_chunk: &Chunk) {
        for content in self.drawn.iter_mut() {
            if (content.rect.x() <= x) && (x <= content.rect.x() + content.rect.width() as i32) && (content.rect.y() <= y) && (y <= content.rect.y() + content.rect.height() as i32) {
                if content.name == Some(String::from_str("menu_start").unwrap()) && self.started == false && self.game_is_lost == false {
                    self.started = true;

                    sdl2::mixer::Channel(0).halt();
                    sdl2::mixer::Channel(1).play(background_ig_music_chunk, 10000).unwrap();

                    self.balls = Vec::new();
                    self.game_is_loaded = false;
                }
                if content.name == Some(String::from_str("pause_button").unwrap()) && self.paused == false && self.game_is_lost == false {
                    self.paused = true;
                    sdl2::mixer::Channel(1).pause();
                }
                if content.name == Some(String::from_str("pause_resume").unwrap()) && self.paused == true {
                    self.paused = false;
                    sdl2::mixer::Channel(1).resume();
                }
                if content.name == Some(String::from_str("pause_giveup").unwrap()) && (self.started == true && self.paused == true) {
                    self.started = false;
                    self.paused = false;

                    sdl2::mixer::Channel(1).halt();
                    sdl2::mixer::Channel(0).play(home_music_chunk, 2).unwrap();
                }
                if content.name == Some(String::from_str("retry_button").unwrap()) && (self.started == false && self.paused == false && self.game_is_lost == true) {
                    self.game_is_lost = false;
                    self.started = true;
                    self.game_is_loaded = false;

                    sdl2::mixer::Channel(1).play(background_ig_music_chunk, 10000).unwrap();
                }
            }
        }
    }

    pub(crate) fn update_balls_state(&mut self, frame: i32, ttf_context: &Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>) {
        if (self.round && self.balls_in_round < N && frame % 2 == 0) || (self.round && self.balls_in_round == 0) {
            self.balls.push(Ball::new(
                (WINDOW_WIDTH - (BALL_SIZE as u32)) as f32 / 2.0,
                (WINDOW_HEIGHT - (BALL_SIZE as u32)) as f32,
                (self.angle.cos() as f32)*(unsafe { sqrtf((WINDOW_HEIGHT/10) as f32) }),
                -(self.angle.sin() as f32)*(unsafe { sqrtf((WINDOW_WIDTH/10) as f32) }),
            ));
            self.balls_in_round += 1;
        }        

        for i in 0..self.balls.len() {
            if self.balls[i].collision(&ttf_context, &texture_creator, &mut self.bricks) == -1 {
                self.index.push(i);  
            }
        }
        
        for i in self.index.iter().rev() {
            self.balls.remove(*i);
        }
        self.index.clear();
     
        if self.balls.is_empty() && self.round == true { self.round = false; self.balls_in_round = 0; self.get_bricks_down(); self.game_is_lost = self.is_lost();} 
    }

    pub(crate) fn display_balls_and_bricks(&mut self, mut canvas: Canvas<Window>, ball_texture: &Texture<'_>, frame: i32) -> Canvas<Window> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        
        canvas = self.display_game(canvas, frame);
        
        if !self.round {
            canvas.draw_line(
                (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32),
                (
                    (WINDOW_WIDTH as f64 / 2.0 + 200.0 * self.angle.cos()) as i32,
                    (WINDOW_HEIGHT as f64 - 200.0 * self.angle.sin()) as i32,
                ),
            ).unwrap();
        }

        if self.round {
            for i in 0..self.bricks.len() {
                if self.bricks[i].life <= 0 {
                    self.index.push(i);
                }
            }

            for i in self.index.iter().rev() {
                self.bricks.remove(*i);
            }
            self.index.clear();

            for ball in &(self.balls) {
                canvas.copy(&ball_texture, None, ball.rect()).unwrap();
            }
        }

        canvas
    }

    pub(crate) fn get_bricks_down(&mut self) {
        for brick in self.bricks.iter_mut() {
            brick.rect.y += BRICK_SIZE as i32;
        }
    }

    pub(crate) fn is_lost(&mut self) -> bool {
        for brick in self.bricks.iter() {
            if brick.rect.y + brick.rect.height() as i32 > 585 {
                self.started = false;
                self.game_is_lost = true;
                sdl2::mixer::Channel(1).halt();
                print!("yes!");
                return true;
            }
        }
        print!("no!");
        return false;
    }
}



