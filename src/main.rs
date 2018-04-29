extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};

use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;

use rand::Rng;

fn  init<'a>()-> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.clear();
    renderer.present();

    (renderer, event_pump)
}

fn main() {
    let (mut r,mut e) = init();

    //let mut x = 0;
    //let y = 20;
    let white = Color::RGB(255, 255, 255);
    let red = Color::RGB(255, 0, 0);

    let st = time::SystemTime::now();
    let mut last_time:f32 = 0.0;

    let mut objects: Vec<GameObject> = Vec::new();
    for _i in 0..1000 {
        let xpos = rand::thread_rng().gen_range(0, 900);
        let ypos = rand::thread_rng().gen_range(0, 650);

        objects.push (GameObject{ rect: Rect::new(xpos, ypos, 10, 10)})
    }

    'running:loop {
        for event in e.poll_iter() {
            match event {
                Event::KeyDown {
                  keycode: Some(Keycode::Escape), .. 
                } => { break 'running },
                _ => {}
            }
        }
        r.set_draw_color(white);
        r.clear();
        r.set_draw_color(red);
        for go in &objects {
            r.fill_rect(go.rect);
        }
        r.present();

        let delta_time = st.elapsed().unwrap().subsec_nanos()as f32 - last_time;
        last_time = st.elapsed().unwrap().subsec_nanos() as f32;

        for go in &mut objects {
            let rect = (go.rect.x() + 1) % 400; //(delta_time / 10000000.0) as i32) % 400;
            go.rect.set_x(rect);
        }
        //x = (x + 5) % 400;
        //thread::sleep(time::Duration::from_millis(50));
        //let delta_time = st.elapsed().unwrap().subsec_nanos()as f32 - last_time;
        //last_time = st.elapsed().unwrap().subsec_nanos() as f32;
        //println!("elapsed {:?}", delta_time);
    }
}

struct GameObject{
    rect: Rect,
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