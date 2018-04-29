use ggez::Context;
use ggez::GameResult;
use ggez::graphics::{DrawParam,Point2,Rect};
use ggez::graphics;
use ggez::event::{Keycode, MouseButton};
use ggez::timer;

use ecs::{ECS, Entity, EntityProperties, SystemUpdate, SystemDraw, SystemMouse};
use ecs::{SystemKeyboard, KeyboardEventData, MouseButtonventData, MouseMotionEventData, MouseWheelEventData};
use components::Components;
use components::{Navigation, Selection, Spritesheet, Sprite};

use std::time::{Duration, SystemTime};
use assets::Assets;

pub struct DebugSystem;
impl SystemUpdate for DebugSystem {

	fn system_update(&mut self) {}
	/// update the entity using this system
	fn update (&self, ctx: &mut Context, entity: &mut Entity, _delta_time: f32){
		if let Some(ref mut d) = entity.components.debug {
			d.fps_str = format!("Fps: {}", timer::get_fps(ctx));			
		}		
	}	
}

impl SystemDraw for DebugSystem {
	fn draw (&self, ctx: &mut Context, entity: &mut Entity, _assets: &Assets)
	{
		if let Some(ref mut d) = entity.components.debug {
			
    		let fps_text = graphics::Text::new(ctx, &d.fps_str, &d.font).unwrap();		
			let params = DrawParam {..Default::default() };

    		graphics::draw_ex(ctx, &fps_text, params).unwrap();
		}
	}
}

pub struct SelectSystem;

impl SystemMouse for SelectSystem {
	fn mouse_button_down_event(&mut self, _ctx: &mut Context, _entity: &mut Entity, _data: MouseButtonventData)
	{
		if _data.button == MouseButton::Left {

			//println!("button down by select system");

			if let (&mut Some(ref mut select), &mut Some(ref mut sheet)) = (&mut _entity.components.selection, &mut _entity.components.spritesheet) {
				
				select.sprite.transform.x = sheet.sprite.transform.x;
				select.sprite.transform.y = sheet.sprite.transform.y;
				let t = sheet.sprite.transform;
				select.selected = _data.x as f32 >= t.x && _data.y as f32 <= t.x + t.w 
									&& _data.y as f32 >= t.y && _data.y as f32 <= t.y + t.h;

				//println!("button pos=({:?},{:?}), ent rect=({:?},{:?},{:?},{:?}", _data.x, _data.y, t.x, t.y, t.w, t.h);
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
		if let (&mut Some(ref mut nav), &mut Some(ref mut select)) = (&mut entity.components.navigation, &mut entity.components.selection) {

			//println!("get asset at {:?}", s.sprite.image);
			if select.selected
			{
				let image = _assets.get_image_at(select.sprite.image).expect("SelectSystem: image not found in assets!");
				let params = DrawParam { src: select.sprite.frame, dest:Point2::new(nav.location.x.round(), nav.location.y.round()), ..Default::default()	};
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
		if let (&mut Some(ref mut nav), &mut Some(ref mut s)) = (&mut entity.components.navigation, &mut entity.components.spritesheet) {
		    if !nav.arrived
		    {
		    	nav.location.y = match nav.location.y {
		    		y if y < nav.goto.y && y + (delta_time * 10.0) <= nav.goto.y => y + (delta_time * 10.0),
		    		y if y > nav.goto.y && y - (delta_time * 10.0) >= nav.goto.y => y - (delta_time * 10.0),
		    		_ => nav.goto.y
		    	};

		    	nav.location.x = match nav.location.x {
		    		x if x < nav.goto.x && x + (delta_time * 10.0) <= nav.goto.x => x + (delta_time * 10.0),
		    		x if x > nav.goto.x && x - (delta_time * 10.0) >= nav.goto.x => x - (delta_time * 10.0),
		    		_ => nav.goto.x
		    	};

		    	if  nav.location.x == nav.goto.x &&  nav.location.y == nav.goto.y {
		    		nav.arrived = true;
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
	    	if let (&mut Some(ref mut nav), &mut Some(ref mut select)) = (&mut _entity.components.navigation, &mut _entity.components.selection){
	    		if select.selected {
	    			nav.goto = Point2::new (_data.x as f32, _data.y as f32);
	    			nav.arrived = false;
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
		if let Some(ref mut sheet) = entity.components.spritesheet {

			let animation = &mut sheet.animations[sheet.playing_animation];

			animation.time += delta_time;
			sheet.sprite.frame = animation.frames[
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
		if let (&mut Some(ref mut nav), &mut Some(ref mut sheet)) = (&mut entity.components.navigation, &mut entity.components.spritesheet) {

			//println!("get asset at {:?}", s.sprite.image);

			let image = _assets.get_image_at(sheet.sprite.image).expect("SpriteSystem: image not found in assets!");
			let params = DrawParam { src: sheet.sprite.frame, dest: Point2::new(nav.location.x.round(), nav.location.y.round()), ..Default::default()};
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
