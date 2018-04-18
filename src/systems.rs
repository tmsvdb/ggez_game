use ggez::Context;
use ggez::GameResult;
use ggez::graphics::DrawParam;
use ggez::graphics;
use ggez::event::{Keycode};

use ecs::{ECS, Entity, EntityProperties, SystemUpdate, SystemDraw};
use ecs::{SystemKeyboard, KeyboardEventData};
use components::Components;
use components::{Position, Velocity, Graphics, DrawEntity};

pub struct MoveSystem;

impl SystemUpdate for MoveSystem {
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


/*	
	extern crate ggez;

	use ggez::graphics;
	use ggez::graphics::{DrawParam};
	use ggez::{Context, GameResult};
	use ecs::{Entity, SystemUpdate, EntityProperties, DrawEntity};
	use ecs::Components;
	use ecs::ECS;
	use components::CustomComponents;
	use components::{Position, Velocity };


	pub struct MoveSystem;

	impl MoveSystem{
		pub fn new <C: Components<ComponentsType = CustomComponents>> (ctx: &mut Context, ecs: &mut ECS<C>) -> GameResult<MoveSystem> {
			let i = graphics::Image::new(ctx, "/dragon1.png")?;


		    // create startup entities
			ecs.register_entity(Entity::new_graphic(
				EntityProperties{
					name:"dragon".to_string(), 
					drawable:DrawEntity::Image(i), 
					transform: DrawParam { ..Default::default() }
				}, 
				CustomComponents { 
					position: Some(Position { x:-939.0, y:0.0 }), 
					velocity: Some(Velocity { x:3.0, y:0.0 })
			}));

			Ok(MoveSystem)
		}
	}

	impl <C: Components<ComponentsType = CustomComponents>> SystemUpdate<C> for MoveSystem
	{
		fn update (&self, entity: &mut Entity<C>)
		{
			if let (&mut Some(ref mut p), &mut Some(ref mut v)) = (&mut entity.components.position, &mut entity.components.velocity) {
			    //println!("Move position x:{:?}, y:{:?}, velocity x:{:?}, y:{:?}", &p.x, &p.y, &v.x, &v.y);
			    p.x = p.x + v.x;
			    p.y = p.y + v.y;

			    if p.x > 939.0 { p.x = -939.0; }
			    if p.y > 678.0 { p.y = -678.0; }

			    entity.properties.transform.dest = graphics::Point2::new(p.x, p.y);
			}
		}

		/*
		fn draw (&self, ctx: &mut Context, entity: &mut Entity<C>){

			if let &mut Some(ref mut p) = &mut entity.components.position {
			    let dest = graphics::Point2::new(p.x, p.y);
				graphics::draw_ex(ctx, &self.image, graphics::DrawParam{ dest: dest, ..Default::default() }).unwrap();
			}
			else {
			    unimplemented!();
			}
			
		}
		*/
	}
*/