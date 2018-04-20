use ggez::Context;
use ggez::GameResult;
use ggez::graphics::DrawParam;
use ggez::graphics;
use ggez::event::{Keycode};
use ggez::timer;

use ecs::{ECS, Entity, EntityProperties, SystemUpdate, SystemDraw};
use ecs::{SystemKeyboard, KeyboardEventData};
use components::Components;
use components::{Position, Velocity, Graphics, DrawEntity};

use std::time::{Duration, SystemTime};

pub struct MoveSystem;

impl SystemUpdate for MoveSystem {
	fn system_update (&mut self)
	{

	}
	fn update (&self, _ctx:&mut Context, entity: &mut Entity)
	{
		if let (&mut Some(ref mut p), &mut Some(ref mut v), &mut Some(ref mut g)) = (&mut entity.components.position, &mut entity.components.velocity, &mut entity.components.graphics) {
		    //println!("Move position x:{:?}, y:{:?}, velocity x:{:?}, y:{:?}", &p.x, &p.y, &v.x, &v.y);
		    p.x = p.x + v.x;
		    p.y = p.y + v.y;

		    if p.x > 939.0 { p.x = -939.0; }
		    if p.y > 678.0 { p.y = -678.0; }

		    g.transform.dest = graphics::Point2::new(p.x, p.y);
		}
	}
}

pub struct  DrawSystem;

impl SystemDraw for DrawSystem{
	fn draw (&self, ctx: &mut Context, entity: &mut Entity)
	{
		if let Some(ref mut g) = entity.components.graphics {
			match g.draw {
				DrawEntity::Image(ref i) => graphics::draw_ex(ctx, i, g.transform).unwrap(),
				DrawEntity::Text(ref t) => graphics::draw_ex(ctx, t, g.transform).unwrap(),
				DrawEntity::None => (),
			}
		}
	}
}

pub struct SpriteSystem {
	system_time: SystemTime,
	delta_time: f32,
}

impl SpriteSystem {
	pub fn new () -> SpriteSystem {
		SpriteSystem {system_time: SystemTime::now(), delta_time:0.0 }
	}
}

impl SystemUpdate for SpriteSystem {

	fn system_update (&mut self)
	{
		self.delta_time = self.system_time.elapsed().unwrap().subsec_nanos() as f32 / 1000000000.0;
		self.system_time = SystemTime::now();
	}

	fn update (&self, _ctx: &mut Context, entity: &mut Entity)
	{
		if let (&mut Some(ref mut s), &mut Some(ref mut g)) = (&mut entity.components.spritesheet, &mut entity.components.graphics) {

			let mut animation = &mut s.animations[s.playing_animation];

			animation.time += self.delta_time;
			g.transform.src = animation.frames[
				match (animation.time * animation.fps).floor() as usize {
				    i if i >= animation.frames.len() => { animation.time = 0.0; 0 },
					i => i,
				}];
		}
	}
}

pub struct FireSystem;

impl SystemKeyboard for FireSystem 
{
	   // Handle key events.  These just map keyboard events
    // and alter our input state appropriately.
    fn key_down_event(&mut self, ctx: &mut Context, entity: &mut Entity, data:KeyboardEventData) {
        match data.keycode {
            Keycode::Up => {
            }
            Keycode::Left => {
            }
            Keycode::Right => {
            }
            Keycode::Space => {
            	println!("fire!");
            }
            Keycode::P => {
                let img = graphics::screenshot(ctx).expect("Could not take screenshot");
                img.encode(ctx, graphics::ImageFormat::Png, "/screenshot.png")
                    .expect("Could not save screenshot");
            }
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, entity: &mut Entity, data:KeyboardEventData) {
        match data.keycode {
            Keycode::Up => {
            }
            Keycode::Left | Keycode::Right => {
            }
            Keycode::Space => {
            }
            _ => (), // Do nothing
        }
    }
}
