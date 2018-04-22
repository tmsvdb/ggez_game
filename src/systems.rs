use ggez::Context;
use ggez::GameResult;
use ggez::graphics::{DrawParam,Point2};
use ggez::graphics;
use ggez::event::{Keycode};
use ggez::timer;

use ecs::{ECS, Entity, EntityProperties, SystemUpdate, SystemDraw, SystemMouse};
use ecs::{SystemKeyboard, KeyboardEventData, MouseButtonventData, MouseMotionEventData, MouseWheelEventData};
use components::Components;
use components::{Directions, Selection, Graphics, DrawEntity};

use std::time::{Duration, SystemTime};


pub struct SelectSystem;

impl SystemMouse for SelectSystem {
	fn mouse_button_down_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData){}
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData){}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseMotionEventData){}
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseWheelEventData){}
}

pub struct MoveSystem;

impl SystemUpdate for MoveSystem {
	fn system_update (&mut self)
	{

	}
	fn update (&self, _ctx:&mut Context, entity: &mut Entity, delta_time: f32)
	{
		if let (&mut Some(ref mut direct), &mut Some(ref mut g), &mut Some(ref mut sprite)) = (&mut entity.components.directions, &mut entity.components.graphics, &mut entity.components.spritesheet) {
		    if !direct.arrived
		    {
		    	if g.transform.dest.y < direct.goto.y {
		    		g.transform.dest.y += delta_time * 3.0;
		    		if g.transform.dest.y > direct.goto.y {
		    			g.transform.dest.y = direct.goto.y;
		    		}
		    	}
		    	else if g.transform.dest.y > direct.goto.y {
		    	    g.transform.dest.y -= delta_time * 3.0;
		    	    if g.transform.dest.y < direct.goto.y {
		    			g.transform.dest.y = direct.goto.y;
		    		}
		    	}

		    	if g.transform.dest.x < direct.goto.x {
		    		g.transform.dest.x += delta_time * 3.0;
		    		if g.transform.dest.x > direct.goto.x {
		    			g.transform.dest.x = direct.goto.x;
		    		}
		    	}
		    	else if g.transform.dest.x > direct.goto.x {
		    	    g.transform.dest.x -= delta_time * 3;
		    	    if g.transform.dest.x < direct.goto.x {
		    			g.transform.dest.x = direct.goto.x;
		    		}
		    	}

		    	if  g.transform.dest.x == direct.goto.x &&  g.transform.dest.y == direct.goto.y {
		    		direct.arrived = true;
		    		sprite.playing_animation = 0;
		    	}

		    } else {

		    	if sprite.playing_animation != 0 {
		    		sprite.playing_animation = 0;
		    	}
		    }
		}
	}
}

impl SystemMouse for MoveSystem {
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData){
    	if let (&mut Some(ref mut direct), &mut Some(ref mut select)) = (&mut _entity.components.directions, &mut _entity.components.selection){
    		if select.selected {
    			direct.goto = Point2::new (_data.x as f32, _data.y as f32);
    			direct.arrived = false;
    		}
    	}
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData){}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseMotionEventData){}
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseWheelEventData){}
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

	fn update (&self, _ctx: &mut Context, entity: &mut Entity, delta_time: f32)
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
    fn key_down_event(&mut self, ctx: &mut Context, _entity: &mut Entity, _data:KeyboardEventData) {
        match _data.keycode {
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

    fn key_up_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data:KeyboardEventData) {
        match _data.keycode {
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
