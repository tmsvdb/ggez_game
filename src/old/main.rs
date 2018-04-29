//! The simplest possible example that does something.

extern crate ggez;
extern crate rand;

use std::env;
use std::path;

use ggez::conf;
use ggez::event;
use ggez::ContextBuilder;

use ecs::ECS;
use systems::{MoveSystem, SpriteSystem, SelectSystem, DebugSystem};

use assets::Assets;

mod ecs;
mod components;
mod systems;
mod assets;
mod scene;

/// MAIN

/// test git
pub fn main() {

    let cb = ContextBuilder::new("ecs test", "thomas")
        .window_setup(conf::WindowSetup::default().title("ECS TEST"))
        .window_mode(conf::WindowMode::default().dimensions(939, 678));
        //.add_resource_path(std::path::PathBuf::from("/assets"));
    let ctx = &mut cb.build().unwrap();

    // create new ecs engine
    let assets = Assets::new();
    let ecs: &mut ECS = &mut ecs::ECS::new(ctx, assets).unwrap();

    // direct ctx asset path to asset folder
	if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        ctx.filesystem.mount(&path, true);
    }
    else {
        println!("Warning: coudn't mount path 'assets' to ctx filesystem");
    }

    
    
    /*
    // create and register systems
    ecs.register_for_update(MoveSystem);
    ecs.register_for_draw(SpriteSystem);
    ecs.register_for_update(SpriteSystem);
    ecs.register_for_mouse(MoveSystem);
    ecs.register_for_draw(SelectSystem);
    ecs.register_for_mouse(SelectSystem);
    ecs.register_for_update(DebugSystem);
    ecs.register_for_draw(DebugSystem);
	*/
	scene::StartScene::setup(ctx, ecs);

	// do gameloop and use ecs engine as ggez event handler
    if let Err(e) = event::run(ctx, ecs) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}


// A GAMEOBJECT

struct GameObject {
	//somesystem: SomeSystem,
}

impl GameObject {
	fn build (ecs: &mut ECS) {
		let go = GameObject {};
		ecs.register_entity(go);
	}
}

impl event::EventHandler for GameObject {
	fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
		Ok(())
	}

	fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
		Ok(())
    }
}


// SOMESYSTEM

struct SomeSystemValues {
	name: String
}

struct SomeSystem;

impl SomeSystem {
	fn do_something (&self, change_this: &mut SomeSystemValues) {

	}
}
