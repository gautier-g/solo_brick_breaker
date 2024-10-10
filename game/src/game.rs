extern crate sdl2;
extern crate rand;

use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use crate::utils::{WINDOW_WIDTH, WINDOW_HEIGHT};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};

use crate::utils::Brick;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub(crate) struct DrawnContent {
    pub(crate) displayed_in_game: bool,
    pub(crate) displayed_in_pause: bool,
    pub(crate) name: Option<String>,
    pub(crate) rect: Rect,
    pub(crate) color: Color
}

pub(crate) struct TexturedContent<'a> {
    pub(crate) displayed_in_game: bool,
    pub(crate) displayed_in_pause: bool,
    pub(crate) name: Option<String>,
    pub(crate) texture: Texture<'a>,
    pub(crate) src: Option<Rect>,
    pub(crate) dst: Option<Rect>
}

pub(crate) struct Game<'a> {
    pub(crate) started: bool,
    pub(crate) paused: bool,
    pub(crate) drawn: Vec<DrawnContent>,
    pub(crate) textured: Vec<TexturedContent<'a>>
}

impl<'a> Game<'a> {
    pub (crate) fn load_content(&mut self, ttf_context: &Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>, level : String) -> Vec<Brick> {
        let font = ttf_context.load_font("fonts/Marlboro.ttf", 128).unwrap();

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
            name: Some(String::from_str("menu_title").unwrap()),
            texture: texture,
            src: None,
            dst: Some(rect!(WINDOW_WIDTH/2 - width/6, 50, width/3, height/3))
        };

        self.textured.push(title_textured_content);

        let start_button = DrawnContent {
            displayed_in_game: false,
            displayed_in_pause: false,
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
            name: None,
            texture: start_texture,
            src: None,
            dst: Some(rect!(225, 225, 150, 50))
        };

        let settings_button = DrawnContent {
            displayed_in_game: false,
            displayed_in_pause: false,
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
            name: None,
            texture: settings_texture,
            src: None,
            dst: Some(rect!(225, 375, 150, 50))
        };

        let credits_button = DrawnContent {
            displayed_in_game: false,
            displayed_in_pause: false,
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
            name: None,
            texture: credits_texture,
            src: None,
            dst: Some(rect!(225, 525, 150, 50))
        };

        let pause_button = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
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
            name: None,
            texture: pause_texture,
            src: None,
            dst: Some(rect!(425, 20, 140, 40))
        };

        let resume_button = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: true,
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
            name: None,
            texture: resume_texture,
            src: None,
            dst: Some(rect!(225, 225, 150, 50))
        };

        let giveup_button = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: true,
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
            name: Some(String::from_str("left_bar").unwrap()),
            rect: rect!(100, 75, 5, 600),
            color: Color::RGB(50, 50, 255)
        };

        let right_bar_outside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("right_bar").unwrap()),
            rect: rect!(WINDOW_WIDTH-105, 75, 5, 600),
            color: Color::RGB(50, 50, 255)
        };
        
        let left_bar_inside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("left_bar").unwrap()),
            rect: rect!(101, 76, 3, 598),
            color: Color::RGB(0, 0, 0)
        };

        let right_bar_inside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("right_bar").unwrap()),
            rect: rect!(WINDOW_WIDTH-104, 76, 3, 598),
            color: Color::RGB(0, 0, 0)
        };

        let top_bar_outside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("top_bar").unwrap()),
            rect: rect!(100, 75, 400, 5),
            color: Color::RGB(50, 50, 255)
        };

        let top_bar_inside = DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
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

        //################################################################################################################################################################################
        
        let file = File::open(level).unwrap();
        let reader = io::BufReader::new(file);
    
        let mut bricks:Vec<Brick> = Vec::new();
        let mut j= 0;

        for line in reader.lines() {
            match line {
                Ok(content) => {                
                    let tmp : Vec<&str> =  content.split(" ").collect();
                    for i in 0..tmp.len() {
                        match tmp[i].parse::<u32>() {
                            Ok(0) => {},
                            Ok(nombre) => {
                             
                                bricks.push(Brick::new(i as i32,j as i32 , nombre as i32))
                            },
                            _ => {}, 
                        }
                    }
                }
                _ => {}, 
            }
            j = j + 1;
        }
        
//################################################################################################################################################################################
        self.drawn.push(DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("player_weapon").unwrap()),
            rect: rect!(WINDOW_WIDTH/2 - 16, 3*WINDOW_HEIGHT/4 + 50, 32, 32),
            color: Color::RGB(255, 255, 255)
        });
        self.drawn.push(DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("player_weapon").unwrap()),
            rect: rect!(WINDOW_WIDTH/2 - 15, 3*WINDOW_HEIGHT/4 + 51, 30, 30),
            color: Color::RGB(0, 0, 0)
        });
        self.drawn.push(DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("player_weapon").unwrap()),
            rect: rect!(WINDOW_WIDTH/2 - 14, 3*WINDOW_HEIGHT/4 + 52, 28, 28),
            color: Color::RGB(255, 255, 255)
        });
        self.drawn.push(DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("player_weapon").unwrap()),
            rect: rect!(WINDOW_WIDTH/2 - 13, 3*WINDOW_HEIGHT/4 + 53, 26, 26),
            color: Color::RGB(0, 0, 0)
        });
        self.drawn.push(DrawnContent {
            displayed_in_game: true,
            displayed_in_pause: false,
            name: Some(String::from_str("player_weapon").unwrap()),
            rect: rect!(WINDOW_WIDTH/2 - 12, 3*WINDOW_HEIGHT/4 + 54, 24, 24),
            color: Color::RGB(255, 255, 255)
        });
        bricks
    }

    pub(crate) fn display_menu(&self, mut can: Canvas<Window>) -> Canvas<Window> {
        for content in self.drawn.iter() {
            if !content.displayed_in_game {
                let _ = can.set_draw_color(content.color);
                let _ = can.fill_rect(content.rect);
            }
        }
        
        for content in self.textured.iter() {
            if !content.displayed_in_game {
                let _ = can.copy(&content.texture, content.src, content.dst);
            }
        }
        can
    }

    pub(crate) fn display_pause(&self, mut can: Canvas<Window>) -> Canvas<Window> {
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

    pub(crate) fn display_game(&self, mut can: Canvas<Window>, ttf_context: &Sdl2TtfContext, texture_creator: &'a TextureCreator<WindowContext>,bricks : &Vec<&mut Brick>) -> Canvas<Window> {
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

        let font = ttf_context.load_font("fonts/Marlboro.ttf", 128).unwrap();

        for brick in bricks {
            let brick_surface = font
            .render(brick.life.to_string().as_str())
            .blended(Color::RGBA(0, 255, 0, 255))
            .map_err(|e| e.to_string()).unwrap();

            let brick_texture = texture_creator
                .create_texture_from_surface(&brick_surface)
                .map_err(|e| e.to_string()).unwrap();

            let _ = can.fill_rect(brick.rect);
            let _ = can.copy(&brick_texture,None,brick.rect);
        }


        

        //###########################################################################################
        can
    }

    pub(crate) fn act_drawn(mut self, x: i32, y: i32) -> Game<'a> {
        for content in self.drawn.iter() {
            if (content.rect.x() <= x) && (x <= content.rect.x() + content.rect.width() as i32) && (content.rect.y() <= y) && (y <= content.rect.y() + content.rect.height() as i32) {
                if content.name == Some(String::from_str("menu_start").unwrap()) {
                    self.started = true;
                }
                if content.name == Some(String::from_str("pause_button").unwrap()) {
                    self.paused = true;
                }
                if content.name == Some(String::from_str("pause_resume").unwrap()) {
                    self.paused = false;
                }
                if content.name == Some(String::from_str("pause_giveup").unwrap()) {
                    self.started = false;
                    self.paused = false;
                }
            }
        }
        self
    }
}



