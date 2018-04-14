	extern crate ggez;

	use ggez::graphics;
	use ggez::{Context, GameResult};
	use ecs::{Entity, System};
	use ecs::Components;
	use components::CustomComponents;


	pub struct MoveSystem {

		image: ggez::graphics::Image,
	}

	impl MoveSystem {
		pub fn new (ctx: &mut Context) -> GameResult<MoveSystem> {
			let i = graphics::Image::new(ctx, "/dragon1.png")?;
			let m = MoveSystem { image: i };
			Ok(m)
		}
	}

	impl <C: Components<ComponentsType = CustomComponents>> System<C> for MoveSystem
	{
		fn update (&self, entity: &mut Entity<C>)
		{
			if let (&mut Some(ref mut p), &mut Some(ref mut v)) = (&mut entity.components.position, &mut entity.components.velocity) {
			    //println!("Move position x:{:?}, y:{:?}, velocity x:{:?}, y:{:?}", &p.x, &p.y, &v.x, &v.y);
			    p.x = p.x + v.x;
			    p.y = p.y + v.y;

			    if p.x > 939.0 { p.x = -939.0; }
			    if p.y > 678.0 { p.y = -678.0; }
			}
		}

		fn draw (&self, ctx: &mut Context, entity: &mut Entity<C>){

			if let &mut Some(ref mut p) = &mut entity.components.position {
			    let dest = graphics::Point2::new(p.x, p.y);
				graphics::draw_ex(ctx, &self.image, graphics::DrawParam{ dest: dest, ..Default::default() }).unwrap();
			}
			else {
			    unimplemented!();
			}
			
		}
	}