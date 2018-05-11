extern crate sdl2;
use std::path::Path;
use std::time;

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
//use std::time::Duration;


struct Sprite {
    texture_index: usize,
    animations: Vec<Vec<Rect>>,
    playing_animation: usize,
    playing_animation_frame: usize,
}

impl Sprite {
    fn single_animation (texture_index: usize, animation: Vec<Rect>) -> Sprite {

        let mut new_animations = Vec::new();
        new_animations.push (animation);

        Sprite {
            texture_index: texture_index,
            animations: new_animations,
            playing_animation: 0,
            playing_animation_frame: 0,
        }
    }
}

impl SpriteAnimations for Sprite {

    fn play (&mut self, animation_index: usize) {
        self.playing_animation = animation_index;
        self.playing_animation_frame = 0;
    }

    fn add_animation (&mut self, animation_frames: Vec<Rect>) -> usize {
        self.animations.push(animation_frames);
        self.animations.len() - 1
    }

    fn draw_current_frame (&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>, screen_dimensions: Rect) {
        let frame = self.animations[self.playing_animation][self.playing_animation_frame];
        canvas.copy_ex(&textures[self.texture_index], Some(frame), Some(screen_dimensions), 0.0, None, false, false).unwrap();
    }

    /*
    fn center_destenation_on (&mut self, x: i32, y: i32){
        self.dest_rect.center_on(Point::new(x,y));
    }
    fn set_frame (&mut self, index: usize){

        let fwi = self.source_rect.w * index as i32;
        let fh = self.source_rect.h;
        self.source_rect.set_x(fwi % self.dest_rect.w);
        self.source_rect.set_y((fwi as f32 / self.dest_rect.w as f32).floor() as i32 * fh);
    }
    fn set_position_x (&mut self, x: i32){
        self.dest_rect.set_x(x);
    }
    fn draw (&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>) {
        canvas.copy_ex(&textures[self.texture_index], Some(self.source_rect), Some(self.dest_rect), 0.0, None, false, false).unwrap();
    }*/
}

impl Updater for Sprite {
    fn update (&mut self, delta_time: i32) {
        self.playing_animation_frame = (delta_time / 100) as usize % self.animations[self.playing_animation].len()
    }
}

//--

struct GamePosition {
    position: Point,
}

impl GamePosition {
    fn new (x: i32, y: i32) -> GamePosition {
        GamePosition { position: Point::new (x, y) }
    }
}

impl Location for GamePosition {
    fn set_position (&mut self, x: i32, y: i32){
        self.position.x = x;
        self.position.y = y;
    }

    fn to_screen_position(&self) -> Point {
        Point::new (self.position.x, self.position.y)
    }
}

impl Updater for GamePosition {
    fn update (&mut self, delta_time: i32) {
        self.position.x += delta_time / 1000;
        if self.position.x > 640 { self.position.x = 0; }
    }
}

//--

struct GameSize {
    size: Point,
}

impl GameSize {
    fn new (x: i32, y: i32) -> GameSize {
        GameSize { size: Point::new (x, y) }
    }
}

impl Size for GameSize {

    fn set_size (&mut self, width: i32, height: i32) {
        self.size.x = width;
        self.size.y = height;
    }

    fn to_screen_size(&self) -> Point {
        Point::new (self.size.x, self.size.y)
    }
}

impl Updater for GameSize {
    fn update (&mut self, delta_time: i32) {
    }
}

//--

trait Updater {
    fn update (&mut self, delta_time: i32);
}

trait SpriteAnimations : Updater {
    fn play(&mut self, animation_index: usize);
    fn add_animation (&mut self, animation_frames: Vec<Rect>) -> usize;
    fn draw_current_frame (&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>, screen_dimensions: Rect);
}

trait Location : Updater {
    fn set_position (&mut self, x: i32, y: i32);
    fn to_screen_position(&self) -> Point;
}

trait Size : Updater {
    fn set_size (&mut self, width: i32, height: i32);
    fn to_screen_size(&self) -> Point;
}

//--

struct Character {
    sprite_animations: Box<SpriteAnimations>,
    location: Box<Location>,
    size: Box<Size>,
}

impl Character {

    fn update (&mut self, delta_time: i32){
        self.sprite_animations.update(delta_time);
        self.location.update(delta_time);
    }

    fn draw(&self, canvas: &mut Canvas<Window>, textures: &Vec<Texture>) {
        let p = self.location.to_screen_position();
        let s = self.size.to_screen_size();
        self.sprite_animations.draw_current_frame(canvas, textures, Rect::new(p.x, p.y, s.x as u32, s.y as u32));
    }
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2", 640, 480)
        .position_centered().build().unwrap();

    let mut canvas = window.into_canvas()
        .accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,60,80,255));

    let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // animation sheet and extras are available from
    // https://opengameart.org/content/a-platformer-in-the-forest
    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp")).unwrap();
    let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

    let mut textures: Vec<Texture> = Vec::new();
    textures.push (texture);

    let frames_per_anim = 4;
    //let sprite_tile_size = (32,32);

    let mut characters: Vec<Character> = Vec::new();


    let baby_animation = vec![Rect::new(0, 0, 32, 32),Rect::new(32, 0, 32, 32),Rect::new(64, 0, 32, 32),Rect::new(96, 0, 32, 32)];
    let king_animation = vec![Rect::new(0, 32, 32, 32),Rect::new(32,32, 32, 32),Rect::new(64, 32, 32, 32),Rect::new(96, 32, 32, 32)];
    let soldier_animation = vec![Rect::new(0, 64, 32, 32),Rect::new(32, 64, 32, 32),Rect::new(64, 64, 32, 32),Rect::new(96, 64, 32, 32)];

    // Baby - walk animation
    let mut baby = Character {
        sprite_animations: Box::new (Sprite::single_animation (0, baby_animation)),
        location: Box::new (GamePosition::new (64, 120)),
        size: Box::new (GameSize::new (128, 128)),
    };
    //let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    //let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0*4, sprite_tile_size.0*4);
    //baby.dest_rect.center_on(Point::new(-64,120));
    //baby.center_destenation_on(-64, 120);
    characters.push(baby);

    // King - walk animation
    let mut king = Character {
        sprite_animations: Box::new (Sprite::single_animation (0, king_animation)),
        location: Box::new (GamePosition::new (0, 240)),
        size: Box::new (GameSize::new (128, 128)),
    };
    //let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
    //let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0*4, sprite_tile_size.0*4);
    //king.dest_rect.center_on(Point::new(0,240));
    //king.center_destenation_on(0, 240);
    characters.push(king);

    // Soldier - walk animation
    let mut soldier = Character {
        sprite_animations: Box::new (Sprite::single_animation (0, soldier_animation)),
        location: Box::new (GamePosition::new (440, 360)),
        size: Box::new (GameSize::new (128, 128)),
    };
    //let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    //let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
    //soldier.dest_rect.center_on(Point::new(440,360));
    //soldier.center_destenation_on(440,360);
    characters.push (soldier);


    // fps counter:
    let mut num_frames = 0;
    let time = time::SystemTime::now();

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
        num_frames += 1;

        for character in &mut characters {
            character.update(ticks);
            //character.set_frame(((ticks / 100) % frames_per_anim) as usize);
            //character.set_position_x(((ticks / 14) % 768) - 128);
        }

        /*
        // set the current frame for time
        //baby.source_rect.set_x(32 * ((ticks / 100) % frames_per_anim));
        //baby.dest_rect.set_x(1 * ((ticks / 14) % 768) - 128);
        baby.set_frame(((ticks / 100) % frames_per_anim) as usize);
        baby.set_position_x(((ticks / 14) % 768) - 128);

        //king.source_rect.set_x(32 * ((ticks / 100) % frames_per_anim));
        //king.dest_rect.set_x((1 * ((ticks / 12) % 768) - 672) * -1);
        king.set_frame(((ticks / 100) % frames_per_anim) as usize +4);
        king.set_position_x((1 * ((ticks / 12) % 768) - 672) * -1);

        //soldier.source_rect.set_x(32 * ((ticks / 100) % frames_per_anim));
        //soldier.dest_rect.set_x(1 * ((ticks / 10) % 768) - 128);
        soldier.set_frame(((ticks / 100) % frames_per_anim) as usize +8);
        soldier.set_position_x(1 * ((ticks / 10) % 768) - 128);
        */

        canvas.clear();

        for character in &mut characters {
            character.draw(&mut canvas, &textures);
        }
        /*
        // copy the frame to the canvas
        //canvas.copy_ex(&texture, Some(baby.source_rect), Some(baby.dest_rect), 0.0, None, false, false).unwrap();
        //canvas.copy_ex(&texture, Some(king.source_rect), Some(king.dest_rect), 0.0, None, true, false).unwrap();
        //canvas.copy_ex(&texture, Some(soldier.source_rect), Some(soldier.dest_rect), 0.0, None, false, false).unwrap();
        baby.draw(&mut canvas, &textures);
        king.draw(&mut canvas, &textures);
        soldier.draw(&mut canvas, &textures);
        */
        canvas.present();

        //std::thread::sleep(Duration::from_millis(100));
    }

    match time.elapsed() {
       Ok(elapsed) => {
            let total_time = elapsed.as_secs() as f64+(elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
            println!("total time = {:?}, num frames = {:?}, average fps = {:?}", total_time, num_frames, num_frames as f64 / total_time);
       }
       Err(e) => {
           // an error occurred!
           println!("Error: {:?}", e);
       }
    }
    
}



/*
trait Entity {
    fn update(&mut self, _event: &Option<event::Event>, _delta_time: f64) {}
    fn draw(&self, _ctx: &mut Context, image: &graphics::Image, src: graphics::Rect) {}
}

struct GameObject {
    position: Point2,
}

impl Entity for GameObject {
    fn update (&mut self, _event: &Option<event::Event>, _delta_time: f64) {
        self.position.x += _delta_time as f32 * 10.0;
    }

    fn draw (&self, ctx: &mut Context, image: &graphics::Image, src: graphics::Rect) {

        graphics::draw_ex(ctx, image, graphics::DrawParam { src: src, dest: Point2::new (self.position.x.round(), self.position.y.round()), ..Default::default() });
        /*graphics::circle(
            ctx,
            DrawMode::Fill,
            self.position, //Point2::new(self.position, 380.0),
            10.0,
            2.0,
        ).unwrap();
    */
    }
}
*/
/*
pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("eventloop", "ggez", c).unwrap();
    let mut events = event::Events::new(ctx).unwrap();
    let mut continuing = true;

    // direct ctx asset path to asset folder
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        ctx.filesystem.mount(&path, true);
    }
    else {
        println!("Warning: coudn't mount path 'assets' to ctx filesystem");
    }

    //let mut position: f32 = 1.0;
    let spritesheed_image = graphics::Image::new(ctx, "/rifle_man_red_alert_trans.png").expect("Scene: load /rifle_man_red_alert_trans.png failed!");
    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18).expect("Scene: load /DejaVuSerif.ttf failed!");
    let rect = normalize_sprite_frame(Point2::new(312.0,149.0), graphics::Rect::new(75.0, 14.0, 16.0, 16.0));

    let mut entities: Vec<Box<Entity>> = Vec::new();
    for _i in 0..100 {
        let xpos = rand::thread_rng().gen_range(0, 900);
        let ypos = rand::thread_rng().gen_range(0, 650);

        entities.push(Box::new(GameObject{position: Point2::new(xpos as f32, ypos as f32)}));
    }
    

    while continuing {
        // Tell the timer stuff a frame has happened.
        // Without this the FPS timer functions and such won't work.
        ctx.timer_context.tick();
        
        let mut recent_event: Option<event::Event> = None;
        let delta_time = ggez::timer::duration_to_f64(ggez::timer::get_delta(ctx));

        // Handle events
        for event in events.poll() {
            ctx.process_event(&event);
            recent_event = match event {
                event::Event::Quit { .. }
                | event::Event::KeyDown {
                    keycode: Some(event::Keycode::Escape),
                    ..
                } => {
                    println!("Quitting");
                    continuing = false;
                    None
                }
                x => Some(x), //println!("Event fired: {:?}", x),
            }
        }
        
        // Update
        
        for entity in &mut entities {
            entity.update(&recent_event, delta_time);
        }

        // Draw
        graphics::clear(ctx);
        
        for entity in &mut entities {
            entity.draw(ctx, &spritesheed_image, rect);
        }
        let fps_str = &format!("Fps: {:.2}", 1.0 / delta_time);
        let fps_text = graphics::Text::new(ctx, fps_str, &font).unwrap();      
        let params = graphics::DrawParam {..Default::default() };

        graphics::draw_ex(ctx, &fps_text, params).unwrap();

        graphics::present(ctx);
        ggez::timer::yield_now();
    }
}

fn normalize_sprite_frame (image_size: Point2, mut frame: graphics::Rect) -> graphics::Rect {
    let fx = 1.0 / image_size.x;
    let fy = 1.0 / image_size.y;

    frame.x *= fx; 
    frame.y *= fy; 
    frame.w *= fx; 
    frame.h *= fy; 

    frame
}
*/