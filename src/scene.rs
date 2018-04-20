use ggez::graphics::{DrawParam, Rect, Point2};
use ggez::graphics;
use ggez::Context;

use ecs::{ECS, Entity, EntityProperties};
use components::Components;
use components::{Position, Velocity, Graphics, DrawEntity, Spritesheet, SpriteAnimation};

pub struct StartScene;

impl StartScene {
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


		let spritesheet = graphics::Image::new(ctx, "/rifle_man_red_alert.png").unwrap();


		let w = 0.0512820512820513;
		let h = 0.1073825503355705;
		let y = 0.3624161073825503;
		let x1 = 0.2211538461538462;
		let x2 = 0.2820512820512821;
		let x3 = 0.3301282051282051;
		let x4 = 0.3782051282051282;

		let animation = SpriteAnimation {
			time: 0.0,
			fps:4.0,
			frames: vec![
				Rect::new(x1, y, w, h),
				Rect::new(x2, y, w, h),
				Rect::new(x3, y, w, h),
				Rect::new(x4, y, w, h)
			],
		};

		ecs.register_entity(
			Entity::new(
				EntityProperties{
					name:"spritesheet".to_string()
				}, 
				Components { 
					graphics: Some(Graphics {draw:DrawEntity::Image(spritesheet), transform: DrawParam { scale: Point2::new(5.0,5.0), ..Default::default() }}),
					spritesheet: Some(Spritesheet { 
						animations: vec![animation],
						playing_animation: 0,
					}),
					..Default::default()
				}
			)
		);

		/*
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
		*/
	}
} 