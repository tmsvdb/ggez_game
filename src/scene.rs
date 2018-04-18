use ggez::graphics::{DrawParam, Rect, Point2};
use ggez::graphics;
use ggez::Context;

use ecs::{ECS, Entity, EntityProperties};
use components::Components;
use components::{Position, Velocity, Graphics, DrawEntity, Spritesheet};

pub struct Scene;

impl Scene {
	pub fn setup (ctx: &mut Context, ecs: &mut ECS) {

		// create entities
		let dragon = graphics::Image::new(ctx, "/dragon1.png").unwrap();

		ecs.register_entity(
			Entity::new(
				EntityProperties{
					name:"dragon".to_string()
				}, 
				Components { 
					position: Some(Position { x:-939.0, y:0.0 }), 
					velocity: Some(Velocity { x:3.0, y:0.0 }),
					graphics: Some(Graphics {draw:DrawEntity::Image(dragon), transform: DrawParam { ..Default::default() }}),
					..Default::default()
				}
			)
		);


		let spritesheet = graphics::Image::new(ctx, "/WalkingManSpriteSheet.png").unwrap();

		ecs.register_entity(
			Entity::new(
				EntityProperties{
					name:"spritesheet".to_string()
				}, 
				Components { 
					graphics: Some(Graphics {draw:DrawEntity::Image(spritesheet), transform: DrawParam { ..Default::default() }}),
					spritesheet: Some(Spritesheet { 
						animation_time: 0.0,
						fps: 12.0,
						image_size: Point2::new(1024.0, 157.0),
						frame: Rect{x:0.0,y:0.0,w:128.0, h:157.0},
						index: 0,
						number_of_sprites: 8,
					}),
					..Default::default()
				}
			)
		);

	}
} 