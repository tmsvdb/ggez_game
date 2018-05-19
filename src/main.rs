extern crate sdl2;
use strategies::Animations;
use strategies::animations::sprite::Sprite;
use strategies::Updater;
use strategies::Location;
use strategies::Size;
use units::Unit;
use std::path::Path;

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
//use std::time::Duration;

mod units;
mod strategies;

struct GamePosition {
    x: f32,
    y: f32,
}

impl GamePosition {
    fn new (x: f32, y: f32) -> GamePosition {
        GamePosition { x: x, y: y }
    }
}

impl Location for GamePosition {
    fn set_position (&mut self, x: f32, y: f32){
        self.x = x;
        self.y = y;
    }

    fn to_screen_position(&self) -> Point {
        Point::new (self.x as i32, self.y as i32)
    }
}

impl Updater for GamePosition {
    fn update (&mut self, delta_time: f32) {
        self.x += 100.0 * delta_time;
        if self.x > 640.0 { self.x = 0.0; }
    }
}

//--

struct GameSize {
    width: f32,
    height: f32,
}

impl GameSize {
    fn new (w: f32, h: f32) -> GameSize {
        GameSize { width: w, height: h }
    }
}

impl Size for GameSize {

    fn set_size (&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    fn to_screen_size(&self) -> Point {
        Point::new (self.width as i32, self.height as i32)
    }
}

impl Updater for GameSize {
    fn update (&mut self, delta_time: f32) {
    }
}

//--
/*
trait Updater {
    fn update (&mut self, delta_time: f32);
}

trait SpriteAnimations : Updater {
    fn play(&mut self, animation_index: usize);
    fn add_animation (&mut self, animation: Animation) -> usize;
    fn draw_current_frame (&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>, screen_dimensions: Rect);
}

trait Location : Updater {
    fn set_position (&mut self, x: f32, y: f32);
    fn to_screen_position(&self) -> Point;
}

trait Size : Updater {
    fn set_size (&mut self, width: f32, height: f32);
    fn to_screen_size(&self) -> Point;
}*/

//--
/*
struct Character {
    sprite_animations: Box<SpriteAnimations>,
    location: Box<Location>,
    size: Box<Size>,
}

impl Character {

    fn update (&mut self, delta_time: f32){
        self.sprite_animations.update(delta_time);
        self.location.update(delta_time);
    }

    fn draw(&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>) {
        let p = self.location.to_screen_position();
        let s = self.size.to_screen_size();
        self.sprite_animations.draw_current_frame(canvas, textures, Rect::new(p.x, p.y, s.x as u32, s.y as u32));
    }
}
*/


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered().build().unwrap();
    let mut canvas = window.into_canvas()
        .accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,60,80,255));

    let timer = sdl_context.timer().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp")).unwrap();
    let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();
    let mut textures: Vec<Texture> = Vec::new();
    textures.push (texture);

    let mut units: Vec<Unit> = Vec::new();

    let baby_animation = vec![Rect::new(0, 0, 32, 32),Rect::new(32, 0, 32, 32),Rect::new(64, 0, 32, 32),Rect::new(96, 0, 32, 32)];
    let king_animation = vec![Rect::new(0, 32, 32, 32),Rect::new(32,32, 32, 32),Rect::new(64, 32, 32, 32),Rect::new(96, 32, 32, 32)];
    let soldier_animation = vec![Rect::new(0, 64, 32, 32),Rect::new(32, 64, 32, 32),Rect::new(64, 64, 32, 32),Rect::new(96, 64, 32, 32)];

    // Baby - walk animation
    let baby = Unit {
        sprite_animations: Box::new (Sprite::single_animation (0, 5, baby_animation)),
        location: Box::new (GamePosition::new (64.0, 120.0)),
        size: Box::new (GameSize::new (128.0, 128.0)),
    };
    units.push(baby);

    // King - walk animation
    let king = Unit {
        sprite_animations: Box::new (Sprite::single_animation (0, 10, king_animation)),
        location: Box::new (GamePosition::new (0.0, 240.0)),
        size: Box::new (GameSize::new (128.0, 128.0)),
    };
    units.push(king);

    // Soldier - walk animation
    let soldier = Unit {
        sprite_animations: Box::new (Sprite::single_animation (0, 15, soldier_animation)),
        location: Box::new (GamePosition::new (440.0, 360.0)),
        size: Box::new (GameSize::new (128.0, 128.0)),
    };
    units.push (soldier);

    // play the game
    run_game(event_pump, timer, units, canvas, textures);
}

fn run_game (mut event_pump: sdl2::EventPump, mut timer: sdl2::TimerSubsystem, mut units: Vec<Unit>, mut canvas: Canvas<Window>, textures: Vec<Texture>)
{
    let mut delta_time: f32 = 0.025;
    let mut last_ticks: i32 = 0;
    let mut poll_frames: i32 = 0;
    let mut poll_dt: f32 = 0.0;
    let mut running = true;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    running = false;
                },
                _ => {}
            }
        }

        // delta time in miliseconds
        let ticks = timer.ticks() as i32;
        delta_time = (ticks - last_ticks) as f32 / 1_000.0;
        if delta_time < 0.0001 { delta_time = 0.0001; }
        last_ticks = ticks;

        // update all
        for character in &mut units {
            character.update(delta_time);
        }

        // draw all
        canvas.clear();
        for character in &mut units {
            character.draw(&mut canvas, &textures);
        }
        canvas.present();

        // poll fps every 5 seconds
        if poll_dt > 5.0 {
            println!("fps = {:?}", 1.0 / (poll_dt / poll_frames as f32));
            poll_frames = 0;
            poll_dt = 0.0;
        } else {
            poll_frames += 1;
            poll_dt += delta_time;
        }
    }
}
