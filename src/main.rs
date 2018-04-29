extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;

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

    let frames_per_anim = 4;
    let sprite_tile_size = (32,32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_0.center_on(Point::new(-64,120));

    // King - walk animation
    let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_1.center_on(Point::new(0,240));

    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_2.center_on(Point::new(440,360));

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

        let ticks = timer.ticks() as i32;

        // set the current frame for time
        source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);

        source_rect_1.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_1.set_x((1 * ((ticks / 12) % 768) - 672) * -1);

        source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_2.set_x(1 * ((ticks / 10) % 768) - 128);

        canvas.clear();
        // copy the frame to the canvas
        canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false).unwrap();
        canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, true, false).unwrap();
        canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false).unwrap();
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
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