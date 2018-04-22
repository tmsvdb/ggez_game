use ggez::Context;
use ggez::GameResult;
use ggez::graphics::{DrawParam,Point2,Rect};
use ggez::graphics;
use ggez::event::{Keycode, MouseButton};
use ggez::timer;

use ecs::{ECS, Entity, EntityProperties, SystemUpdate, SystemDraw, SystemMouse};
use ecs::{SystemKeyboard, KeyboardEventData, MouseButtonventData, MouseMotionEventData, MouseWheelEventData};
use components::Components;
use components::{Directions, Selection, Spritesheet, Sprite};

use std::time::{Duration, SystemTime};
use assets::Assets;

pub struct SelectSystem;

impl SystemMouse for SelectSystem {
	fn mouse_button_down_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData)
	{
		if _data.button == MouseButton::Left {

			println!("button down by select system");

			if let (&mut Some(ref mut select), &mut Some(ref mut sheet)) = (&mut _entity.components.selection, &mut _entity.components.spritesheet) {
				
				select.sprite.transform.x = sheet.sprite.transform.x;
				select.sprite.transform.y = sheet.sprite.transform.y;
				let t = sheet.sprite.transform;
				select.selected = _data.x as f32 >= t.x && _data.y as f32 <= t.x + t.w 
									&& _data.y as f32 >= t.y && _data.y as f32 <= t.y + t.h;

				println!("button pos=({:?},{:?}), ent rect=({:?},{:?},{:?},{:?}", _data.x, _data.y, t.x, t.y, t.w, t.h);
			}
		}
	}
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData){}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseMotionEventData){}
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseWheelEventData){}
}

impl SystemDraw for SelectSystem {
	fn draw (&self, ctx: &mut Context, entity: &mut Entity, _assets: &Assets)
	{
		if let Some(ref mut s) = entity.components.selection {

			//println!("get asset at {:?}", s.sprite.image);
			if s.selected
			{
				let image = _assets.get_image_at(s.sprite.image).expect("SelectSystem: image not found in assets!");
				let params = DrawParam { src: s.sprite.frame, dest: Point2::new(s.sprite.transform.x, s.sprite.transform.y), ..Default::default()	};
				graphics::draw_ex(ctx, image, params).unwrap();
			}
		}
	}
}

pub struct MoveSystem;

impl SystemUpdate for MoveSystem {
	fn system_update (&mut self)
	{

	}
	fn update (&self, _ctx:&mut Context, entity: &mut Entity, delta_time: f32)
	{
		if let (&mut Some(ref mut direct), &mut Some(ref mut s)) = (&mut entity.components.directions, &mut entity.components.spritesheet) {
		    if !direct.arrived
		    {
		    	s.sprite.transform.y = match s.sprite.transform.y {
		    		y if y < direct.goto.y && y + (delta_time * 10.0) <= direct.goto.y => y + (delta_time * 10.0),
		    		y if y > direct.goto.y && y - (delta_time * 10.0) >= direct.goto.y => y - (delta_time * 10.0),
		    		_ => direct.goto.y
		    	};

		    	s.sprite.transform.x = match s.sprite.transform.x {
		    		x if x < direct.goto.x && x + (delta_time * 10.0) <= direct.goto.x => x + (delta_time * 10.0),
		    		x if x > direct.goto.x && x - (delta_time * 10.0) >= direct.goto.x => x - (delta_time * 10.0),
		    		_ => direct.goto.x
		    	};

		    	if  s.sprite.transform.x == direct.goto.x &&  s.sprite.transform.y == direct.goto.y {
		    		direct.arrived = true;
		    		s.playing_animation = 0;
		    	}
		    	else if s.playing_animation != 1 {
		    	    s.playing_animation = 1;
		    	}

		    } else if s.playing_animation != 0 {
		    	
		    	s.playing_animation = 0;
		    }
		}
	}
}

impl SystemMouse for MoveSystem {
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData){
    	if _data.button == MouseButton::Right {
	    	if let (&mut Some(ref mut direct), &mut Some(ref mut select)) = (&mut _entity.components.directions, &mut _entity.components.selection){
	    		if select.selected {
	    			direct.goto = Point2::new (_data.x as f32, _data.y as f32);
	    			direct.arrived = false;
	    		}
	    	}
    	}
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData){}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseMotionEventData){}
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseWheelEventData){}
}

/*
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
}*/

pub struct SpriteSystem;

impl SystemUpdate for SpriteSystem {

	fn system_update (&mut self) {}

	fn update (&self, _ctx: &mut Context, entity: &mut Entity, delta_time: f32)
	{
		if let Some(ref mut s) = entity.components.spritesheet {

			let animation = &mut s.animations[s.playing_animation];

			animation.time += delta_time;
			s.sprite.frame = animation.frames[
				match (animation.time * animation.fps).floor() as usize {
				    i if i >= animation.frames.len() => { animation.time = 0.0; 0 },
					i => i,
				}];
		}
	}
}

impl SystemDraw for SpriteSystem {
	fn draw (&self, ctx: &mut Context, entity: &mut Entity, _assets: &Assets)
	{
		if let Some(ref mut s) = entity.components.spritesheet {

			//println!("get asset at {:?}", s.sprite.image);

			let image = _assets.get_image_at(s.sprite.image).expect("SpriteSystem: image not found in assets!");
			let params = DrawParam { src: s.sprite.frame, dest: Point2::new(s.sprite.transform.x, s.sprite.transform.y), ..Default::default()};
			graphics::draw_ex(ctx, image, params).unwrap();
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
