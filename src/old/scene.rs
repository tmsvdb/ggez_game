use ggez::graphics::{DrawParam, Rect, Point2};
use ggez::graphics;
use ggez::Context;

use ecs::{ECS, Entity, EntityProperties};
use components::Components;
use components::{Navigation, Selection, Sprite, Spritesheet, SpriteAnimation, DebugStats};

use assets::Assets;

extern crate rand;
use rand::Rng;

pub struct StartScene;

impl StartScene {
	pub fn setup (ctx: &mut Context, ecs: &mut ECS) {

		// create entities
		/*
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
		*/

		let spritesheed_image = graphics::Image::new(ctx, "/rifle_man_red_alert_trans.png").expect("Scene: load /rifle_man_red_alert_trans.png failed!");
		let spritesheed_id = ecs.get_assets().add_image(spritesheed_image);	

		//let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18).expect("Scene: load /DejaVuSerif.ttf failed!");
		//let font_id = ecs.get_assets().add_font(font);	

		/*
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
		*/

		/*

		let idle_animation = SpriteAnimation {
			tag:"idle".to_string(),
			time: 0.0,
			fps:4.0,
			frames: StartScene::normalize_animation_frames(
				Point2::new(312.0,149.0),
				vec![
					Rect::new(75.0, 14.0, 16.0, 16.0)
				]
			),
		};

		let run_animation = SpriteAnimation {

			tag:"run".to_string(),
			time: 0.0,
			fps:4.0,
			frames: StartScene::normalize_animation_frames(
				Point2::new(312.0,149.0),
				vec![
					Rect::new(69.0, 55.0, 16.0, 16.0),
					Rect::new(87.0, 55.0, 16.0, 16.0),
					Rect::new(102.0, 55.0, 16.0, 16.0),
					Rect::new(117.0, 55.0, 16.0, 16.0)
				]
			),
		};*/

		for i in 0..10
		{
			let go = GameObject;

			/*
			let xpos = rand::thread_rng().gen_range(0, 900);
			let ypos = rand::thread_rng().gen_range(0, 650);

			let idle_animation = SpriteAnimation {
				tag:"idle".to_string(),
				time: 0.0,
				fps:4.0,
				frames: StartScene::normalize_animation_frames(
					Point2::new(312.0,149.0),
					vec![
						Rect::new(75.0, 14.0, 16.0, 16.0)
					]
				),
			};

			let run_animation = SpriteAnimation {

				tag:"run".to_string(),
				time: 0.0,
				fps:4.0,
				frames: StartScene::normalize_animation_frames(
					Point2::new(312.0,149.0),
					vec![
						Rect::new(69.0, 55.0, 16.0, 16.0),
						Rect::new(87.0, 55.0, 16.0, 16.0),
						Rect::new(102.0, 55.0, 16.0, 16.0),
						Rect::new(117.0, 55.0, 16.0, 16.0)
					]
				),
			};
			ecs.register_entity(
				Entity::new(
					EntityProperties{
						name:format!("unit {:?}", i)
					}, 
					Components { 
						navigation: Some(Navigation { 
								location: Point2::new(xpos as f32,ypos as f32),
								arrived: true, 
								goto: Point2::new (0.0,0.0)
							}),
						selection: Some(Selection { 
								sprite: Sprite {
									visible: false,
									image: spritesheed_id,
									transform: Rect::new(0.0,0.0,16.0,16.0),
									frame: StartScene::normalize_sprite_frame (
										Point2::new(312.0,149.0), 
										Rect::new(133.0, 55.0, 16.0, 16.0) 
									),
								},
								selected: true 
							}),
						spritesheet: Some(Spritesheet { 
							sprite: Sprite {
									visible: false,
									image: spritesheed_id,
									transform: Rect::new(0.0,0.0,16.0,16.0),
									frame: StartScene::normalize_sprite_frame (
										Point2::new(312.0,149.0), 
										Rect::new(75.0, 14.0, 16.0, 16.0) 
									),
								},
							animations: vec![idle_animation, run_animation],
							playing_animation: 0,
						}),
						..Default::default()
					}
				)
			);
			*/
		}

		let font = graphics::Font::default_font().expect("default font not working!");
		
		ecs.register_entity(
			Entity::new(
				EntityProperties{
					name:"spritesheet".to_string()
				}, 
				Components { 
					debug: Some(DebugStats{font: font, fps_str: "first frame".to_string()}),
					..Default::default()
				}
			)
		);
		
	}

	fn normalize_animation_frames (image_size: Point2, mut frames: Vec<Rect>) -> Vec<Rect> {
		let fx = 1.0 / image_size.x;
		let fy = 1.0 / image_size.y;

		for i in 0..frames.len() {
			frames[i].x *= fx; 
			frames[i].y *= fy; 
			frames[i].w *= fx; 
			frames[i].h *= fy; 
		}
		frames
	}

	fn normalize_sprite_frame (image_size: Point2, mut frame: Rect) -> Rect {
		let fx = 1.0 / image_size.x;
		let fy = 1.0 / image_size.y;

		frame.x *= fx; 
		frame.y *= fy; 
		frame.w *= fx; 
		frame.h *= fy; 

		frame
	}
} 