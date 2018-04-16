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
			ecs.register_entity(Entity::new(
				EntityProperties{
					name:"dragon".to_string(), 
					drawable:DrawEntity::Image(i), 
					params: DrawParam { ..Default::default() }
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

			    entity.properties.params.dest = graphics::Point2::new(p.x, p.y);
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